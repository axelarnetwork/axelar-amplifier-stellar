use stellar_axelar_std::{contract, contractimpl, ensure, soroban_sdk, Address, Bytes, Env};

use crate::error::ContractError;
use crate::interface::StellarTokenUtilsInterface;

#[contract]
pub struct StellarTokenUtils;

#[contractimpl]
impl StellarTokenUtilsInterface for StellarTokenUtils {
    /// Resolves the Stellar Asset Contract (SAC) address for a given asset XDR.
    ///
    /// This function takes an asset's XDR representation and returns the corresponding
    /// Stellar Asset Contract address that would be deployed for that asset.
    ///
    /// # Arguments
    /// * `env` - The contract execution environment
    /// * `asset_xdr` - The XDR byte representation of the Stellar asset
    ///
    /// # Returns
    /// * `Ok(Address)` - The resolved Stellar Asset Contract address
    /// * `Err(ContractError::InvalidAssetXdr)` - If the asset XDR is empty, malformed, or invalid
    fn stellar_asset_contract_address(
        env: Env,
        asset_xdr: Bytes,
    ) -> Result<Address, ContractError> {
        // Ensure asset_xdr is at least 32 bytes (Stellar address length)
        ensure!(asset_xdr.len() >= 32, ContractError::InvalidAssetXdr);

        let deployer = env.deployer().with_stellar_asset(asset_xdr);
        let sac_address = deployer.deployed_address();

        Ok(sac_address)
    }
}
