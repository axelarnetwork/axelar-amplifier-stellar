//! Base for the dummy.wasm file. This is the dummy contract after upgrade.

use axelar_soroban_std::interfaces;
use axelar_soroban_std::interfaces::{OwnableInterface, UpgradableInterface};
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, BytesN, Env};

#[contract]
pub struct DummyContract;

#[contractimpl]
impl UpgradableInterface for DummyContract {
    fn version(env: &Env) -> soroban_sdk::String {
        soroban_sdk::String::from_str(env, "0.2.0")
    }

    fn upgrade(env: &Env, new_wasm_hash: BytesN<32>) {
        Self::owner(env).require_auth();

        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}

#[contractimpl]
impl OwnableInterface for DummyContract {
    fn owner(env: &Env) -> Address {
        interfaces::owner(env)
    }
}

#[contractimpl]
impl DummyContract {
    pub fn __constructor(env: Env, owner: Address) {
        interfaces::set_owner(&env, &owner);
    }

    pub fn migrate(env: Env, migration_data: soroban_sdk::String) -> Result<(), ContractError> {
        Self::owner(&env).require_auth();
        env.storage()
            .instance()
            .set(&DataKey::Data, &migration_data);
        Ok(())
    }
}

#[contracttype]
pub enum DataKey {
    Data,
}

#[contracterror]
pub enum ContractError {
    SomeFailure = 1,
}
