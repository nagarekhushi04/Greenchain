# GreenChain Demo Script

This script outlines how to demonstrate the GreenChain marketplace from start to finish. It covers registering an environmental project, verifying it, minting tokens, listing them on the marketplace, purchasing them as a buyer, and retiring them for an environmental offset.

## Prerequisites
1. Contracts are deployed to Stellar Testnet (Registry, Token, Marketplace).
2. The indexer is running locally and connected to the Testnet RPC.
3. The React frontend is running locally.
4. Freighter wallet is installed with at least two accounts (Admin/Verifier and Buyer).

## Step 1: Register and Verify a Project
**Role:** Admin / Verifier
1. Open the frontend and connect your Freighter wallet using the **Admin** account.
2. Navigate to the **Verifier Dashboard**.
3. Click **Register New Project**.
   - **Name:** Amazon Reforestation Initiative
   - **Location:** Brazil
   - **Project Type:** Forestry
   - **Methodology:** VM0015
   - **Total Capacity:** 50,000
4. Submit the transaction. A new project is created with `verified = false`.
5. In the dashboard, locate the newly created project and click **Verify**.
6. Sign the transaction. The project is now verified and eligible for tokenization.

## Step 2: Mint Green Tokens
**Role:** Admin
1. Navigate to the **Token Management** section.
2. Select the verified project from Step 1.
3. Set the **Vintage Year** (e.g., 2026) and **Amount** to mint (e.g., 10,000).
4. Click **Mint Tokens**.
5. *Under the hood:* The Token contract makes a cross-contract call to the Registry to ensure the project is verified. Once confirmed, it mints 10,000 SEP-41 compliant tokens to your address.

## Step 3: List Tokens on the Marketplace
**Role:** Admin (Seller)
1. Go to **My Portfolio** to view your token balance.
2. Select the green tokens you just minted and click **List for Sale**.
3. Set the parameters:
   - **Amount to Sell:** 5,000
   - **Price per Token:** 10 USDC (or equivalent payment token on testnet)
4. Confirm the listing.
5. *Under the hood:* The tokens are transferred to the Marketplace contract's escrow balance. An event is emitted and picked up by the indexer.

## Step 4: Purchase Tokens
**Role:** Buyer
1. Switch your Freighter wallet to the **Buyer** account.
2. Refresh the frontend and navigate to the **Marketplace** page.
3. You will see the new "Amazon Reforestation" listing. Click **Buy**.
4. Enter the amount to purchase: 1,000.
5. Approve the transaction in Freighter.
6. *Under the hood:* 10,000 USDC is transferred from the buyer to the seller (minus platform fees), and 1,000 Green Tokens are transferred from escrow to the buyer.

## Step 5: Retire Tokens (Claim Offset)
**Role:** Buyer
1. Navigate to **My Portfolio**. You will see 1,000 Green Tokens.
2. Click **Retire Tokens**.
3. Enter the amount to retire: 500.
4. Confirm the transaction.
5. The tokens are burned from your active balance and permanently recorded in your "Retired Balance".
6. Navigate to the **Retirement Certificates** tab to view your verifiable on-chain offset certificate.

---
**End of Demo**
