use stellar_axelar_std::{Env, Val, Vec};

use crate::error::ContractError;
use crate::types::FunctionCall;

//#[contractclient(name = "MulticallClient")]
pub trait MulticallInterface {
    /// Executes an arbitrary list of contract calls and returns the results of all the calls.
    ///
    /// # Arguments
    /// * `function_calls` - A list of params containing the contract address, function name and arguments for each contract call.
    ///
    /// # Returns
    /// - `Ok(Vec<Val>)`: Returns a vector with the return data of each function call
    ///
    /// # Errors:
    /// - Propagates any error that occurs during the execution of the contract calls.
    fn multicall(env: &Env, function_calls: Vec<FunctionCall>) -> Result<Vec<Val>, ContractError>;
}
