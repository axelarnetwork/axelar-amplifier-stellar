use soroban_sdk::{contract, contractimpl, Env, Val, Vec};

use crate::error::ContractError;
use crate::interface::MulticallInterface;
use crate::types::FunctionCall;

#[contract]
pub struct Multicall;

#[contractimpl]
impl Multicall {
    pub const fn __constructor(_env: &Env) {}
}

#[contractimpl]
impl MulticallInterface for Multicall {
    fn multicall(env: &Env, params: Vec<FunctionCall>) -> Result<Vec<Val>, ContractError> {
        let mut results = Vec::new(env);

        for FunctionCall {
            contract,
            function,
            args,
        } in params.into_iter()
        {
            let result: Val = env.invoke_contract(&contract, &function, args.clone());

            results.push_back(result);
        }

        Ok(results)
    }
}
