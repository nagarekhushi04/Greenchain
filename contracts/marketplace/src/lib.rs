#![no_std]

mod events;
mod storage;
mod types;
mod test;

use soroban_sdk::{contract, contractimpl, token, Address, Env};
use types::Listing;

#[contract]
pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {
    pub fn initialize(env: Env, admin: Address, fee_address: Address) {
        if storage::has_admin(&env) {
            panic!("already initialized");
        }
        storage::set_admin(&env, &admin);
        storage::set_fee_address(&env, &fee_address);
    }

    /// List tokens for sale. Requires the seller to authorize the marketplace
    /// to pull `amount` of `asset_token` into escrow.
    pub fn list_token(
        env: Env,
        seller: Address,
        asset_token: Address,
        payment_token: Address,
        amount: i128,
        price_per_token: i128,
    ) -> u64 {
        seller.require_auth();

        if amount <= 0 || price_per_token <= 0 {
            panic!("amount and price must be positive");
        }

        let token_client = token::Client::new(&env, &asset_token);
        token_client.transfer(&seller, &env.current_contract_address(), &amount);

        let id = storage::get_next_listing_id(&env);
        let listing = Listing {
            id,
            seller: seller.clone(),
            asset_token: asset_token.clone(),
            payment_token,
            amount,
            price_per_token,
            active: true,
        };

        storage::set_listing(&env, id, &listing);
        events::emit_listed(&env, id, &seller, &asset_token, amount, price_per_token);

        id
    }

    /// Buy tokens from a listing. Requires the buyer to authorize the marketplace
    /// to pull `amount * price_per_token` of `payment_token`.
    pub fn buy_tokens(env: Env, buyer: Address, listing_id: u64, amount: i128) {
        buyer.require_auth();

        if amount <= 0 {
            panic!("amount must be positive");
        }

        let mut listing = storage::get_listing(&env, listing_id).expect("listing not found");
        if !listing.active {
            panic!("listing is not active");
        }
        if listing.amount < amount {
            panic!("insufficient listing amount");
        }

        let total_price = amount.checked_mul(listing.price_per_token).expect("overflow");
        
        // Fee split: e.g. 5% platform fee
        let fee_address = storage::get_fee_address(&env);
        let fee = total_price / 20; // 5%
        let seller_revenue = total_price - fee;

        let payment_client = token::Client::new(&env, &listing.payment_token);
        
        payment_client.transfer(&buyer, &listing.seller, &seller_revenue);
        if fee > 0 {
            payment_client.transfer(&buyer, &fee_address, &fee);
        }

        let asset_client = token::Client::new(&env, &listing.asset_token);
        asset_client.transfer(&env.current_contract_address(), &buyer, &amount);

        listing.amount -= amount;
        if listing.amount == 0 {
            listing.active = false;
        }
        storage::set_listing(&env, listing_id, &listing);

        events::emit_purchased(&env, listing_id, &buyer, amount);
    }

    pub fn cancel_listing(env: Env, seller: Address, listing_id: u64) {
        seller.require_auth();

        let mut listing = storage::get_listing(&env, listing_id).expect("listing not found");
        if listing.seller != seller {
            panic!("not the seller");
        }
        if !listing.active {
            panic!("listing is not active");
        }

        let asset_client = token::Client::new(&env, &listing.asset_token);
        asset_client.transfer(&env.current_contract_address(), &seller, &listing.amount);

        listing.active = false;
        listing.amount = 0;
        storage::set_listing(&env, listing_id, &listing);

        events::emit_cancelled(&env, listing_id);
    }

    pub fn get_listing(env: Env, listing_id: u64) -> Listing {
        storage::get_listing(&env, listing_id).expect("listing not found")
    }
}
