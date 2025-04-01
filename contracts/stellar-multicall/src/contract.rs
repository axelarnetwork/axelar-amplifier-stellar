use stellar_axelar_std::{contract, contractimpl, soroban_sdk, Env, Val, Vec};

use crate::error::ContractError;
use crate::interface::MulticallInterface;
use crate::types::FunctionCall;

#[contract]
pub struct Multicall;

#[contractimpl]
impl Multicall {
    pub fn __constructor(_env: &Env) {}
}

#[contractimpl]
impl MulticallInterface for Multicall {
    fn multicall(env: &Env, function_calls: Vec<FunctionCall>) -> Result<Vec<Val>, ContractError> {
        let mut results = Vec::new(env);

        for FunctionCall {
            contract,
            approver,
            function,
            args,
        } in function_calls.into_iter()
        {
            approver.require_auth();

            let result: Val = env.invoke_contract(&contract, &function, args.clone());

            results.push_back(result);
        }

        Ok(results)
    }
}
