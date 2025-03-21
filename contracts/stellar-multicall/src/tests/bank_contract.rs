#![cfg(test)]
extern crate std;

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};
use stellar_axelar_std::{interfaces, Ownable};

#[contract]
#[derive(Ownable)]
pub struct TestBankContract;

#[contractimpl]
impl TestBankContract {
    const BALANCE_KEY: Symbol = symbol_short!("balance");

    pub fn __constructor(env: Env, owner: Address) {
        interfaces::set_owner(&env, &owner);
        env.storage().instance().set(&Self::BALANCE_KEY, &0u32);
    }

    pub fn balance(env: &Env) -> u32 {
        env.storage()
            .instance()
            .get(&Self::BALANCE_KEY)
            .unwrap_or(0u32)
    }

    pub fn deposit(env: &Env, amount: u32) {
        let owner = Self::owner(env);
        owner.require_auth();

        let current_balance: u32 = env
            .storage()
            .instance()
            .get(&Self::BALANCE_KEY)
            .unwrap_or(0u32);
        let new_balance = current_balance + amount;
        env.storage()
            .instance()
            .set(&Self::BALANCE_KEY, &new_balance);
    }

    pub fn withdraw(env: &Env, amount: u32) {
        let owner = Self::owner(env);
        owner.require_auth();

        let current_balance: u32 = env
            .storage()
            .instance()
            .get(&Self::BALANCE_KEY)
            .unwrap_or(0u32);
        if current_balance < amount {
            panic!("Insufficient balance");
        }
        let new_balance = current_balance - amount;
        env.storage()
            .instance()
            .set(&Self::BALANCE_KEY, &new_balance);
    }
}
