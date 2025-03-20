#![cfg(test)]
extern crate std;

use soroban_sdk::contractimpl;
use stellar_axelar_std::Ownable;
use soroban_sdk::contract;
use soroban_sdk::Symbol;
use soroban_sdk::Env;
use soroban_sdk::Address;
use stellar_axelar_std::interfaces;
use soroban_sdk::symbol_short;

#[contract]
#[derive(Ownable)]
pub struct TestSubscriptionContract;

#[contractimpl]
impl TestSubscriptionContract {
    const SUBSCRIBER_KEY: Symbol = symbol_short!("subscribe");

    pub fn __constructor(env: Env, owner: Address) {
        interfaces::set_owner(&env, &owner);
        env.storage().instance().set(&Self::SUBSCRIBER_KEY, &None::<Address>);
    }

    pub fn subscribe(env: &Env, user: Address) {
        let owner = Self::owner(env);
        owner.require_auth();

        let current_subscriber: Option<Address> = env
            .storage()
            .instance()
            .get(&Self::SUBSCRIBER_KEY)
            .unwrap_or(None);

        if current_subscriber.is_some() {
            panic!("A subscriber already exists");
        }

        env.storage().instance().set(&Self::SUBSCRIBER_KEY, &Some(user));
    }

    pub fn unsubscribe(env: &Env) {
        let owner = Self::owner(env);
        owner.require_auth();

        let current_subscriber: Option<Address> = env
            .storage()
            .instance()
            .get(&Self::SUBSCRIBER_KEY)
            .unwrap_or(None);

        if current_subscriber.is_none() {
            panic!("No subscriber to unsubscribe");
        }

        env.storage().instance().set(&Self::SUBSCRIBER_KEY, &None::<Address>);
    }

    pub fn is_subscribed(env: &Env, user: Address) -> bool {
        let current_subscriber: Option<Address> = env
            .storage()
            .instance()
            .get(&Self::SUBSCRIBER_KEY)
            .unwrap_or(None);

        current_subscriber == Some(user)
    }
}
