use soroban_sdk::{contract, contractimpl, Env, Val, Vec};
use stellar_axelar_std::assert_some;

use crate::error::ContractError;
use crate::interface::MulticallInterface;
use crate::types::MulticallData;

#[contract]
pub struct Multicall;

#[contractimpl]
impl MulticallInterface for Multicall {
    fn multicall(env: Env, params: Vec<MulticallData>) -> Result<Vec<Val>, ContractError> {
        let mut results = Vec::new(&env);

        for i in 0..params.len() {
            let param = assert_some!(params.get(i));
            let contract_address = &param.contract_address;
            let function = &param.function;
            let args = &param.args;

            let result: Result<Val, ContractError> =
                env.invoke_contract(contract_address, function, args.clone());

            match result {
                Ok(val) => results.push_back(val),
                Err(_) => return Err(ContractError::MulticallFailed),
            }
        }

        Ok(results)
    }
}
