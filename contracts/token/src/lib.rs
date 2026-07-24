#![no_std]

mod events;
mod storage;
mod types;
mod test;

use soroban_sdk::{contract, contractimpl, contractclient, Address, Env};
use types::TokenMetadata;

#[contractclient(name = "RegistryClient")]
pub trait RegistryTrait {
    fn is_project_verified(env: Env, project_id: u64) -> bool;
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(
        env: Env, 
        admin: Address, 
        registry: Address, 
        project_id: u64,
        vintage_year: u32,
        serial_range_start: i128,
        serial_range_end: i128
    ) {
        if storage::has_admin(&env) {
            panic!("already initialized");
        }
        storage::set_admin(&env, &admin);
        storage::set_registry(&env, &registry);
        
        let metadata = TokenMetadata {
            project_id,
            vintage_year,
            serial_range_start,
            serial_range_end,
        };
        storage::set_metadata(&env, &metadata);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        if amount <= 0 {
            panic!("amount must be positive");
        }

        let admin = storage::get_admin(&env);
        admin.require_auth();

        let registry = storage::get_registry(&env);
        let metadata = storage::get_metadata(&env);

        let registry_client = RegistryClient::new(&env, &registry);
        let is_verified = registry_client.is_project_verified(&metadata.project_id);
        
        if !is_verified {
            panic!("project is not verified");
        }

        let balance = storage::get_balance(&env, &to);
        storage::set_balance(&env, &to, balance.checked_add(amount).expect("overflow"));

        let supply = storage::get_total_supply(&env);
        storage::set_total_supply(&env, supply.checked_add(amount).expect("overflow"));

        events::emit_mint(&env, &to, amount);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        if amount <= 0 {
            panic!("amount must be positive");
        }
        from.require_auth();

        let from_balance = storage::get_balance(&env, &from);
        if from_balance < amount {
            panic!("insufficient balance");
        }

        storage::set_balance(&env, &from, from_balance - amount);
        
        let to_balance = storage::get_balance(&env, &to);
        storage::set_balance(&env, &to, to_balance.checked_add(amount).expect("overflow"));

        events::emit_transfer(&env, &from, &to, amount);
    }

    pub fn retire(env: Env, from: Address, amount: i128) {
        if amount <= 0 {
            panic!("amount must be positive");
        }
        from.require_auth();

        let balance = storage::get_balance(&env, &from);
        if balance < amount {
            panic!("insufficient balance");
        }

        storage::set_balance(&env, &from, balance - amount);
        
        let retired = storage::get_retired(&env, &from);
        storage::set_retired(&env, &from, retired.checked_add(amount).expect("overflow"));
        
        let total_retired = storage::get_total_retired(&env);
        storage::set_total_retired(&env, total_retired.checked_add(amount).expect("overflow"));

        events::emit_retire(&env, &from, amount);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        storage::get_balance(&env, &id)
    }

    pub fn retired_balance(env: Env, id: Address) -> i128 {
        storage::get_retired(&env, &id)
    }
}
