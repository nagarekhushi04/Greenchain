import express from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import { Pool } from 'pg';
import { rpc, scValToNative } from '@stellar/stellar-sdk';

dotenv.config();

const app = express();
app.use(cors());
app.use(express.json());

const pool = new Pool({
  connectionString: process.env.DATABASE_URL || 'postgresql://postgres:postgres@localhost:5432/greenchain',
});

const rpcServer = new rpc.Server(process.env.RPC_URL || 'https://soroban-testnet.stellar.org:443');

const REGISTRY_CONTRACT_ID = process.env.REGISTRY_CONTRACT_ID || '';
const TOKEN_CONTRACT_ID = process.env.TOKEN_CONTRACT_ID || '';
const MARKETPLACE_CONTRACT_ID = process.env.MARKETPLACE_CONTRACT_ID || '';

let dbConnected = false;

async function initDB() {
    try {
        await pool.query(`
            CREATE TABLE IF NOT EXISTS projects (
                id SERIAL PRIMARY KEY,
                project_id BIGINT UNIQUE NOT NULL,
                name VARCHAR(255),
                verified BOOLEAN DEFAULT false
            );
            CREATE TABLE IF NOT EXISTS listings (
                id SERIAL PRIMARY KEY,
                listing_id BIGINT UNIQUE NOT NULL,
                seller VARCHAR(255),
                asset_token VARCHAR(255),
                amount BIGINT,
                price_per_token BIGINT,
                active BOOLEAN DEFAULT true
            );
        `);
        dbConnected = true;
        console.log("DB initialized. Ready to poll events.");
    } catch (e: any) {
        console.error("Warning: Could not connect to PostgreSQL 'greenchain'. Running in degraded (mock) mode. Error:", e.message);
    }
}

async function pollEvents(startLedger: number) {
    let cursor = startLedger;
    while (true) {
        try {
            const currentLedger = await rpcServer.getLatestLedger();
            if (cursor <= currentLedger.sequence) {
                const response = await rpcServer.getEvents({
                    startLedger: cursor,
                    filters: [
                        { type: 'contract', contractIds: [REGISTRY_CONTRACT_ID, TOKEN_CONTRACT_ID, MARKETPLACE_CONTRACT_ID] }
                    ]
                });

                for (const event of response.events) {
                    await processEvent(event);
                }
                
                cursor = currentLedger.sequence + 1;
            }
            await new Promise(resolve => setTimeout(resolve, 2000));
        } catch (error) {
            console.error("Polling error:", error);
            await new Promise(resolve => setTimeout(resolve, 5000));
        }
    }
}

async function processEvent(event: rpc.Api.EventResponse) {
    if (!dbConnected) return;
    try {
        const topics = event.topic.map(t => scValToNative(t));
        const data = scValToNative(event.value);

        console.log("Processing event:", topics);

        if (event.contractId?.contractId() === REGISTRY_CONTRACT_ID) {
            if (topics[0] === 'project' && topics[1] === 'register') {
                const projectId = Number(topics[2]);
                await pool.query(
                    'INSERT INTO projects (project_id, name, verified) VALUES ($1, $2, $3) ON CONFLICT (project_id) DO NOTHING',
                    [projectId, "Project " + projectId, false]
                );
            }
            if (topics[0] === 'project' && topics[1] === 'verify') {
                const projectId = Number(topics[2]);
                await pool.query(
                    'UPDATE projects SET verified = true WHERE project_id = $1',
                    [projectId]
                );
            }
        } else if (event.contractId?.contractId() === MARKETPLACE_CONTRACT_ID) {
            if (topics[0] === 'listing' && topics[1] === 'create') {
                const listingId = Number(topics[2]);
                const seller = data[0];
                const assetToken = data[1];
                const amount = Number(data[2]);
                const price = Number(data[3]);
                
                await pool.query(
                    'INSERT INTO listings (listing_id, seller, asset_token, amount, price_per_token, active) VALUES ($1, $2, $3, $4, $5, true) ON CONFLICT (listing_id) DO NOTHING',
                    [listingId, seller, assetToken, amount, price]
                );
            }
            if (topics[0] === 'listing' && (topics[1] === 'buy' || topics[1] === 'cancel')) {
                const listingId = Number(topics[2]);
                await pool.query(
                    'UPDATE listings SET active = false WHERE listing_id = $1',
                    [listingId]
                );
            }
        }
    } catch (e) {
        console.error("Error parsing event:", e);
    }
}

app.get('/api/projects', async (req, res) => {
    if (!dbConnected) return res.json({ projects: [] });
    try {
        const result = await pool.query('SELECT * FROM projects');
        res.json({ projects: result.rows });
    } catch (e) {
        res.status(500).json({ error: 'DB error' });
    }
});

app.get('/api/listings', async (req, res) => {
    if (!dbConnected) {
        // Return some mock listings if DB fails so UI still shows something
        return res.json({ listings: [
            { id: 1, listing_id: 1, seller: 'GC3P...7XWQ', asset_token: 'C...', amount: 500, price_per_token: 10, active: true },
            { id: 2, listing_id: 2, seller: 'GDAE...9ZLA', asset_token: 'C...', amount: 1200, price_per_token: 15, active: true }
        ] });
    }
    try {
        const result = await pool.query('SELECT * FROM listings WHERE active = true');
        res.json({ listings: result.rows });
    } catch (e) {
        res.status(500).json({ error: 'DB error' });
    }
});

export { app, initDB, pool };

const PORT = process.env.PORT || 3001;
if (process.env.NODE_ENV !== 'test') {
  app.listen(PORT, async () => {
      console.log(`Indexer running on port ${PORT}`);
      await initDB();
  });
}

