use stellar_axelar_std::token::Client;
use stellar_axelar_std::{contract, contractimpl, soroban_sdk, Address, Bytes, Env};

use crate::interface::TokenUtilsInterface;

#[contract]
pub struct TokenUtils;

#[contractimpl]
impl TokenUtilsInterface for TokenUtils {
    fn create_stellar_asset_contract(env: Env, asset_xdr: Bytes) -> Address {
        let deployer = env.deployer().with_stellar_asset(asset_xdr);
        let deployed_address = deployer.deployed_address();

        // Return if the asset contract has already been deployed
        // This prevents failures triggered by frontrunning this method
        if Client::new(&env, &deployed_address).try_decimals().is_ok() {
            return deployed_address;
        }

        deployer.deploy()
    }
}
