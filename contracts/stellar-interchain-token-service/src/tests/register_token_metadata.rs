use stellar_axelar_gas_service::testutils::setup_gas_token;
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::{assert_auth_err, assert_contract_err, events, Address, String};

use super::utils::setup_env;
use crate::error::ContractError;
use crate::event::TokenMetadataRegisteredEvent;

pub const REGISTER_TOKEN_METADATA_EVENT_IDX: i32 = -1;

#[test]
fn register_token_metadata_succeeds() {
    let (env, client, _, _, _) = setup_env();
    let owner = Address::generate(&env);
    let token = &env.register_stellar_asset_contract_v2(owner);
    let spender = Address::generate(&env);
    let gas_token = setup_gas_token(&env, &spender);

    let its_hub_chain = String::from_str(&env, "axelar");

    client.mock_all_auths().set_trusted_chain(&its_hub_chain);

    client
        .mock_all_auths()
        .register_token_metadata(&token.address(), &spender, &Some(gas_token));

    goldie::assert!(events::fmt_emitted_event_at_idx::<
        TokenMetadataRegisteredEvent,
    >(&env, REGISTER_TOKEN_METADATA_EVENT_IDX));
}

#[test]
fn register_token_metadata_fails_when_paused() {
    let (env, client, _, _, _) = setup_env();
    let owner = Address::generate(&env);
    let token = &env.register_stellar_asset_contract_v2(owner);
    let spender = Address::generate(&env);
    let gas_token = setup_gas_token(&env, &spender);

    client.mock_all_auths().pause();

    assert_contract_err!(
        client.try_register_token_metadata(&token.address(), &spender, &Some(gas_token)),
        ContractError::ContractPaused
    );
}

#[test]
fn register_token_metadata_fails_with_invalid_token() {
    let (env, client, _, _, _) = setup_env();
    let spender = Address::generate(&env);
    let gas_token = setup_gas_token(&env, &spender);
    let token_address = Address::generate(&env);

    assert_contract_err!(
        client.mock_all_auths().try_register_token_metadata(
            &token_address,
            &spender,
            &Some(gas_token)
        ),
        ContractError::InvalidTokenAddress
    );
}

#[test]
fn register_token_metadata_fails_with_unauthorized() {
    let (env, client, _, _, _) = setup_env();
    let owner = Address::generate(&env);
    let token = &env.register_stellar_asset_contract_v2(owner.clone());
    let spender = Address::generate(&env);
    let gas_token = setup_gas_token(&env, &spender);

    assert_auth_err!(
        spender,
        client.register_token_metadata(&token.address(), &spender, &Some(gas_token.clone()))
    );
}
