use soroban_sdk::{symbol_short, Address, Env};

pub fn emit_mint(env: &Env, to: &Address, amount: i128) {
    let topics = (symbol_short!("token"), symbol_short!("mint"), to.clone());
    env.events().publish(topics, amount);
}

pub fn emit_transfer(env: &Env, from: &Address, to: &Address, amount: i128) {
    let topics = (symbol_short!("token"), symbol_short!("transfer"), from.clone(), to.clone());
    env.events().publish(topics, amount);
}

pub fn emit_retire(env: &Env, from: &Address, amount: i128) {
    let topics = (symbol_short!("token"), symbol_short!("retire"), from.clone());
    env.events().publish(topics, amount);
}
