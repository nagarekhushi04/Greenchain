use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectType {
    Carbon,
    REC,
    Forestry,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub location: String,
    pub project_type: ProjectType,
    pub verifier: Address,
    pub methodology: String,
    pub total_capacity: i128,
    pub verified: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    ProjectCount,
    Project(u64),
    Verifier(Address),
}
