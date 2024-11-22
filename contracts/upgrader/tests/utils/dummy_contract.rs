use axelar_soroban_std::UpgradeableInterface;
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, BytesN, Env};

/// A simple contract to test the upgrader
#[contract]
pub struct DummyContract;

/// Dummy contract logic before upgrade
#[contractimpl]
impl UpgradeableInterface for DummyContract {
    fn version(env: Env) -> soroban_sdk::String {
        soroban_sdk::String::from_str(&env, "0.1.0")
    }

    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        Self::owner(&env).require_auth();

        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}

#[contractimpl]
impl DummyContract {
    pub fn __constructor(env: Env, owner: Address) {
        env.storage().instance().set(&DataKey::Owner, &owner)
    }

    fn owner(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Owner).unwrap()
    }
}

#[contracttype]
pub enum DataKey {
    Data,
    Owner,
}

#[contracterror]
pub enum ContractError {
    SomeFailure = 1,
}

// Dummy contract logic after upgrade is available as testdata/dummy.wasm
//
// #[contractimpl]
// impl UpgradeableInterface for DummyContract {
//     fn version(env: Env) -> String {
//         String::from_str(&env, "0.2.0")
//     }
//
//     fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
//         Self::owner(&env).require_auth();
//
//         env.deployer().update_current_contract_wasm(new_wasm_hash);
//     }
// }
//
// #[contractimpl]
// impl DummyContract {
//     pub fn migrate(env: Env, migration_data: String) -> Result<(), ContractError> {
//         Self::owner(&env).require_auth();
//
//         env.storage()
//             .instance()
//             .set(&DataKey::Data, &migration_data);
//
//         Ok(())
//     }
//
//     fn owner(env: &Env) -> Address {
//         env.storage().instance().get(&DataKey::Owner).unwrap()
//     }
// }
