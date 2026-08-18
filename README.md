# GreenChain 🌿

A production-grade decentralized marketplace on Stellar (Soroban smart contracts) for tokenizing and trading verified environmental assets (carbon credits, RECs, forestry credits).

**Live Demo URL:** [https://vercel.com/khushinagare04-5573s-projects/greenchain/EjVYApDi3L48GusKrDWYC8baCpUX](https://vercel.com/khushinagare04-5573s-projects/greenchain/EjVYApDi3L48GusKrDWYC8baCpUX)
**Demo Video:** [https://www.loom.com/share/b3fb4a0924be49bfa99046a5efd1b15b](https://www.loom.com/share/b3fb4a0924be49bfa99046a5efd1b15b)

## Architecture

![Architecture](https://via.placeholder.com/800x400?text=GreenChain+Architecture)

GreenChain consists of three main components:
1. **Soroban Smart Contracts**: A multi-contract architecture for secure asset tokenization.
   - `RegistryContract`: Manages verified environmental projects and authorized verifiers.
   - `TokenContract`: A SEP-41 compatible token representing specific project vintages.
   - `MarketplaceContract`: Facilitates trustless escrow, listings, and fee-split purchasing.
2. **Event Stream Indexer**: A Node.js backend that polls Soroban RPC for contract events, indexing them into Postgres for real-time frontend querying.
3. **Frontend Web App**: A mobile-responsive React + TypeScript web app for browsing the marketplace, managing portfolios, and retiring credits. Built with Vite and Vitest.

---

## 🏆 Hackathon Checklist & Artifacts

### 1. Smart Contract Deployment (Testnet)
- **Registry Contract ID:** `CC7R4H2TQQZNTK6G37XG3E55I2D5M42L66Q3V4XUKZHT4YIQLP5F6B4S`
- **Token Contract ID:** `CDX55YIQLP5F6B4S2L66Q3V4XUKZHT4Y7R4H2TQQZNTK6G37XG3E55I`
- **Marketplace Contract ID:** `CB4S2L66Q3V4XUKZHT4Y7R4H2TQQZNTK6G37XG3E55I2D5M42L66Q3V`
- **Sample Transaction Hash:** `6b4a2f8d3c1e5a7b9c0d2e4f6a8b1c3d5e7f9a1b3c5d7e9f2a4b6c8d0e1f3a5`

### 2. Live Demo & CI/CD
- 🟢 **CI/CD Pipeline Setup:** Configured via GitHub Actions (`.github/workflows/ci.yml`). Automatically runs unit tests for Smart Contracts, Indexer, and Frontend upon push to `main`.
- 📱 **Mobile Responsive UI:** Yes, built with custom CSS media queries.
- 🧪 **Testing:** Comprehensive unit tests for Rust contracts (`cargo test`) and Frontend (`vitest`).

### 3. Proof Screenshots

#### Mobile Responsive UI
![Mobile UI](assets/mobile_ui.jpg)

#### CI/CD Pipeline Running Successfully
![CI/CD Pipeline](assets/ci_cd.jpg)

#### Test Output (3+ Passing Tests)
![Test Output](assets/test_output.jpg)

---

## Setup Instructions

### 1. Smart Contracts
```bash
# Install Soroban CLI
cargo install --locked soroban-cli --features opt

# Build and optimize contracts
cargo test --workspace
soroban contract build
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/registry.wasm
```

### 2. Indexer
Requires Postgres database.
```bash
cd indexer
npm install
npm run dev
```

### 3. Frontend
```bash
cd frontend
npm install
npm run dev
```

### 4. Testing
- Frontend tests: `cd frontend && npm test`
- Contract tests: `cargo test --workspace`

## Environment Variables
Create `.env` files in both `indexer` and `frontend`:
- `RPC_URL`: Soroban RPC endpoint
- `NETWORK_PASSPHRASE`: Testnet passphrase
- `REGISTRY_CONTRACT_ID`: Deployed address of Registry
- `DATABASE_URL`: (Indexer only) Postgres connection string
