#![cfg(test)]
extern crate alloc;
extern crate std;

use alloc::string::ToString;
use std::vec::Vec;

use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::token::TokenClient;
use stellar_axelar_std::{bytes, Address, Bytes, String};

use super::testutils::{
    address_to_string, assert_valid_contract_address, create_asset_xdr, setup, str_to_address,
};

const TEST_ISSUER_1: &str = "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
const TEST_ISSUER_2: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";

#[test]
fn create_stellar_asset_contract_succeeds_with_valid_xdr() {
    let (env, client) = setup();
    let asset_code = "USDC";
    let issuer = str_to_address(&env, TEST_ISSUER_1);
    let asset_xdr = create_asset_xdr(&env, asset_code, &issuer);
    let asset_address = client.create_stellar_asset_contract(&asset_xdr);

    assert_valid_contract_address(&asset_address);

    let token_client = TokenClient::new(&env, &asset_address);
    let test_account = Address::generate(&env);

    assert_eq!(token_client.balance(&test_account), 0);

    let symbol = token_client.symbol();
    let decimals = token_client.decimals();

    assert_eq!(symbol, String::from_str(&env, asset_code));
    assert_eq!(decimals, 7);

    let address_to_string = address_to_string!([asset_address]);
    goldie::assert_json!(address_to_string);
}

#[test]
#[should_panic(expected = "HostError: Error(Context, ExceededLimit)")]
fn create_stellar_asset_contract_fails_with_empty_asset_xdr() {
    let (env, client) = setup();
    let empty_asset_xdr = Bytes::new(&env);

    client.create_stellar_asset_contract(&empty_asset_xdr);
}

#[test]
#[should_panic(expected = "HostError: Error(Value, InvalidInput)")]
fn create_stellar_asset_contract_fails_with_invalid_asset_xdr() {
    let (env, client) = setup();
    let short_asset_xdr = bytes!(&env, 0x0123456789abcdef0123456789abcdef0123456789abcdef);

    client.create_stellar_asset_contract(&short_asset_xdr);
}

#[test]
fn create_stellar_asset_contract_succeeds_with_different_assets_and_issuers() {
    let (env, client) = setup();
    let test_issuers = [TEST_ISSUER_1, TEST_ISSUER_2];

    let test_assets = ["A", "1234", "USD", "EUR", "TEST", "TESTLONGNAME"];
    let addresses: Vec<Address> = test_assets
        .iter()
        .enumerate()
        .map(|(i, &code)| {
            let issuer = str_to_address(&env, test_issuers[i % 2]);
            let asset_xdr = create_asset_xdr(&env, code, &issuer);
            let address = client.create_stellar_asset_contract(&asset_xdr);

            assert_valid_contract_address(&address);

            address
        })
        .collect();

    let addresses = address_to_string!(addresses);
    goldie::assert_json!(addresses);
}

#[test]
fn create_stellar_asset_contract_consecutive_calls_return_same_address() {
    let (env, client) = setup();
    let issuer = str_to_address(&env, TEST_ISSUER_1);
    let asset_xdr = create_asset_xdr(&env, "REPEAT", &issuer);
    let first_address = client.create_stellar_asset_contract(&asset_xdr);

    assert_valid_contract_address(&first_address);

    let second_address = client.create_stellar_asset_contract(&asset_xdr);
    assert_eq!(first_address, second_address);
}

#[test]
#[should_panic(expected = "HostError: Error(Value, InvalidInput)")]
fn create_stellar_asset_contract_fails_with_invalid_asset_code() {
    let (env, client) = setup();
    let issuer = str_to_address(&env, TEST_ISSUER_1);
    let asset_xdr = create_asset_xdr(&env, "INVALID_ASSET_CODE", &issuer);

    client.create_stellar_asset_contract(&asset_xdr);
}
