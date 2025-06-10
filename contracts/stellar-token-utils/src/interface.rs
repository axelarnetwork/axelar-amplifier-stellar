use stellar_axelar_std::{Address, Bytes, Env};

use crate::error::ContractError;

pub trait TokenUtilsInterface {
    /// Deploys the Stellar Asset Contract (SAC) address for a given asset XDR.
    ///
    /// This function takes an asset's XDR representation and deploys the corresponding
    /// Stellar Asset Contract. It will fail if the contract is already deployed.
    ///
    /// This utility is specifically designed for Stellar Classic tokens (native Stellar assets),
    /// allowing them to be used in Soroban smart contracts through the SAC interface.
    ///
    /// # Arguments
    /// * `asset_xdr` - The XDR byte representation of the Stellar asset
    ///
    /// # Returns
    /// * `Ok(Address)` - The deployed Stellar Asset Contract address
    /// * `Err(ContractError::InvalidAssetXdr)` - If the asset XDR is invalid
    /// * `Err` - If the contract is already deployed (deployment error)
    ///
    /// # Usage
    /// This function is used to deploy a new Stellar Asset Contract for a specific asset.
    /// It takes the XDR representation of a Stellar asset (code and issuer) and creates
    /// a new contract instance for that asset.
    ///
    /// To obtain the asset XDR:
    /// 1. Use Stellar SDK libraries (like js-stellar-sdk) to create an Asset object
    /// 2. Serialize the Asset to XDR format
    /// 3. Convert the XDR to bytes
    ///
    /// Example with js-stellar-sdk:
    /// ```javascript
    /// const asset = new StellarSdk.Asset('USD', 'ISSUER_PUBLIC_KEY');
    /// const xdr = asset.toXDRObject().toXDR('base64');
    /// // Convert base64 to bytes for contract input
    /// ```
    ///
    /// Note that this function will fail if the contract for the given asset is already
    /// deployed on the network.
    fn deploy_stellar_asset_contract(env: Env, asset_xdr: Bytes) -> Result<Address, ContractError>;
}
