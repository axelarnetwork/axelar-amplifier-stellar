use stellar_axelar_std::address::AddressExt;
use stellar_axelar_std::xdr::{
    AccountId, AlphaNum12, AlphaNum4, Asset as XdrAsset, AssetCode12, AssetCode4, Limits,
    PublicKey, WriteXdr,
};
use stellar_axelar_std::{
    contract, contractimpl, ensure, soroban_sdk, Address, Bytes, Env, String,
};

use crate::error::ContractError;
use crate::interface::StellarTokenUtilsInterface;
use crate::r#macro::{address_to_account_id, string_to_asset_code};

#[contract]
pub struct StellarTokenUtils;

#[contractimpl]
impl StellarTokenUtils {
    pub fn __constructor(_env: &Env) {}

    /// Private method to create asset XDR representation
    fn create_asset_xdr(
        &self,
        env: &Env,
        code: &String,
        issuer: &Address,
    ) -> Result<Bytes, ContractError> {
        // Validate code length - Stellar asset codes must be 12 characters or less
        if code.len() > 12 {
            return Err(ContractError::InvalidAssetCode);
        }

        let asset = {
            // Convert issuer address to AccountId using macro
            let issuer_account_id = address_to_account_id!(issuer);

            if code.len() <= 4 {
                let code_array = string_to_asset_code!(code, 4);
                XdrAsset::CreditAlphanum4(AlphaNum4 {
                    asset_code: AssetCode4(code_array),
                    issuer: issuer_account_id,
                })
            } else {
                let code_array = string_to_asset_code!(code, 12);
                XdrAsset::CreditAlphanum12(AlphaNum12 {
                    asset_code: AssetCode12(code_array),
                    issuer: issuer_account_id,
                })
            }
        };

        let xdr_bytes = asset
            .to_xdr(Limits::none())
            .map_err(|_| ContractError::InvalidAssetCode)?;
        Ok(Bytes::from_slice(env, &xdr_bytes))
    }
}

#[contractimpl]
impl StellarTokenUtilsInterface for StellarTokenUtils {
    fn stellar_asset_contract_address(
        env: Env,
        code: String,
        issuer: Address,
    ) -> Result<Address, ContractError> {
        ensure!(!code.is_empty(), ContractError::InvalidAssetCode);

        let serialized_asset = Self.create_asset_xdr(&env, &code, &issuer)?;

        let deployer = env.deployer().with_stellar_asset(serialized_asset);
        let sac_address = deployer.deployed_address();

        Ok(sac_address)
    }
}
