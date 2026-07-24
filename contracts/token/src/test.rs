#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, contract, contractimpl};

#[contract]
pub struct MockRegistry;

#[contractimpl]
impl MockRegistry {
    pub fn is_project_verified(_env: Env, project_id: u64) -> bool {
        project_id == 1
    }
}

#[test]
fn test_token_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let registry_id = env.register(MockRegistry, ());
    
    let token_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &token_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    client.initialize(
        &admin,
        &registry_id,
        &1, // project_id 1 is verified
        &2026,
        &1,
        &1000,
    );

    client.mint(&user1, &500);
    assert_eq!(client.balance(&user1), 500);

    client.transfer(&user1, &user2, &200);
    assert_eq!(client.balance(&user1), 300);
    assert_eq!(client.balance(&user2), 200);

    client.retire(&user2, &50);
    assert_eq!(client.balance(&user2), 150);
    assert_eq!(client.retired_balance(&user2), 50);
}

#[test]
#[should_panic(expected = "project is not verified")]
fn test_mint_unverified_project() {
    let env = Env::default();
    env.mock_all_auths();

    let registry_id = env.register(MockRegistry, ());
    let token_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &token_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(
        &admin,
        &registry_id,
        &2, // project_id 2 is not verified
        &2026,
        &1,
        &1000,
    );

    client.mint(&user, &500); // Should panic
}
