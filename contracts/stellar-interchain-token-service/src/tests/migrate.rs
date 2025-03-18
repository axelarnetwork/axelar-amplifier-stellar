use soroban_sdk::testutils::{Address as _, BytesN as _};
use soroban_sdk::{vec, Address, BytesN, String};
use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_std::interfaces::CustomMigratableInterface;
use stellar_axelar_std::{assert_auth, assert_err};

use crate::error::ContractError;
use crate::migrate::legacy_storage;
use crate::storage::{self, TokenIdConfigValue};
use crate::tests::utils::setup_env;
use crate::testutils::setup_its_token;
use crate::types::TokenManagerType;
use crate::InterchainTokenService;

const NEW_ITS_WASM: &[u8] =
    include_bytes!("testdata/stellar_interchain_token_service.optimized.wasm");
const NEW_TOKEN_MANAGER_WASM: &[u8] =
    include_bytes!("testdata/stellar_token_manager.optimized.wasm");
const NEW_INTERCHAIN_TOKEN_WASM: &[u8] =
    include_bytes!("testdata/stellar_interchain_token.optimized.wasm");

#[test]
fn migrate_succeeds() {
    let (env, client, _, _, _) = setup_env();
    let owner: Address = client.owner();
    let (token_id, _) = setup_its_token(&env, &client, &owner, 100);

    let new_its_wasm_hash = env.deployer().upload_contract_wasm(NEW_ITS_WASM);
    let new_token_manager_wasm_hash = env.deployer().upload_contract_wasm(NEW_TOKEN_MANAGER_WASM);
    let new_interchain_token_wasm_hash = env
        .deployer()
        .upload_contract_wasm(NEW_INTERCHAIN_TOKEN_WASM);

    let epoch = 123u64;

    let token_manager = client.token_manager_address(&token_id);

    let token_config = TokenIdConfigValue {
        token_address: Address::generate(&env),
        token_manager: token_manager.clone(),
        token_manager_type: TokenManagerType::LockUnlock,
    };

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    env.as_contract(&client.address, || {
        let flow_key = legacy_storage::FlowKey {
            token_id: token_id.clone(),
            epoch,
        };

        legacy_storage::set_flow_in(&env, flow_key.clone(), &flow_in_amount);
        legacy_storage::set_flow_out(&env, flow_key, &flow_out_amount);

        storage::set_token_id_config(&env, token_id.clone(), &token_config);
        storage::set_token_manager_wasm_hash(&env, &new_token_manager_wasm_hash);
        storage::set_interchain_token_wasm_hash(&env, &new_interchain_token_wasm_hash);
    });

    assert_auth!(owner, client.upgrade(&new_its_wasm_hash));

    let migration_data = (
        new_its_wasm_hash.clone(),
        vec![&env, (token_id.clone(), epoch)],
    );

    assert_auth!(owner, client.migrate(&migration_data));

    assert_eq!(
        env.as_contract(&client.address, || {
            storage::token_manager_wasm_hash(&env)
        }),
        new_its_wasm_hash,
        "token manager WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&client.address, || {
            storage::interchain_token_wasm_hash(&env)
        }),
        new_its_wasm_hash,
        "interchain token WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&client.address, || {
            storage::flow_in(&env, token_id.clone(), epoch)
        }),
        flow_in_amount,
        "flow in amount should be migrated correctly"
    );
    assert_eq!(
        env.as_contract(&client.address, || {
            storage::flow_out(&env, token_id.clone(), epoch)
        }),
        flow_out_amount,
        "flow out amount should be migrated correctly"
    );
}

#[test]
fn migrate_fails_with_invalid_token_id() {
    let (env, client, _, _, _) = setup_env();
    let owner = client.owner();

    let new_its_wasm_hash = env.deployer().upload_contract_wasm(NEW_ITS_WASM);

    let non_existent_token_id = BytesN::random(&env);
    let epoch = 123u64;

    assert_auth!(owner, client.upgrade(&new_its_wasm_hash));

    let token_ids_and_epochs = vec![&env, (non_existent_token_id.clone(), epoch)];
    let migration_data = (new_its_wasm_hash.clone(), token_ids_and_epochs);

    assert_err!(
        env.as_contract(&client.address, || {
            <InterchainTokenService as CustomMigratableInterface>::__migrate(&env, migration_data)
        }),
        ContractError::InvalidTokenId
    );
}

#[test]
fn migrate_fails_with_invalid_flow_key() {
    let (env, client, _, _, _) = setup_env();
    let owner = client.owner();
    let (token_id, _) = setup_its_token(&env, &client, &owner, 100);

    let new_its_wasm_hash = env.deployer().upload_contract_wasm(NEW_ITS_WASM);

    let epoch = 123u64;

    let token_manager = client.token_manager_address(&token_id);

    let token_config = TokenIdConfigValue {
        token_address: Address::generate(&env),
        token_manager: token_manager.clone(),
        token_manager_type: TokenManagerType::LockUnlock,
    };

    env.as_contract(&client.address, || {
        storage::set_token_id_config(&env, token_id.clone(), &token_config);
    });

    assert_auth!(owner, client.upgrade(&new_its_wasm_hash));

    let token_ids_and_epochs = vec![&env, (token_id.clone(), epoch)];
    let migration_data = (new_its_wasm_hash.clone(), token_ids_and_epochs);

    assert_err!(
        env.as_contract(&client.address, || {
            <InterchainTokenService as CustomMigratableInterface>::__migrate(&env, migration_data)
        }),
        ContractError::InvalidFlowKey
    );
}

#[test]
fn migrate_succeeds_with_multiple_token_ids() {
    let (env, client, _, _, _) = setup_env();
    let owner = client.owner();
    let (token_id_1, _) = setup_its_token(&env, &client, &owner, 100);
    let token_id_2 = client.mock_all_auths().deploy_interchain_token(
        &owner,
        &BytesN::random(&env),
        &TokenMetadata {
            name: String::from_str(&env, "Token2"),
            symbol: String::from_str(&env, "TOKEN2"),
            decimal: 6,
        },
        &200,
        &None,
    );

    let new_its_wasm_hash = env.deployer().upload_contract_wasm(NEW_ITS_WASM);
    let new_token_manager_wasm_hash = env.deployer().upload_contract_wasm(NEW_TOKEN_MANAGER_WASM);
    let new_interchain_token_wasm_hash = env
        .deployer()
        .upload_contract_wasm(NEW_INTERCHAIN_TOKEN_WASM);

    let epoch_1 = 123u64;
    let epoch_2 = 456u64;

    let token_manager_1 = client.token_manager_address(&token_id_1);
    let token_manager_2 = client.token_manager_address(&token_id_2);

    let token_config_1 = TokenIdConfigValue {
        token_address: Address::generate(&env),
        token_manager: token_manager_1.clone(),
        token_manager_type: TokenManagerType::LockUnlock,
    };

    let token_config_2 = TokenIdConfigValue {
        token_address: Address::generate(&env),
        token_manager: token_manager_2.clone(),
        token_manager_type: TokenManagerType::NativeInterchainToken,
    };

    let flow_in_amount_1 = 100i128;
    let flow_out_amount_1 = 50i128;
    let flow_in_amount_2 = 200i128;
    let flow_out_amount_2 = 150i128;

    env.as_contract(&client.address, || {
        let flow_key_1 = legacy_storage::FlowKey {
            token_id: token_id_1.clone(),
            epoch: epoch_1,
        };
        let flow_key_2 = legacy_storage::FlowKey {
            token_id: token_id_2.clone(),
            epoch: epoch_2,
        };

        legacy_storage::set_flow_in(&env, flow_key_1.clone(), &flow_in_amount_1);
        legacy_storage::set_flow_out(&env, flow_key_1, &flow_out_amount_1);
        legacy_storage::set_flow_in(&env, flow_key_2.clone(), &flow_in_amount_2);
        legacy_storage::set_flow_out(&env, flow_key_2, &flow_out_amount_2);

        storage::set_token_id_config(&env, token_id_1.clone(), &token_config_1);
        storage::set_token_id_config(&env, token_id_2.clone(), &token_config_2);
        storage::set_token_manager_wasm_hash(&env, &new_token_manager_wasm_hash);
        storage::set_interchain_token_wasm_hash(&env, &new_interchain_token_wasm_hash);
    });

    assert_auth!(owner, client.upgrade(&new_its_wasm_hash));

    let token_ids_and_epochs = vec![
        &env,
        (token_id_1.clone(), epoch_1),
        (token_id_2.clone(), epoch_2),
    ];
    let migration_data = (new_its_wasm_hash.clone(), token_ids_and_epochs);

    assert_auth!(owner, client.migrate(&migration_data));

    assert_eq!(
        env.as_contract(&client.address, || {
            storage::flow_in(&env, token_id_1.clone(), epoch_1)
        }),
        flow_in_amount_1
    );
    assert_eq!(
        env.as_contract(&client.address, || {
            storage::flow_out(&env, token_id_1.clone(), epoch_1)
        }),
        flow_out_amount_1
    );
    assert_eq!(
        env.as_contract(&client.address, || {
            storage::flow_in(&env, token_id_2.clone(), epoch_2)
        }),
        flow_in_amount_2
    );
    assert_eq!(
        env.as_contract(&client.address, || {
            storage::flow_out(&env, token_id_2.clone(), epoch_2)
        }),
        flow_out_amount_2
    );
}

#[test]
fn migrate_succeeds_with_empty_migration_data() {
    let (env, client, _, _, _) = setup_env();

    let owner = client.owner();

    let new_its_wasm_hash = env.deployer().upload_contract_wasm(NEW_ITS_WASM);

    assert_auth!(owner, client.upgrade(&new_its_wasm_hash));

    let token_ids_and_epochs = vec![&env];
    let migration_data = (new_its_wasm_hash.clone(), token_ids_and_epochs);

    assert_auth!(owner, client.migrate(&migration_data));
}
