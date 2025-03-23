//! Base for the dummy_contract_after_upgrade.wasm file. This is the dummy contract after upgrade.

use soroban_sdk::{contract, contracterror, vec, Address, BytesN, Env};
use stellar_axelar_std::interfaces::{OwnableInterface, UpgradableInterface};
use stellar_axelar_std::{contractimpl, contractstorage, interfaces};

#[contract]
pub struct DummyContract;

#[contractimpl]
impl UpgradableInterface for DummyContract {
    fn version(env: &Env) -> soroban_sdk::String {
        soroban_sdk::String::from_str(env, "0.2.0")
    }

    fn required_auths(env: &Env) -> soroban_sdk::Vec<Address> {
        vec![env, Self::owner(env)]
    }

    fn upgrade(env: &Env, new_wasm_hash: BytesN<32>) {
        Self::required_auths(env)
            .iter()
            .for_each(|addr| addr.require_auth());
        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}

#[contractimpl]
impl OwnableInterface for DummyContract {
    fn owner(env: &Env) -> Address {
        interfaces::owner(env)
    }

    fn transfer_ownership(env: &Env, new_owner: Address) {
        interfaces::transfer_ownership::<Self>(env, new_owner);
    }
}

#[contractimpl]
impl DummyContract {
    pub fn __constructor(env: Env, owner: Address) {
        interfaces::set_owner(&env, &owner);
    }

    pub fn migrate(env: Env, migration_data: soroban_sdk::String) -> Result<(), ContractError> {
        Self::required_auths(&env)
            .iter()
            .for_each(|addr| addr.require_auth());

        storage::set_data(&env, &migration_data);

        Ok(())
    }
}

#[contracterror]
pub enum ContractError {
    SomeFailure = 1,
    MigrationInProgress = 2,
}

mod storage {
    use super::*;

    #[contractstorage]
    enum DataKey {
        #[instance]
        #[value(soroban_sdk::String)]
        Data,
    }
}
