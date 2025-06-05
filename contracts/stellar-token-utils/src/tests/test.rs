#![cfg(test)]
extern crate alloc;
extern crate std;

use alloc::string::ToString;

use stellar_axelar_std::{assert_contract_err, assert_ok, bytes, Bytes, Env};

use crate::error::ContractError;
use crate::{TokenUtils, TokenUtilsClient};

const CONTRACT_ADDRESS_PREFIX: char = 'C';

pub struct TestConfig<'a> {
    pub env: Env,
    pub client: TokenUtilsClient<'a>,
}

fn setup<'a>() -> TestConfig<'a> {
    let env = Env::default();
    let contract_id = env.register(TokenUtils, ());
    let client = TokenUtilsClient::new(&env, &contract_id);

    TestConfig { env, client }
}

#[test]
fn create_stellar_asset_contract_succeeds_with_valid_xdr() {
    let TestConfig { env, client } = setup();

    let valid_asset_xdr = bytes!(
        &env,
        0x0000000155534400000000002dbb7dfec733df8c4b044d3ae01e5fce901071a19b2b2cf903acaa16299f8d56
    );

    let contract_address = assert_ok!(client.try_create_stellar_asset_contract(&valid_asset_xdr))
        .expect("Contract creation should succeed with valid XDR");

    let address_string = contract_address.to_string().to_string();

    assert!(
        !address_string.is_empty(),
        "Contract address should not be empty"
    );

    assert!(
        address_string.starts_with(CONTRACT_ADDRESS_PREFIX),
        "Contract address should start with '{}', got: {}",
        CONTRACT_ADDRESS_PREFIX,
        address_string
    );
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
