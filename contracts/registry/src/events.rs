use soroban_sdk::{symbol_short, Address, Env};

pub fn emit_project_registered(env: &Env, id: u64, verifier: &Address) {
    let topics = (symbol_short!("project"), symbol_short!("register"), id);
    env.events().publish(topics, verifier.clone());
}

pub fn emit_project_verified(env: &Env, id: u64, verifier: &Address) {
    let topics = (symbol_short!("project"), symbol_short!("verify"), id);
    env.events().publish(topics, verifier.clone());
}
