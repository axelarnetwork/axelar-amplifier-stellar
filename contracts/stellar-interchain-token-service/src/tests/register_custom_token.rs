use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::{assert_auth, assert_contract_err, events, Address, BytesN};

use super::utils::setup_env;
use crate::error::ContractError;
use crate::event::TokenManagerDeployedEvent;
use crate::types::TokenManagerType;

const TEST_SALT: [u8; 32] = [1; 32];

struct RegisterCustomTokenTestData {
    deployer: Address,
    token: stellar_axelar_std::testutils::StellarAssetContract,
    salt: BytesN<32>,
}

fn setup_register_custom_token_test_data(
    env: &stellar_axelar_std::Env,
) -> RegisterCustomTokenTestData {
    let deployer = Address::generate(env);
    let owner = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(owner);
    let salt = BytesN::<32>::from_array(env, &TEST_SALT);

    RegisterCustomTokenTestData {
        deployer,
        token,
        salt,
    }
}

#[test]
fn register_custom_token_succeeds_with_token_manager_type_lock_unlock() {
    let (env, client, _, _, _) = setup_env();
    let test_data = setup_register_custom_token_test_data(&env);
    let RegisterCustomTokenTestData {
        deployer,
        token,
        salt,
    } = test_data;
    let token_manager_type = TokenManagerType::LockUnlock;
    let expected_id = client.linked_token_id(&deployer, &salt);

    let token_id = assert_auth!(
        &deployer,
        client.register_custom_token(&deployer, &salt, &token.address(), &token_manager_type)
    );

    let token_manager_deployed_event =
        events::fmt_emitted_event_at_idx::<TokenManagerDeployedEvent>(&env, -2);

    assert_eq!(token_id, expected_id);

    assert_eq!(
        client.registered_token_address(&expected_id),
        token.address()
    );
    assert_eq!(client.token_manager_type(&expected_id), token_manager_type);

    goldie::assert!(token_manager_deployed_event);
}

#[test]
fn register_custom_token_succeeds_with_token_manager_type_mint_burn() {
    let (env, client, _, _, _) = setup_env();
    let test_data = setup_register_custom_token_test_data(&env);
    let RegisterCustomTokenTestData {
        deployer,
        token,
        salt,
    } = test_data;
    let token_manager_type = TokenManagerType::MintBurn;
    let expected_id = client.linked_token_id(&deployer, &salt);

    let token_id = assert_auth!(
        &deployer,
        client.register_custom_token(&deployer, &salt, &token.address(), &token_manager_type)
    );

    let token_manager_deployed_event =
        events::fmt_last_emitted_event::<TokenManagerDeployedEvent>(&env);

    assert_eq!(token_id, expected_id);

    assert_eq!(
        client.registered_token_address(&expected_id),
        token.address()
    );
    assert_eq!(client.token_manager_type(&expected_id), token_manager_type);

    goldie::assert!(token_manager_deployed_event);
}

#[test]
fn register_custom_token_fails_with_token_manager_type_native_interchain_token() {
    let (env, client, _, _, _) = setup_env();
    let test_data = setup_register_custom_token_test_data(&env);
    let RegisterCustomTokenTestData {
        deployer,
        token,
        salt,
    } = test_data;
    let token_manager_type = TokenManagerType::NativeInterchainToken;

    assert_contract_err!(
        client.mock_all_auths().try_register_custom_token(
            &deployer,
            &salt,
            &token.address(),
            &token_manager_type
        ),
        ContractError::InvalidTokenManagerType
    );
}

#[test]
fn register_custom_token_fails_when_paused() {
    let (env, client, _, _, _) = setup_env();
    let deployer = Address::generate(&env);

    client.mock_all_auths().pause();

    assert_contract_err!(
        client.try_register_custom_token(
            &deployer,
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
    let test_data = setup_register_custom_token_test_data(&env);
    let RegisterCustomTokenTestData {
        deployer,
        token,
        salt,
    } = test_data;
    let token_manager_type = TokenManagerType::LockUnlock;

    client.mock_all_auths().register_custom_token(
        &deployer,
        &salt,
        &token.address(),
        &token_manager_type,
    );

    assert_contract_err!(
        client.mock_all_auths().try_register_custom_token(
            &deployer,
            &salt,
            &token.address(),
            &token_manager_type
        ),
        ContractError::TokenAlreadyRegistered
    );
}

#[test]
fn custom_token_id_derivation() {
    let (env, client, _, _, _) = setup_env();
    let deployer = Address::generate(&env);
    let salt = BytesN::<32>::from_array(&env, &[1; 32]);

    let token_id = client.linked_token_id(&deployer, &salt);

    goldie::assert!(hex::encode(token_id.to_array()));
}

#[test]
fn register_custom_token_fails_with_native_interchain_token_type() {
    let (env, client, _, _, _) = setup_env();
    let test_data = setup_register_custom_token_test_data(&env);
    let RegisterCustomTokenTestData {
        deployer,
        token,
        salt,
    } = test_data;
    let token_manager_type = TokenManagerType::NativeInterchainToken;

    assert_contract_err!(
        client.mock_all_auths().try_register_custom_token(
            &deployer,
            &salt,
            &token.address(),
            &token_manager_type
        ),
        ContractError::InvalidTokenManagerType
    );
}

#[test]
fn register_custom_token_fails_with_invalid_token_address() {
    let (env, client, _, _, _) = setup_env();
    let invalid_token_address = Address::generate(&env);
    let test_data = setup_register_custom_token_test_data(&env);
    let RegisterCustomTokenTestData {
        deployer,
        token: _,
        salt,
    } = test_data;
    let token_manager_type = TokenManagerType::LockUnlock;

    assert_contract_err!(
        client.mock_all_auths().try_register_custom_token(
            &deployer,
            &salt,
            &invalid_token_address,
            &token_manager_type
        ),
        ContractError::InvalidTokenAddress
    );
}
