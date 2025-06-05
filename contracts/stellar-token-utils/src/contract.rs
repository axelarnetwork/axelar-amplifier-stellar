use stellar_axelar_std::{contract, contractimpl, ensure, soroban_sdk, Address, Bytes, Env};

use crate::error::ContractError;
use crate::interface::TokenUtilsInterface;

#[contract]
pub struct TokenUtils;

#[contractimpl]
impl TokenUtilsInterface for TokenUtils {
    /// Creates the Stellar Asset Contract (SAC) address for a given asset XDR.
    ///
    /// This function takes an asset's XDR representation
    /// and returns the corresponding Stellar Asset Contract address.
    ///
    /// # Arguments
    /// * `asset_xdr` - The XDR byte representation of the Stellar asset
    ///
    /// # Returns
    /// * `Ok(Address)` - The resolved Stellar Asset Contract address
    /// * `Err(ContractError::InvalidAssetXdr)` - If the asset XDR is invalid
    fn create_stellar_asset_contract(env: Env, asset_xdr: Bytes) -> Result<Address, ContractError> {
        // Ensure asset_xdr is at least 32 bytes (Stellar address length)
        ensure!(asset_xdr.len() >= 32, ContractError::InvalidAssetXdr);

        let deployed_address = env.deployer().with_stellar_asset(asset_xdr).deploy();

        Ok(deployed_address)
    }
}
