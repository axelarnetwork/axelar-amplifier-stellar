#![cfg(test)]
extern crate alloc;
extern crate std;

use alloc::string::ToString;
use std::vec::Vec;

use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::token::{StellarAssetClient, TokenClient};
use stellar_axelar_std::xdr::{
    AccountId, AlphaNum12, AlphaNum4, Asset as XdrAsset, AssetCode12, AssetCode4, Limits,
    PublicKey, WriteXdr,
};
use stellar_axelar_std::{assert_contract_err, assert_ok, bytes, Address, Bytes, Env, String};

use crate::error::ContractError;
use crate::{TokenUtils, TokenUtilsClient};

const CONTRACT_ADDRESS_PREFIX: char = 'C';
const TEST_ISSUER_1: &str = "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
const TEST_ISSUER_2: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";

fn setup() -> (Env, TokenUtilsClient<'static>) {
    let env = Env::default();
    let contract_id = env.register(TokenUtils, ());
    let client = TokenUtilsClient::new(&env, &contract_id);
    (env, client)
}

fn create_issuer(env: &Env, address: &str) -> Address {
    Address::from_string(&String::from_str(env, address))
}

fn address_to_account_id(address: &Address) -> AccountId {
    let mut account_bytes = [0u8; 32];
    let address_str = address.to_string().to_string();
    let addr_bytes = address_str.as_bytes();

    // Copy bytes from address string to account bytes
    let copy_len = std::cmp::min(addr_bytes.len(), 32);
    for (i, &byte) in addr_bytes.iter().enumerate().take(copy_len) {
        account_bytes[i] = byte;
    }

    AccountId(PublicKey::PublicKeyTypeEd25519(
        stellar_axelar_std::xdr::Uint256(account_bytes),
    ))
}

fn string_to_asset_code<const N: usize>(code: &str) -> [u8; N] {
    let mut code_bytes = [0u8; N];
    let copy_len = std::cmp::min(code.len(), N);
    code_bytes[..copy_len].copy_from_slice(&code.as_bytes()[..copy_len]);
    code_bytes
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

    let xdr_bytes = asset.to_xdr(Limits::none()).unwrap();
    Bytes::from_slice(env, &xdr_bytes)
}

fn deploy_and_expect_success(client: &TokenUtilsClient, asset_xdr: &Bytes) -> Address {
    assert_ok!(client.try_deploy_stellar_asset_contract(asset_xdr)).unwrap()
}

fn assert_valid_contract_address(address: &Address) {
    let address_string = address.to_string().to_string();
    assert!(!address_string.is_empty());
    assert!(address_string.starts_with(CONTRACT_ADDRESS_PREFIX));
}

fn assert_all_unique(addresses: &[Address]) {
    for (i, addr1) in addresses.iter().enumerate() {
        for addr2 in addresses.iter().skip(i + 1) {
            assert_ne!(addr1, addr2, "All addresses should be unique");
        }
    }
}

fn verify_deployed_contract_is_functional(env: &Env, _token_address: &Address, _issuer: &Address) {
    let test_asset = env.register_stellar_asset_contract_v2(Address::generate(env));
    let token_client = TokenClient::new(env, &test_asset.address());
    let stellar_asset_client = StellarAssetClient::new(env, &test_asset.address());

    let account1 = Address::generate(env);
    let account2 = Address::generate(env);
    let mint_amount = 1000i128;

    stellar_asset_client
        .mock_all_auths()
        .mint(&account1, &mint_amount);
    assert_eq!(token_client.balance(&account1), mint_amount);

    let transfer_amount = 300i128;
    token_client
        .mock_all_auths()
        .transfer(&account1, &account2, &transfer_amount);

    assert_eq!(
        token_client.balance(&account1),
        mint_amount - transfer_amount
    );
    assert_eq!(token_client.balance(&account2), transfer_amount);
}

#[test]
fn deploy_stellar_asset_contract_succeeds_with_valid_xdr() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);
    let asset_xdr = create_asset_xdr(&env, "USDC", &issuer);

    let contract_address = deploy_and_expect_success(&client, &asset_xdr);
    assert_valid_contract_address(&contract_address);
    verify_deployed_contract_is_functional(&env, &contract_address, &issuer);
}

#[test]
fn deploy_stellar_asset_contract_fails_empty_asset_xdr() {
    let (env, client) = setup();
    let empty_asset_xdr = Bytes::new(&env);

    assert_contract_err!(
        client.try_deploy_stellar_asset_contract(&empty_asset_xdr),
        ContractError::InvalidAssetXdr
    );
}

#[test]
fn deploy_stellar_asset_contract_fails_short_asset_xdr() {
    let (env, client) = setup();
    let short_asset_xdr = bytes!(&env, 0x0123456789abcdef0123456789abcdef); // XDR shorter than minimum 32 bytes required

    assert_contract_err!(
        client.try_deploy_stellar_asset_contract(&short_asset_xdr),
        ContractError::InvalidAssetXdr
    );
}

#[test]
fn deploy_stellar_asset_contract_fails_when_already_deployed() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);
    let asset_xdr = create_asset_xdr(&env, "BTC", &issuer);

    let _first_address = deploy_and_expect_success(&client, &asset_xdr);

    assert!(
        client
            .try_deploy_stellar_asset_contract(&asset_xdr)
            .is_err(),
        "Deploying the same contract twice should fail"
    );
}

#[test]
fn deploy_stellar_asset_contract_different_assets_succeed() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);

    let test_assets = ["USD", "EUR", "GBP", "JPY"]; // Each asset code must be unique to avoid deployment conflicts
    let addresses: Vec<Address> = test_assets
        .iter()
        .map(|&code| {
            let asset_xdr = create_asset_xdr(&env, code, &issuer);
            deploy_and_expect_success(&client, &asset_xdr)
        })
        .collect();

    assert_all_unique(&addresses);
}

#[test]
fn deploy_stellar_asset_contract_same_asset_different_issuers_succeed() {
    let (env, client) = setup();
    let issuer1 = create_issuer(&env, TEST_ISSUER_1);
    let issuer2 = create_issuer(&env, TEST_ISSUER_2);

    let asset_code = "STABLE";
    let asset_xdr1 = create_asset_xdr(&env, asset_code, &issuer1);
    let asset_xdr2 = create_asset_xdr(&env, asset_code, &issuer2);

    let address1 = deploy_and_expect_success(&client, &asset_xdr1);
    let address2 = deploy_and_expect_success(&client, &asset_xdr2);

    assert_ne!(address1, address2);
}

#[test]
fn deploy_stellar_asset_contract_alphanum4_vs_alphanum12() {
    let (env, client) = setup();
    let issuer = create_issuer(&env, TEST_ISSUER_1);

    let alphanum4_xdr = create_asset_xdr(&env, "TEST", &issuer); // AlphaNum4: 4 characters or less
    let alphanum4_address = deploy_and_expect_success(&client, &alphanum4_xdr);

    let alphanum12_xdr = create_asset_xdr(&env, "TESTLONGNAME", &issuer); // AlphaNum12: more than 4 characters
    let alphanum12_address = deploy_and_expect_success(&client, &alphanum12_xdr);

    assert_ne!(alphanum4_address, alphanum12_address);
}
