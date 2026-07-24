#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_register_and_verify() {
    let env = Env::default();
    env.mock_all_auths();

    // The method to register changed in recent SDK versions, trying `env.register`
    let contract_id = env.register(RegistryContract, ());
    let client = RegistryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let verifier = Address::generate(&env);

    client.initialize(&admin);
    client.add_verifier(&verifier);

    let name = String::from_str(&env, "Amazon Reforestation");
    let location = String::from_str(&env, "Brazil");
    let methodology = String::from_str(&env, "VM0015");

    let project_id = client.register_project(
        &name,
        &location,
        &ProjectType::Forestry,
        &verifier,
        &methodology,
        &10_000,
    );

    assert_eq!(project_id, 1);
    
    let is_verified = client.is_project_verified(&project_id);
    assert_eq!(is_verified, false);

    client.verify_project(&project_id);

    let is_verified_now = client.is_project_verified(&project_id);
    assert_eq!(is_verified_now, true);
    
    let project = client.get_project(&project_id);
    assert_eq!(project.verified, true);
}
