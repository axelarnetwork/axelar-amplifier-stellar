#[allow(dead_code)]
mod utils;
use interchain_token_service::event::{TrustedChainRemovedEvent, TrustedChainSetEvent};
use utils::setup_env;

use axelar_soroban_std::{
    assert_contract_err, assert_invoke_auth_err, assert_invoke_auth_ok, events,
};
use interchain_token_service::error::ContractError;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, String};

#[test]
fn set_trusted_address() {
    let (env, client, _, _) = setup_env();

    let chain = String::from_str(&env, "chain");

    assert_invoke_auth_ok!(client.owner(), client.try_set_trusted_chain(&chain));

    goldie::assert!(events::fmt_last_emitted_event::<TrustedChainSetEvent>(&env));

    assert!(client.is_trusted_chain(&chain));
}

#[test]
fn set_trusted_chain_fails_if_not_owner() {
    let (env, client, _, _) = setup_env();

    let not_owner = Address::generate(&env);
    let chain = String::from_str(&env, "chain");

    assert_invoke_auth_err!(not_owner, client.try_set_trusted_chain(&chain));
}

#[test]
fn set_trusted_chain_fails_if_already_set() {
    let (env, client, _, _) = setup_env();
    env.mock_all_auths();

    let chain = String::from_str(&env, "chain");
    client.set_trusted_chain(&chain);

    assert_contract_err!(
        client.try_set_trusted_chain(&chain),
        ContractError::TrustedChainAlreadySet
    );
}

#[test]
fn remove_trusted_chain() {
    let (env, client, _, _) = setup_env();

    let chain = String::from_str(&env, "chain");

    assert_invoke_auth_ok!(client.owner(), client.try_set_trusted_chain(&chain));

    assert_invoke_auth_ok!(client.owner(), client.try_remove_trusted_chain(&chain));

    goldie::assert!(events::fmt_last_emitted_event::<TrustedChainRemovedEvent>(
        &env
    ));

    assert!(!client.is_trusted_chain(&chain));
}

#[test]
fn remove_trusted_chain_fails_if_not_set() {
    let (env, client, _, _) = setup_env();
    env.mock_all_auths();

    let chain = String::from_str(&env, "chain");

    assert!(!client.is_trusted_chain(&chain));

    assert_contract_err!(
        client.try_remove_trusted_chain(&chain),
        ContractError::TrustedChainNotSet
    );
}
