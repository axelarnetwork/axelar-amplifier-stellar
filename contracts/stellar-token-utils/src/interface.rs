use stellar_axelar_std::{Address, Bytes, Env};

use crate::error::ContractError;

pub trait TokenUtilsInterface {
    /// Create Stellar Asset Contract
    ///
    /// This function takes an asset's XDR representation and creates the corresponding
    /// Stellar Asset Contract. If the contract is already deployed, it returns the
    /// existing contract address instead of failing.
    ///
    /// This utility is specifically designed for Stellar Classic tokens (native Stellar assets),
    /// not for Soroban custom tokens. The asset XDR must contain a valid Stellar asset
    /// representation with both the asset code and issuer information.
    ///
    /// # Arguments
    /// * `asset_xdr` - Bytes representing the XDR encoding of a Stellar asset
    ///   - Must be at least 40 bytes to accommodate:
    ///     - Asset type discriminant (4 bytes)
    ///     - Asset code (4 bytes for AlphaNum4, 12 bytes for AlphaNum12)
    ///     - Issuer Ed25519 public key (32 bytes)
    ///
    /// # Returns
    /// * `Ok(Address)` - The created Stellar Asset Contract address (existing or newly created)
    /// * `Err(ContractError::InvalidAssetXdr)` - If the asset XDR is invalid
    ///
    /// # Usage
    /// This function is used to create a new Stellar Asset Contract for a specific asset.
    /// It takes the XDR representation of a Stellar asset (code and issuer) and creates
    /// a new contract instance for that asset, or returns the existing one if already created.
    ///
    /// To obtain the asset XDR:
    /// 1. Use stellar-sdk to create an Asset object with code and issuer
    /// 2. Convert the Asset to XDR bytes
    /// 3. Pass the XDR bytes to this function
    ///
    /// Example with stellar-sdk (JavaScript):
    /// ```javascript
    /// const asset = new Asset('USDC', 'ISSUER_ADDRESS');
    /// const xdr = asset.toXDRObject().toXDR('base64');
    /// // Convert base64 to bytes for contract input
    /// ```
    fn create_stellar_asset_contract(env: Env, asset_xdr: Bytes) -> Result<Address, ContractError>;
}
