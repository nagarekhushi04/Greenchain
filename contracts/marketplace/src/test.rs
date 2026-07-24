#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

#[test]
fn test_marketplace_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    // Create a mock token admin
    let token_admin = Address::generate(&env);

    // Register builtin token contracts for testing
    let asset_token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let payment_token_id = env.register_stellar_asset_contract_v2(token_admin.clone());

    let asset_client = token::StellarAssetClient::new(&env, &asset_token_id.address());
    let payment_client = token::StellarAssetClient::new(&env, &payment_token_id.address());

    let token_client_asset = token::Client::new(&env, &asset_token_id.address());
    let token_client_payment = token::Client::new(&env, &payment_token_id.address());

    // Register marketplace
    let marketplace_id = env.register(MarketplaceContract, ());
    let client = MarketplaceContractClient::new(&env, &marketplace_id);

    let admin = Address::generate(&env);
    let fee_address = Address::generate(&env);
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);

    client.initialize(&admin, &fee_address);

    // Mint some assets to seller and payment to buyer
    asset_client.mint(&seller, &1000);
    payment_client.mint(&buyer, &5000);

    // Seller lists 100 assets for 10 payment tokens each (total 1000)
    let listing_id = client.list_token(
        &seller,
        &asset_token_id.address(),
        &payment_token_id.address(),
        &100,
        &10,
    );

    assert_eq!(listing_id, 1);
    
    // Check escrow balance
    assert_eq!(token_client_asset.balance(&marketplace_id), 100);
    assert_eq!(token_client_asset.balance(&seller), 900);

    // Buyer buys 50 tokens (total 500 payment tokens)
    client.buy_tokens(&buyer, &listing_id, &50);

    // Check balances
    assert_eq!(token_client_asset.balance(&buyer), 50);
    assert_eq!(token_client_asset.balance(&marketplace_id), 50);

    // 500 payment tokens spent: 5% fee = 25. Seller gets 475.
    assert_eq!(token_client_payment.balance(&fee_address), 25);
    assert_eq!(token_client_payment.balance(&seller), 475);
    assert_eq!(token_client_payment.balance(&buyer), 4500);

    // Seller cancels listing
    client.cancel_listing(&seller, &listing_id);
    
    // Remaining 50 returned to seller
    assert_eq!(token_client_asset.balance(&seller), 950);
    assert_eq!(token_client_asset.balance(&marketplace_id), 0);
}
