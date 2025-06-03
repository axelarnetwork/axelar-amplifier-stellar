#![cfg(test)]
extern crate std;

use stellar_axelar_std::{assert_contract_err, bytes, Bytes, Env};

use crate::error::ContractError;
use crate::{StellarTokenUtils, StellarTokenUtilsClient};

pub struct TestConfig<'a> {
    pub env: Env,
    pub client: StellarTokenUtilsClient<'a>,
}

fn setup<'a>() -> TestConfig<'a> {
    let env = Env::default();
    let contract_id = env.register(StellarTokenUtils, ());
    let client = StellarTokenUtilsClient::new(&env, &contract_id);

    TestConfig { env, client }
}

#[test]
fn stellar_asset_contract_address_succeeds_with_valid_xdr() {
    let TestConfig { env, client } = setup();

    // Create valid asset XDR that's exactly 32 bytes (minimum required length)
    let valid_asset_xdr = bytes!(
        &env,
        0x0000000000000000000000000000000000000000000000000000000000000001
    );

    // Call should succeed and return a valid address
    let result = client.stellar_asset_contract_address(&valid_asset_xdr);

    // Verify that we got a valid address
    assert!(!result.to_string().is_empty());
}

#[test]
fn stellar_asset_contract_address_fails_empty_asset_xdr() {
    let TestConfig { env, client } = setup();

    let empty_asset_xdr = Bytes::new(&env);

    assert_contract_err!(
        client.try_stellar_asset_contract_address(&empty_asset_xdr),
        ContractError::InvalidAssetXdr
    );
}

#[test]
fn stellar_asset_contract_address_fails_short_asset_xdr() {
    let TestConfig { env, client } = setup();

    // Create asset XDR that's shorter than 32 bytes (only 16 bytes)
    let short_asset_xdr = bytes!(&env, 0x0123456789abcdef0123456789abcdef);

    assert_contract_err!(
        client.try_stellar_asset_contract_address(&short_asset_xdr),
        ContractError::InvalidAssetXdr
    );
}
