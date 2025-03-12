use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val, Vec};

use crate::error::ContractError;
use crate::interface::MulticallInterface;

#[contract]
pub struct Multicall;

#[contractimpl]
impl MulticallInterface for Multicall {
    fn multicall(
        _env: Env,
        _contract_address: Address,
        _funcs: Vec<Symbol>,
        _args: Vec<Val>,
    ) -> Result<Vec<Val>, ContractError> {
        todo!("Implement multicall function")
    }
}
