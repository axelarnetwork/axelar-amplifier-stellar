use stellar_axelar_std::interfaces::UpgradableClient;
use stellar_axelar_std::{
    contract, contractimpl, ensure, soroban_sdk, symbol_short, Address, BytesN, Env, String,
    Symbol, Val,
};

use crate::error::ContractError;
use crate::interface::UpgraderInterface;

const MIGRATE: Symbol = symbol_short!("migrate");

#[contract]
pub struct Upgrader;

#[contractimpl]
impl Upgrader {
    pub fn __constructor(_env: Env) {}
}

#[contractimpl]
impl UpgraderInterface for Upgrader {
    fn upgrade(
        env: Env,
        contract_address: Address,
        new_version: String,
        new_wasm_hash: BytesN<32>,
        migration_data: stellar_axelar_std::Vec<Val>,
    ) -> Result<(), ContractError> {
        let contract_client = UpgradableClient::new(&env, &contract_address);

        contract_client.required_auths().iter().for_each(|addr| {
            addr.require_auth();
        });

        ensure!(
            contract_client.version() != new_version,
            ContractError::SameVersion
        );

        contract_client.upgrade(&new_wasm_hash);
        // The types of the arguments to the migrate function are unknown to this contract, so we need to call it with invoke_contract.
        // The migrate function's return value can be safely cast to Val no matter what it really is
        let _: Val = env.invoke_contract(&contract_address, &MIGRATE, migration_data);

        ensure!(
            contract_client.version() == new_version,
            ContractError::UnexpectedNewVersion
        );
        Ok(())
    }
}
