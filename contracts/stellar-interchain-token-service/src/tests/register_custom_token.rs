use stellar_axelar_std::address::AddressExt;
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::{assert_contract_err, events, Address, BytesN};

use super::utils::setup_env;
use crate::error::ContractError;
use crate::event::TokenManagerDeployedEvent;
use crate::types::TokenManagerType;

pub const TOKEN_MANAGER_DEPLOYED_EVENT_IDX: i32 = -1;

#[test]
fn register_custom_token_succeeds() {
    let (env, client, _, _, _) = setup_env();
    let owner = Address::generate(&env);
    let token = &env.register_stellar_asset_contract_v2(owner);
    let salt = BytesN::<32>::from_array(&env, &[1; 32]);
    let token_manager_type = TokenManagerType::LockUnlock;
    let expected_id = client.interchain_token_id(&Address::zero(&env), &salt);

    assert_eq!(
        client
            .mock_all_auths()
            .register_custom_token(&salt, &token.address(), &token_manager_type),
        expected_id
    );
    let token_manager_deployed_event = events::fmt_emitted_event_at_idx::<TokenManagerDeployedEvent>(
        &env,
        TOKEN_MANAGER_DEPLOYED_EVENT_IDX,
    );

    assert_eq!(
        client.registered_token_address(&expected_id),
        token.address()
    );
    assert_eq!(client.token_manager_type(&expected_id), token_manager_type);
    goldie::assert!(token_manager_deployed_event);
}

#[test]
fn register_custom_token_fails_when_paused() {
    let (env, client, _, _, _) = setup_env();

    client.mock_all_auths().pause();

    assert_contract_err!(
        client.try_register_custom_token(
            &BytesN::<32>::from_array(&env, &[1; 32]),
            &Address::generate(&env),
            &TokenManagerType::LockUnlock
        ),
        ContractError::ContractPaused
    );
}

#[test]
fn register_custom_token_fails_if_already_registered() {
    let (env, client, _, _, _) = setup_env();
    let owner = Address::generate(&env);
    let token = &env.register_stellar_asset_contract_v2(owner);
    let salt = BytesN::<32>::from_array(&env, &[1; 32]);
    let token_manager_type = TokenManagerType::LockUnlock;

    client.register_custom_token(&salt, &token.address(), &token_manager_type);

    assert_contract_err!(
        client.try_register_custom_token(&salt, &token.address(), &token_manager_type),
        ContractError::TokenAlreadyRegistered
    );
}

#[test]
fn custom_token_id_derivation() {
    let (env, client, _, _, _) = setup_env();
    let salt = BytesN::<32>::from_array(&env, &[1; 32]);

    let token_id = client.interchain_token_id(&Address::zero(&env), &salt);

    goldie::assert!(hex::encode(token_id.to_array()));
}

#[test]
fn register_custom_token_fails_with_native_interchain_token_type() {
    let (env, client, _, _, _) = setup_env();
    let owner = Address::generate(&env);
    let token = &env.register_stellar_asset_contract_v2(owner);
    let salt = BytesN::<32>::from_array(&env, &[1; 32]);
    let token_manager_type = TokenManagerType::NativeInterchainToken;

    assert_contract_err!(
        client.try_register_custom_token(&salt, &token.address(), &token_manager_type),
        ContractError::InvalidTokenManagerType
    );
}
