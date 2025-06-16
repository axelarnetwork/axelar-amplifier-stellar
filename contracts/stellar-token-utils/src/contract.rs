use stellar_axelar_std::token::Client;
use stellar_axelar_std::{contract, contractimpl, ensure, soroban_sdk, Address, Bytes, Env};

use crate::error::ContractError;
use crate::interface::TokenUtilsInterface;

#[contract]
pub struct TokenUtils;

#[contractimpl]
impl TokenUtilsInterface for TokenUtils {
    fn create_stellar_asset_contract(env: Env, asset_xdr: Bytes) -> Result<Address, ContractError> {
        ensure!(asset_xdr.len() >= 40, ContractError::InvalidAssetXdr);

        let deployer = env.deployer().with_stellar_asset(asset_xdr);
        let deployed_address = deployer.deployed_address();

        match Client::new(&env, &deployed_address).try_decimals() {
            Ok(_) => Ok(deployed_address),
            Err(_) => Ok(deployer.deploy()),
        }
    }
}
