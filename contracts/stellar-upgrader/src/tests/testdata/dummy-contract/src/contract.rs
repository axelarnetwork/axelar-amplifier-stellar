//! Dummy contract to test the `Upgrader` contract

use stellar_axelar_std::interfaces::{OwnableInterface, UpgradableInterface};
use stellar_axelar_std::{
    contract, contracterror, contractimpl, interfaces, soroban_sdk, vec, Address, BytesN, Env,
    String, Vec,
};

#[contract]
pub struct DummyContract;

#[contractimpl]
impl UpgradableInterface for DummyContract {
    fn version(env: &Env) -> String {
        String::from_str(env, "0.1.0")
    }

    fn required_auths(env: &Env) -> Vec<Address> {
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
}

#[contracterror]
pub enum ContractError {
    SomeFailure = 1,
}

pub mod storage {
    use stellar_axelar_std::{contractstorage, soroban_sdk, String};

    #[contractstorage]
    enum DataKey {
        #[instance]
        #[value(String)]
        Data,
    }
}
