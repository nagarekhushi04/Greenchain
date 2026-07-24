use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Listing {
    pub id: u64,
    pub seller: Address,
    pub asset_token: Address,     // Green token
    pub payment_token: Address,   // Payment currency
    pub amount: i128,
    pub price_per_token: i128,
    pub active: bool,
}

#[contracttype]
pub enum DataKey {
    Admin,
    FeeAddress,        // Address that collects platform fees
    ListingCount,
    Listing(u64),
}
