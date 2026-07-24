process.env.NODE_ENV = 'test';
import assert from 'node:assert';
import http from 'node:http';
import { AddressInfo } from 'node:net';
import { app, initDB } from './index';

function makeRequest(url: string): Promise<{ statusCode?: number; data: any }> {
  return new Promise((resolve, reject) => {
    http.get(url, (res) => {
      let body = '';
      res.on('data', chunk => body += chunk);
      res.on('end', () => {
        try {
          resolve({ statusCode: res.statusCode, data: JSON.parse(body) });
        } catch (e) {
          resolve({ statusCode: res.statusCode, data: body });
        }
      });
    }).on('error', reject);
  });
}

async function runTests() {
  console.log("Running indexer API tests...");
  await initDB();

  const server = app.listen(0, async () => {
    const port = (server.address() as AddressInfo).port;
    const baseUrl = `http://127.0.0.1:${port}`;

    try {
      const projectsRes = await makeRequest(`${baseUrl}/api/projects`);
      assert.strictEqual(projectsRes.statusCode, 200, "GET /api/projects should return 200");
      assert(Array.isArray(projectsRes.data.projects), "projects should be an array");
      console.log("✔ GET /api/projects test passed!");

      const listingsRes = await makeRequest(`${baseUrl}/api/listings`);
      assert.strictEqual(listingsRes.statusCode, 200, "GET /api/listings should return 200");
      assert(Array.isArray(listingsRes.data.listings), "listings should be an array");
      console.log("✔ GET /api/listings test passed!");

      console.log("All indexer API tests passed successfully!");
    } catch (err) {
      console.error("❌ Indexer API test failed:", err);
      process.exitCode = 1;
    } finally {
      server.close();
    }
  });
}

runTests();

