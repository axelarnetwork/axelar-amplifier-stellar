use crate::error::ContractError;
use axelar_soroban_std::ensure;
use axelar_soroban_std::shared_interfaces::UpgradableClient;
use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, BytesN, Env, String, Symbol, Val,
};

const MIGRATE: Symbol = symbol_short!("migrate");

#[contract]
pub struct Upgrader;

#[contractimpl]
impl Upgrader {
    pub fn __constructor(_env: Env) {}

    pub fn upgrade(
        env: Env,
        contract_address: Address,
        new_version: String,
        new_wasm_hash: BytesN<32>,
        migration_data: soroban_sdk::Vec<Val>,
    ) -> Result<(), ContractError> {
        let contract_client = UpgradableClient::new(&env, &contract_address);

        ensure!(
            contract_client.version() != new_version,
            ContractError::SameVersion
        );

        contract_client.upgrade(&new_wasm_hash);
        // The types of the arguments to the migrate function are unknown to this contract, so we need to call it with invoke_contract.
        // The migrate function's return value can be safely cast to () no matter what it really is,
        // because it will panic on failure anyway
        env.invoke_contract::<()>(&contract_address, &MIGRATE, migration_data);

        ensure!(
            contract_client.version() == new_version,
            ContractError::UnexpectedNewVersion
        );
        Ok(())
    }
}
