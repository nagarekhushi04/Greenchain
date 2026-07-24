use soroban_sdk::{Address, Env};
use crate::types::{DataKey, Listing};

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn set_fee_address(env: &Env, fee_address: &Address) {
    env.storage().instance().set(&DataKey::FeeAddress, fee_address);
}

pub fn get_fee_address(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::FeeAddress).unwrap()
}

pub fn get_next_listing_id(env: &Env) -> u64 {
    let count: u64 = env.storage().instance().get(&DataKey::ListingCount).unwrap_or(0);
    env.storage().instance().set(&DataKey::ListingCount, &(count + 1));
    count + 1
}

pub fn set_listing(env: &Env, id: u64, listing: &Listing) {
    env.storage().persistent().set(&DataKey::Listing(id), listing);
}

pub fn get_listing(env: &Env, id: u64) -> Option<Listing> {
    env.storage().persistent().get(&DataKey::Listing(id))
}
