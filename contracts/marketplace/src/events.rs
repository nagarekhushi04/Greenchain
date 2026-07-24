use soroban_sdk::{symbol_short, Address, Env};

pub fn emit_listed(env: &Env, id: u64, seller: &Address, token: &Address, amount: i128, price: i128) {
    let topics = (symbol_short!("listing"), symbol_short!("create"), id);
    env.events().publish(topics, (seller.clone(), token.clone(), amount, price));
}

pub fn emit_purchased(env: &Env, id: u64, buyer: &Address, amount: i128) {
    let topics = (symbol_short!("listing"), symbol_short!("buy"), id);
    env.events().publish(topics, (buyer.clone(), amount));
}

pub fn emit_cancelled(env: &Env, id: u64) {
    let topics = (symbol_short!("listing"), symbol_short!("cancel"), id);
    env.events().publish(topics, ());
}
