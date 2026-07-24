#![no_std]

mod events;
mod storage;
mod types;
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env, String};
use types::{Project, ProjectType};

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    /// Initialize the registry with an admin address
    pub fn initialize(env: Env, admin: Address) {
        if storage::has_admin(&env) {
            panic!("already initialized");
        }
        storage::set_admin(&env, &admin);
    }

    /// Add a verifier to the whitelist. Requires Admin auth.
    pub fn add_verifier(env: Env, verifier: Address) {
        let admin = storage::get_admin(&env);
        admin.require_auth();
        storage::set_verifier(&env, &verifier, true);
    }

    /// Remove a verifier from the whitelist. Requires Admin auth.
    pub fn remove_verifier(env: Env, verifier: Address) {
        let admin = storage::get_admin(&env);
        admin.require_auth();
        storage::set_verifier(&env, &verifier, false);
    }

    /// Register a new project. 
    /// Returns the new project_id. Initialized with verified = false.
    pub fn register_project(
        env: Env,
        name: String,
        location: String,
        project_type: ProjectType,
        verifier: Address,
        methodology: String,
        total_capacity: i128,
    ) -> u64 {
        if total_capacity <= 0 {
            panic!("total capacity must be positive");
        }

        let id = storage::get_next_project_id(&env);
        
        let project = Project {
            id,
            name,
            location,
            project_type,
            verifier: verifier.clone(),
            methodology,
            total_capacity,
            verified: false,
        };

        storage::set_project(&env, id, &project);
        events::emit_project_registered(&env, id, &verifier);
        
        id
    }

    /// Verify a project. Requires auth from the project's assigned `verifier`.
    pub fn verify_project(env: Env, project_id: u64) {
        let mut project = storage::get_project(&env, project_id).expect("project not found");
        
        if !storage::is_verifier(&env, &project.verifier) {
            panic!("assigned verifier is not whitelisted");
        }
        
        project.verifier.require_auth();
        
        if project.verified {
            panic!("project already verified");
        }

        project.verified = true;
        storage::set_project(&env, project_id, &project);
        
        events::emit_project_verified(&env, project_id, &project.verifier);
    }

    /// Get project details.
    pub fn get_project(env: Env, project_id: u64) -> Project {
        storage::get_project(&env, project_id).expect("project not found")
    }

    /// Read-only check if a project is verified. Used by TokenContract.
    pub fn is_project_verified(env: Env, project_id: u64) -> bool {
        let project = storage::get_project(&env, project_id).expect("project not found");
        project.verified
    }
}
