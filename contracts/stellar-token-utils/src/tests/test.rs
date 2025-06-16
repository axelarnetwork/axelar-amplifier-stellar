#![cfg(test)]
extern crate alloc;
extern crate std;

use alloc::string::ToString;
use std::vec::Vec;

use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::token::TokenClient;
use stellar_axelar_std::xdr::{
    AccountId, AlphaNum12, AlphaNum4, Asset as XdrAsset, AssetCode12, AssetCode4, Limits,
    PublicKey as XdrPublicKey, Uint256, WriteXdr,
};
use stellar_axelar_std::{assert_contract_err, bytes, Address, Bytes, Env, String};
use stellar_strkey::ed25519::PublicKey;

use super::testutils::{address_strings, setup};
use crate::error::ContractError;

const CONTRACT_ADDRESS_PREFIX: char = 'C';
const TEST_ISSUER_1: &str = "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
const TEST_ISSUER_2: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";

fn create_issuer(env: &Env, address: &str) -> Address {
    Address::from_string(&String::from_str(env, address))
}

fn address_to_account_id(address: &Address) -> AccountId {
    let address_string = address.to_string().to_string();

    let issuer_pk = PublicKey::from_string(&address_string)
        .expect("Address should convert to a valid Ed25519 public key");

    let uint256 = Uint256(issuer_pk.0);
    let pk = XdrPublicKey::PublicKeyTypeEd25519(uint256);

    AccountId(pk)
}

fn string_to_asset_code<const N: usize>(code: &str) -> [u8; N] {
    std::array::from_fn(|i| code.as_bytes().get(i).copied().unwrap_or(0))
}

fn create_asset_xdr(env: &Env, code: &str, issuer: &Address) -> Bytes {
    let issuer_account_id = address_to_account_id(issuer);

    let asset = if code.len() <= 4 {
        XdrAsset::CreditAlphanum4(AlphaNum4 {
            asset_code: AssetCode4(string_to_asset_code::<4>(code)),
            issuer: issuer_account_id,
        })
    } else {
        XdrAsset::CreditAlphanum12(AlphaNum12 {
            asset_code: AssetCode12(string_to_asset_code::<12>(code)),
            issuer: issuer_account_id,
        })
    };

    let asset_xdr = asset.to_xdr(Limits::none()).unwrap();
    Bytes::from_slice(env, &asset_xdr)
}

fn assert_valid_contract_address(address: &Address) {
    let address = address.to_string().to_string();
    assert!(!address.is_empty());
    assert!(address.starts_with(CONTRACT_ADDRESS_PREFIX));
}

#[test]
fn create_stellar_asset_contract_succeeds_with_valid_xdr() {
    let (env, client) = setup();
    let asset_code = "USDC";
    let issuer = create_issuer(&env, TEST_ISSUER_1);
    let asset_xdr = create_asset_xdr(&env, asset_code, &issuer);

    let deployed_address = client.create_stellar_asset_contract(&asset_xdr);

    assert_valid_contract_address(&deployed_address);

    let token_client = TokenClient::new(&env, &deployed_address);
    let test_account = Address::generate(&env);

    assert_eq!(token_client.balance(&test_account), 0);

    let symbol = token_client.symbol();
    let decimals = token_client.decimals();

    assert_eq!(symbol, String::from_str(&env, asset_code));
    assert_eq!(decimals, 7);

    let address_strings = address_strings!([deployed_address]);
    goldie::assert_json!(address_strings);
}

#[test]
fn create_stellar_asset_contract_fails_empty_asset_xdr() {
    let (env, client) = setup();
    let empty_asset_xdr = Bytes::new(&env);

    assert_contract_err!(
        client.try_create_stellar_asset_contract(&empty_asset_xdr),
        ContractError::InvalidAssetXdr
    );
}

#[test]
fn create_stellar_asset_contract_fails_short_asset_xdr() {
    let (env, client) = setup();
    let short_asset_xdr = bytes!(
        &env,
        0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcd
    );

    assert_contract_err!(
        client.try_create_stellar_asset_contract(&short_asset_xdr),
        ContractError::InvalidAssetXdr
    );
}

#[test]
fn create_stellar_asset_contract_different_assets_succeed_and_address_derivation_unchanged() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);

    let test_assets = ["USD", "EUR", "GBP", "JPY"];
    let addresses: Vec<Address> = test_assets
        .iter()
        .map(|&code| {
            let asset_xdr = create_asset_xdr(&env, code, &issuer);
            let address = client.create_stellar_asset_contract(&asset_xdr);

            assert_valid_contract_address(&address);

            address
        })
        .collect();

    let address_strings = address_strings!(addresses);
    goldie::assert_json!(address_strings);
}

#[test]
fn create_stellar_asset_contract_same_asset_code_different_issuers_address_derivation_unchanged() {
    let (env, client) = setup();
    let issuer1 = create_issuer(&env, TEST_ISSUER_1);
    let issuer2 = create_issuer(&env, TEST_ISSUER_2);

    let asset_code = "STABLE";
    let asset_xdr1 = create_asset_xdr(&env, asset_code, &issuer1);
    let asset_xdr2 = create_asset_xdr(&env, asset_code, &issuer2);

    let address1 = client.create_stellar_asset_contract(&asset_xdr1);
    let address2 = client.create_stellar_asset_contract(&asset_xdr2);

    let addresses = [address1, address2];
    let address_strings = address_strings!(addresses);

    goldie::assert_json!(address_strings);
}

#[test]
fn create_stellar_asset_contract_alphanum4() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);

    let alphanum4_xdr = create_asset_xdr(&env, "TEST", &issuer);
    let alphanum4_address = client.create_stellar_asset_contract(&alphanum4_xdr);

    assert_valid_contract_address(&alphanum4_address);

    let address_strings = address_strings!([alphanum4_address]);
    goldie::assert_json!(address_strings);
}

#[test]
fn create_stellar_asset_contract_alphanum12() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);

    let alphanum12_xdr = create_asset_xdr(&env, "TESTLONGNAME", &issuer);
    let alphanum12_address = client.create_stellar_asset_contract(&alphanum12_xdr);

    assert_valid_contract_address(&alphanum12_address);

    let address_strings = address_strings!([alphanum12_address]);
    goldie::assert_json!(address_strings);
}

#[test]
fn create_stellar_asset_contract_same_issuer_different_asset_code_address_derivation_unchanged() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);

    let alphanum4_xdr = create_asset_xdr(&env, "TEST", &issuer);
    let alphanum4_address = client.create_stellar_asset_contract(&alphanum4_xdr);

    let alphanum12_xdr = create_asset_xdr(&env, "TESTLONGNAME", &issuer);
    let alphanum12_address = client.create_stellar_asset_contract(&alphanum12_xdr);

    let addresses = [alphanum4_address, alphanum12_address];
    let address_strings = address_strings!(addresses);

    goldie::assert_json!(address_strings);
}

#[test]
fn create_stellar_asset_contract_consecutive_calls_return_same_address() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);
    let asset_xdr = create_asset_xdr(&env, "REPEAT", &issuer);

    let first_address = client.create_stellar_asset_contract(&asset_xdr);
    assert_valid_contract_address(&first_address);

    let second_address = client.create_stellar_asset_contract(&asset_xdr);
    assert_eq!(
        first_address, second_address,
        "Consecutive calls should return the same address (idempotent behavior)"
    );
}

#[test]
fn create_stellar_asset_contract_multiple_redeploys_return_same_address() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);
    let asset_xdr = create_asset_xdr(&env, "MULTI", &issuer);

    let addresses: Vec<Address> = (0..5)
        .map(|_| client.create_stellar_asset_contract(&asset_xdr))
        .collect();

    for address in &addresses {
        assert_eq!(
            address, &addresses[0],
            "All deployments should return the same address"
        );
        assert_valid_contract_address(address);
    }
}

#[test]
fn create_stellar_asset_contract_idempotent_deployment() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);
    let asset_xdr = create_asset_xdr(&env, "BTC", &issuer);

    let first_address = client.create_stellar_asset_contract(&asset_xdr);

    let second_address = client.create_stellar_asset_contract(&asset_xdr);
    assert_eq!(
        first_address, second_address,
        "Function should be idempotent and return the same address when asset is already deployed"
    );
}

#[test]
fn create_stellar_asset_contract_idempotent_behavior_comprehensive() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);

    let test_cases = [
        ("USD", "short asset code"),
        ("LONGASSETNAME", "long asset code"),
        ("A", "single character"),
        ("1234", "numeric code"),
    ];

    for (asset_code, description) in test_cases {
        let asset_xdr = create_asset_xdr(&env, asset_code, &issuer);

        let first_address = client.create_stellar_asset_contract(&asset_xdr);
        assert_valid_contract_address(&first_address);

        let second_address = client.create_stellar_asset_contract(&asset_xdr);
        assert_eq!(
            first_address, second_address,
            "Failed idempotent behavior for {}",
            description
        );

        let third_address = client.create_stellar_asset_contract(&asset_xdr);
        assert_eq!(
            first_address, third_address,
            "Failed idempotent behavior on third deployment for {}",
            description
        );
    }
}
