use soroban_sdk::{Address, Env};
use crate::types::{DataKey, Project};

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn set_verifier(env: &Env, verifier: &Address, status: bool) {
    env.storage().persistent().set(&DataKey::Verifier(verifier.clone()), &status);
}

pub fn is_verifier(env: &Env, verifier: &Address) -> bool {
    env.storage().persistent().get(&DataKey::Verifier(verifier.clone())).unwrap_or(false)
}

pub fn get_next_project_id(env: &Env) -> u64 {
    let count: u64 = env.storage().instance().get(&DataKey::ProjectCount).unwrap_or(0);
    env.storage().instance().set(&DataKey::ProjectCount, &(count + 1));
    count + 1
}

pub fn set_project(env: &Env, id: u64, project: &Project) {
    env.storage().persistent().set(&DataKey::Project(id), project);
}

pub fn get_project(env: &Env, id: u64) -> Option<Project> {
    env.storage().persistent().get(&DataKey::Project(id))
}
