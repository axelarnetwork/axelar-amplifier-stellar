use soroban_sdk::{Env, Val, Vec};

use crate::error::ContractError;
use crate::types::MulticallData;

pub trait MulticallInterface {
    /// Executes an arbitrary list of contract calls and returns the results of all the calls.
    ///
    /// # Arguments
    /// * `params` - A list of params containing the contract address, function name and arguments for each contract call.
    ///
    /// # Returns
    /// - `Ok(Vec<Val>)`: Returns a vector with the return data of each function call
    ///
    /// # Errors
    /// - [`ContractError::MulticallFailed`]: If any of the contract calls fail.
    fn multicall(env: Env, params: Vec<MulticallData>) -> Result<Vec<Val>, ContractError>;
}
