use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenMetadata {
    pub project_id: u64,
    pub vintage_year: u32,
    pub serial_range_start: i128,
    pub serial_range_end: i128,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Registry,
    Metadata,
    Balance(Address),
    Retired(Address),
    TotalSupply,
    TotalRetired,
}
