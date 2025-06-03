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
fn create_stellar_asset_contract_succeeds_with_valid_xdr() {
    let TestConfig { env, client } = setup();

    let test_asset_xdr = bytes!(
        &env,
        0x0000000155534400000000002dbb7dfec733df8c4b044d3ae01e5fce901071a19b2b2cf903acaa16299f8d56
    );

    let result = client.try_create_stellar_asset_contract(&test_asset_xdr);

    assert!(result.is_ok());
}

#[test]
fn create_stellar_asset_contract_fails_empty_asset_xdr() {
    let TestConfig { env, client } = setup();

    let empty_asset_xdr = Bytes::new(&env);

    assert_contract_err!(
        client.try_create_stellar_asset_contract(&empty_asset_xdr),
        ContractError::InvalidAssetXdr
    );
}

#[test]
fn create_stellar_asset_contract_fails_short_asset_xdr() {
    let TestConfig { env, client } = setup();

    // Create asset XDR that's shorter than 32 bytes (only 16 bytes)
    let short_asset_xdr = bytes!(&env, 0x0123456789abcdef0123456789abcdef);

    assert_contract_err!(
        client.try_create_stellar_asset_contract(&short_asset_xdr),
        ContractError::InvalidAssetXdr
    );
}
