use stellar_axelar_std::token::Client;
use stellar_axelar_std::{contract, contractimpl, ensure, soroban_sdk, Address, Bytes, Env};

use crate::error::ContractError;
use crate::interface::TokenUtilsInterface;

#[contract]
pub struct TokenUtils;

#[contractimpl]
impl TokenUtilsInterface for TokenUtils {
    /// Deploy Stellar Asset Contract
    ///
    /// This function takes an asset's XDR representation and deploys the corresponding
    /// Stellar Asset Contract. If the contract is already deployed, it returns the
    /// existing contract address instead of failing.
    ///
    /// This utility is specifically designed for Stellar Classic tokens (native Stellar assets),
    /// not for Soroban custom tokens. The asset XDR must contain a valid Stellar asset
    /// representation with both the asset code and issuer information.
    ///
    /// # Arguments
    /// * `asset_xdr` - Bytes representing the XDR encoding of a Stellar asset
    ///   - Must be at least 32 bytes (minimum for a valid Stellar address)
    ///   - Contains asset code (4 or 12 bytes) and issuer address (32 bytes)
    ///
    /// # Returns
    /// * `Ok(Address)` - The deployed Stellar Asset Contract address (existing or newly deployed)
    /// * `Err(ContractError::InvalidAssetXdr)` - If the asset XDR is invalid
    ///
    /// # Usage
    /// This function is used to deploy a new Stellar Asset Contract for a specific asset.
    /// It takes the XDR representation of a Stellar asset (code and issuer) and creates
    /// a new contract instance for that asset, or returns the existing one if already deployed.
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
    fn deploy_stellar_asset_contract(env: Env, asset_xdr: Bytes) -> Result<Address, ContractError> {
        ensure!(asset_xdr.len() >= 32, ContractError::InvalidAssetXdr);

        let deployer = env.deployer().with_stellar_asset(asset_xdr);
        let deployed_address = deployer.deployed_address();
        
        match Client::new(&env, &deployed_address).try_decimals() {
            Ok(_) => Ok(deployed_address),
            Err(_) => Ok(deployer.deploy()),
        }
    }
}
