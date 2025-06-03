use stellar_axelar_std::{contractclient, soroban_sdk, Address, Bytes, Env};

use crate::error::ContractError;

#[contractclient(name = "StellarTokenUtilsClient")]
pub trait StellarTokenUtilsInterface {
    /// Resolves the Stellar Asset Contract (SAC) address for a given asset XDR.
    ///
    /// This function takes an asset's XDR representation
    /// and returns the corresponding Stellar Asset Contract address.
    ///
    /// # Arguments
    /// * `env` - The contract execution environment
    /// * `asset_xdr` - The XDR byte representation of the Stellar asset
    ///
    /// # Returns
    /// * `Ok(Address)` - The resolved Stellar Asset Contract address
    /// * `Err(ContractError::InvalidAssetXdr)` - If the asset XDR is invalid
    fn stellar_asset_contract_address(env: Env, asset_xdr: Bytes)
        -> Result<Address, ContractError>;
}
