use stellar_axelar_std::{contractclient, soroban_sdk, Address, Bytes, Env};

use crate::error::ContractError;

pub trait StellarTokenUtilsInterface {
    /// Creates the Stellar Asset Contract (SAC) address for a given asset XDR.
    ///
    /// This function returns the address of the Stellar Asset Contract for
    /// the specified asset, always deploying it when called. The
    /// address is deterministic based on the asset's XDR representation.
    ///
    /// # Arguments
    /// * `env` - The Stellar contract execution environment variable
    /// * `asset_xdr` - The XDR (External Data Representation) bytes of the
    ///                 Stellar asset. This should be a properly formatted
    ///                 Stellar asset XDR as defined by the Stellar protocol.
    ///                 Must be at least 32 bytes (Stellar address length).
    ///
    /// # Returns
    /// - `Ok(Address)`: The deterministic address where the Stellar Asset
    ///                  Contract for this asset would be (or is) deployed.
    /// - `Err(ContractError::InvalidAssetXdr)`: If the provided asset XDR
    ///                                          is empty, too short (<32 bytes),
    ///                                          malformed, or invalid.
    ///
    fn create_stellar_asset_contract(env: Env, asset_xdr: Bytes) -> Result<Address, ContractError>;
}
