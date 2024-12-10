#[allow(dead_code)]
mod utils;
use utils::setup_env;

use axelar_soroban_std::{assert_contract_err, assert_invoke_auth_err, assert_last_emitted_event};
use interchain_token_service::error::ContractError;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, String, Symbol};

#[test]
fn set_trusted_address() {
    let (env, client, _, _) = setup_env();
    env.mock_all_auths();

    let chain = String::from_str(&env, "chain");
    client.set_trusted_chain(&chain);

    assert_last_emitted_event(
        &env,
        &client.address,
        (Symbol::new(&env, "trusted_chain_set"), chain.clone()),
        (),
    );

    assert_eq!(client.trusted_chain(&chain), true);
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

    client.remove_trusted_chain(&chain);

    client.set_trusted_chain(&chain);
}

#[test]
fn remove_trusted_chain() {
    let (env, client, _, _) = setup_env();
    env.mock_all_auths();

    let chain = String::from_str(&env, "chain");
    client.set_trusted_chain(&chain);

    client.remove_trusted_chain(&chain);

    assert_last_emitted_event(
        &env,
        &client.address,
        (Symbol::new(&env, "trusted_chain_removed"), chain.clone()),
        (),
    );

    assert_eq!(client.trusted_chain(&chain), false);
}

#[test]
fn remove_trusted_chain_fails_if_not_set() {
    let (env, client, _, _) = setup_env();
    env.mock_all_auths();

    let chain = String::from_str(&env, "chain");

    assert_eq!(client.trusted_chain(&chain), false);

    assert_contract_err!(
        client.try_remove_trusted_chain(&chain),
        ContractError::TrustedChainNotSet
    );
}
