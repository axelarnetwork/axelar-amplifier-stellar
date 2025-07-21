use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::{assert_auth, assert_auth_err, assert_contract_err, Address, BytesN};
use stellar_interchain_token::InterchainTokenClient;

use super::utils::{setup_env, TokenMetadataExt};
use crate::error::ContractError;

fn setup_token_and_minter(
    env: &stellar_axelar_std::Env,
    client: &crate::InterchainTokenServiceClient,
) -> (Address, Address) {
    let deployer = Address::generate(env);
    let salt = BytesN::<32>::from_array(env, &[1; 32]);
    let token_metadata = TokenMetadata::new(env, "Test", "TEST", 6);
    let initial_supply = 100;

    let token_id = client.mock_all_auths().deploy_interchain_token(
        &deployer,
        &salt,
        &token_metadata,
        &initial_supply,
        &None,
    );

    let token_address = client.registered_token_address(&token_id);
    let new_minter = Address::generate(env);

    (token_address, new_minter)
}

#[test]
fn add_minter_succeeds() {
    let (env, client, _, _, _) = setup_env();
    let (token_address, minter) = setup_token_and_minter(&env, &client);

    let token = InterchainTokenClient::new(&env, &token_address);

    assert!(!token.is_minter(&minter));

    assert_auth!(
        client.operator(),
        client.add_minter(&token_address, &minter)
    );

    assert!(token.is_minter(&minter));
}

#[test]
fn add_minter_succeeds_when_minter_already_exists() {
    let (env, client, _, _, _) = setup_env();
    let (token_address, minter) = setup_token_and_minter(&env, &client);

    let token = InterchainTokenClient::new(&env, &token_address);

    client.mock_all_auths().add_minter(&token_address, &minter);
    assert!(token.is_minter(&minter));

    assert_auth!(
        client.operator(),
        client.add_minter(&token_address, &minter)
    );

    assert!(token.is_minter(&minter));
}

#[test]
fn add_minter_fails_without_authorization() {
    let (env, client, _, _, _) = setup_env();
    let (token_address, minter) = setup_token_and_minter(&env, &client);

    let unauthorized_caller = Address::generate(&env);
    assert_auth_err!(
        unauthorized_caller,
        client.add_minter(&token_address, &minter)
    );

    assert_auth_err!(client.owner(), client.add_minter(&token_address, &minter));
}

#[test]
fn add_minter_fails_with_stellar_classic_asset() {
    let (env, client, _, _, _) = setup_env();

    let owner = Address::generate(&env);
    let asset = env.register_stellar_asset_contract_v2(owner);
    let token_address = asset.address();
    let minter = Address::generate(&env);

    assert_contract_err!(
        client
            .mock_all_auths()
            .try_add_minter(&token_address, &minter),
        ContractError::NotInterchainToken
    );
}

#[test]
fn add_minter_fails_with_invalid_token() {
    let (env, client, _, _, _) = setup_env();
    let invalid_token_address = Address::generate(&env);
    let minter = Address::generate(&env);

    assert_contract_err!(
        client
            .mock_all_auths()
            .try_add_minter(&invalid_token_address, &minter),
        ContractError::NotInterchainToken
    );
}
