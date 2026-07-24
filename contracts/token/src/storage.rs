use soroban_sdk::{Address, Env};
use crate::types::{DataKey, TokenMetadata};

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn set_registry(env: &Env, registry: &Address) {
    env.storage().instance().set(&DataKey::Registry, registry);
}

pub fn get_registry(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Registry).unwrap()
}

pub fn set_metadata(env: &Env, metadata: &TokenMetadata) {
    env.storage().instance().set(&DataKey::Metadata, metadata);
}

pub fn get_metadata(env: &Env) -> TokenMetadata {
    env.storage().instance().get(&DataKey::Metadata).unwrap()
}

pub fn get_balance(env: &Env, address: &Address) -> i128 {
    env.storage().persistent().get(&DataKey::Balance(address.clone())).unwrap_or(0)
}

pub fn set_balance(env: &Env, address: &Address, balance: i128) {
    env.storage().persistent().set(&DataKey::Balance(address.clone()), &balance);
}

pub fn get_retired(env: &Env, address: &Address) -> i128 {
    env.storage().persistent().get(&DataKey::Retired(address.clone())).unwrap_or(0)
}

pub fn set_retired(env: &Env, address: &Address, retired: i128) {
    env.storage().persistent().set(&DataKey::Retired(address.clone()), &retired);
}

pub fn get_total_supply(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
}

pub fn set_total_supply(env: &Env, supply: i128) {
    env.storage().instance().set(&DataKey::TotalSupply, &supply);
}

pub fn get_total_retired(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::TotalRetired).unwrap_or(0)
}

pub fn set_total_retired(env: &Env, retired: i128) {
    env.storage().instance().set(&DataKey::TotalRetired, &retired);
}
