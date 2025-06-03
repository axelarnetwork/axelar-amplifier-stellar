use stellar_axelar_std::{contractclient, soroban_sdk, Address, Env, String};

use crate::error::ContractError;

#[contractclient(name = "StellarTokenUtilsClient")]
pub trait StellarTokenUtilsInterface {
    fn stellar_asset_contract_address(
        env: Env,
        code: String,
        issuer: Address,
    ) -> Result<Address, ContractError>;
}
