use soroban_sdk::{Address, Env, Symbol, Val, Vec};

use crate::error::ContractError;

pub trait MulticallInterface {
    /// Executes an arbitrary list of contract calls and returns the results of all the calls.
    ///
    /// # Arguments
    /// * `contract_address` - The address of the contract to be called.
    /// * `funcs` - The list of functions to be called on the contract.
    /// * `args` - The list of function arguments to be passed to the contract.
    ///
    /// # Returns
    /// - `Ok(Vec<Val>)`: Returns a vector with the return data of each function call
    ///
    /// # Errors
    /// - [`ContractError::MulticallFailed`]: If any of the contract calls fail.
    fn multicall(
        env: Env,
        contract_address: Address,
        funcs: Vec<Symbol>,
        args: Vec<Val>,
    ) -> Result<Vec<Val>, ContractError>;
}
