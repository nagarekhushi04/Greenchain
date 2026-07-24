# GreenChain 🌿

A production-grade decentralized marketplace on Stellar (Soroban smart contracts) for tokenizing and trading verified environmental assets (carbon credits, RECs, forestry credits).

## Architecture

![Architecture](https://via.placeholder.com/800x400?text=GreenChain+Architecture)

GreenChain consists of three main components:
1. **Soroban Smart Contracts**: A multi-contract architecture for secure asset tokenization.
   - `RegistryContract`: Manages verified environmental projects and authorized verifiers.
   - `TokenContract`: A SEP-41 compatible token representing specific project vintages. Uses cross-contract calls to the Registry to ensure only verified projects are tokenized. Supports burning/retirement.
   - `MarketplaceContract`: Facilitates trustless escrow, listings, and fee-split purchasing.
2. **Event Stream Indexer**: A Node.js backend that polls Soroban RPC for contract events, indexing them into a Postgres database for real-time frontend querying.
3. **Frontend**: A React + TypeScript web app for browsing the marketplace, managing portfolios, and retiring credits.

## Setup Instructions

### 1. Smart Contracts
```bash
# Install Soroban CLI
cargo install --locked soroban-cli --features opt

# Build and optimize contracts
cargo test --workspace
soroban contract build
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/registry.wasm
# Repeat for token and marketplace
```

### 2. Indexer
Requires Postgres.
```bash
cd indexer
npm install
# Set DATABASE_URL and RPC_URL in .env
npm run dev
```

### 3. Frontend
```bash
cd frontend
npm install
npm run dev
```

## Environment Variables
Create `.env` files in both `indexer` and `frontend`:
- `RPC_URL`: Soroban RPC endpoint (e.g. `https://soroban-testnet.stellar.org:443`)
- `NETWORK_PASSPHRASE`: Testnet passphrase
- `REGISTRY_CONTRACT_ID`: Deployed address of Registry
- `TOKEN_CONTRACT_ID`: Deployed address of Token
- `MARKETPLACE_CONTRACT_ID`: Deployed address of Marketplace
- `DATABASE_URL`: (Indexer only) Postgres connection string

## Deployment Strategy
To deploy manually:
1. Use `soroban contract deploy` to deploy the optimized wasm files.
2. Call the `initialize` function on each deployed contract using `soroban contract invoke`.
3. Save the resulting contract addresses to `.env`.
