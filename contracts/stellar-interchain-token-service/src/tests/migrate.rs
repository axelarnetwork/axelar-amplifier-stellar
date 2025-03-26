use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_std::testutils::BytesN as _;
use stellar_axelar_std::{assert_contract_err, log, mock_auth, vec, BytesN, Env, IntoVal, String};
use stellar_interchain_token::InterchainTokenClient;
use stellar_token_manager::TokenManagerClient;

use crate::error::ContractError;
use crate::migrate::legacy_storage;
use crate::storage::{self, TokenIdConfigValue};
use crate::tests::utils::{
    assert_migrate_storage, setup_migrate_env, setup_migrate_storage, MigrateTestConfig,
};
use crate::types::TokenManagerType;

const NEW_VERSION: &str = "1.1.0";

#[test]
fn migrate_native_interchain_token_succeeds() {
    let MigrateTestConfig {
        env,
        owner,
        its_client,
        upgrader_client,
        token_id,
        current_epoch,
        its_wasm_hash,
        token_manager,
        interchain_token,
        migration_data,
        ..
    } = setup_migrate_env();

    let token_config = TokenIdConfigValue {
        token_address: interchain_token.clone(),
        token_manager: token_manager.clone(),
        token_manager_type: TokenManagerType::NativeInterchainToken,
    };

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    env.as_contract(&its_client.address, || {
        let flow_key = legacy_storage::FlowKey {
            token_id: token_id.clone(),
            epoch: current_epoch,
        };

        legacy_storage::set_flow_in(&env, flow_key.clone(), &flow_in_amount);
        legacy_storage::set_flow_out(&env, flow_key, &flow_out_amount);

        storage::set_token_id_config(&env, token_id.clone(), &token_config);
    });

    let its_upgrade_auth = mock_auth!(owner, its_client.upgrade(&its_wasm_hash));
    let its_migrate_auth = mock_auth!(owner, its_client.migrate(migration_data.clone()));
    let upgrader_upgrade_auth = mock_auth!(
        owner,
        upgrader_client.upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.clone()],
        ),
        &[
            its_upgrade_auth.invoke.clone(),
            its_migrate_auth.invoke.clone()
        ]
    );

    log!(&env, "OWNER", owner); // TODO: Remove
    log!(&env, "ITS_CLIENT ADDRESS", its_client.address); // TODO: Remove
    log!(&env, "UPGRADE_CLIENT ADDRESS", upgrader_client.address); // TODO: Remove
    log!(&env, "TOKEN_MANAGER", token_manager); // TODO: Remove
    log!(&env, "INTERCHAIN_TOKEN", interchain_token); // TODO: Remove
    log!( // TODO: Remove
        &env,
        "MIGRATION_DATA.NEW_TOKEN_MANAGER_WASM_HASH",
        migration_data.new_token_manager_wasm_hash
    );
    log!( // TODO: Remove
        &env,
        "MIGRATION_DATA.NEW_INTERCHAIN_TOKEN_WASM_HASH",
        migration_data.new_interchain_token_wasm_hash
    );

    log!( // TODO: Remove
        &env,
        "----------CALLING: UPGRADER.UPGRADE----------",
        upgrader_client.address
    );
    upgrader_client
        .mock_auths(&[upgrader_upgrade_auth])
        .upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.into_val(&env)],
        );
    log!( // TODO: Remove
        &env,
        "----------CALLED: UPGRADER.UPGRADE----------",
        upgrader_client.address
    );

    let token_manager_client = TokenManagerClient::new(&env, &token_manager);
    let interchain_token_client = InterchainTokenClient::new(&env, &interchain_token);

    let token_manager_upgrade_auth = mock_auth!(
        its_client.address,
        token_manager_client.upgrade(&migration_data.new_token_manager_wasm_hash)
    );
    let interchain_token_upgrade_auth = mock_auth!(
        its_client.address,
        interchain_token_client.upgrade(&migration_data.new_interchain_token_wasm_hash)
    );
    let token_manager_migrate_auth = mock_auth!(
        its_client.address,
        token_manager_client.migrate(&vec![&env, ()])
    );
    let interchain_token_migrate_auth = mock_auth!(
        its_client.address,
        interchain_token_client.migrate(&vec![&env, ()])
    );
    let upgrader_upgrade_tm_auth = mock_auth!(
        its_client.address,
        upgrader_client.upgrade(
            &token_manager,
            &String::from_str(&env, NEW_VERSION),
            &migration_data.new_token_manager_wasm_hash,
            &vec![&env, ()],
        ),
        &[
            token_manager_upgrade_auth.invoke.clone(),
            token_manager_migrate_auth.invoke.clone()
        ]
    );
    let upgrader_upgrade_it_auth = mock_auth!(
        its_client.address,
        upgrader_client.upgrade(
            &interchain_token,
            &String::from_str(&env, NEW_VERSION),
            &migration_data.new_interchain_token_wasm_hash,
            &vec![&env, ()],
        ),
        &[
            interchain_token_upgrade_auth.invoke.clone(),
            interchain_token_migrate_auth.invoke.clone()
        ]
    );
    let its_migrate_token_auth = mock_auth!(
        owner,
        its_client.migrate_token(
            &token_id,
            &upgrader_client.address,
            &String::from_str(&env, NEW_VERSION),
        ),
        &[
            upgrader_upgrade_tm_auth.invoke.clone(),
            upgrader_upgrade_it_auth.invoke.clone()
        ]
    );

    log!(&env, "----------CALLING: ITS.MIGRATE_TOKEN----------"); // TODO: Remove
    its_client
        // .mock_auths(&[its_migrate_token_auth]) // FIXME: Use
        .mock_all_auths_allowing_non_root_auth()
        .migrate_token(
            &token_id,
            &upgrader_client.address,
            &String::from_str(&env, NEW_VERSION),
        );
    log!(&env, "----------CALLED: ITS.MIGRATE_TOKEN----------"); // TODO: Remove

    assert_migrate_storage(
        &env,
        &its_client,
        migration_data,
        token_id,
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );
}

#[test]
fn migrate_lock_unlock_succeeds() {
    let MigrateTestConfig {
        env,
        owner,
        its_client,
        upgrader_client,
        token_id,
        current_epoch,
        its_wasm_hash,
        token_manager,
        interchain_token,
        migration_data,
        ..
    } = setup_migrate_env();

    let token_config = TokenIdConfigValue {
        token_address: interchain_token,
        token_manager,
        token_manager_type: TokenManagerType::LockUnlock,
    };

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    setup_migrate_storage(
        &env,
        token_config,
        &its_client,
        token_id.clone(),
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );

    let its_upgrade_auth = mock_auth!(owner, its_client.upgrade(&its_wasm_hash));
    let its_migrate_auth = mock_auth!(owner, its_client.migrate(migration_data.clone()));
    let upgrader_upgrade_auth = mock_auth!(
        owner,
        upgrader_client.upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.clone()],
        ),
        &[
            its_upgrade_auth.invoke.clone(),
            its_migrate_auth.invoke.clone()
        ]
    );

    upgrader_client
        .mock_auths(&[upgrader_upgrade_auth])
        .upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.into_val(&env)],
        );

    its_client
        // .mock_auths(&[its_migrate_token_auth]) // FIXME: Use
        .mock_all_auths_allowing_non_root_auth()
        .migrate_token(
            &token_id,
            &upgrader_client.address,
            &String::from_str(&env, NEW_VERSION),
        );

    assert_migrate_storage(
        &env,
        &its_client,
        migration_data,
        token_id,
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );
}

#[test]
fn migrate_token_fails_with_invalid_token_id() {
    let MigrateTestConfig {
        env,
        owner,
        its_client,
        upgrader_client,
        its_wasm_hash,
        migration_data,
        ..
    } = setup_migrate_env();

    let non_existent_token_id = BytesN::random(&env);

    let its_upgrade_auth = mock_auth!(owner, its_client.upgrade(&its_wasm_hash));
    let its_migrate_auth = mock_auth!(owner, its_client.migrate(migration_data.clone()));
    let upgrader_upgrade_auth = mock_auth!(
        owner,
        upgrader_client.upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.clone()],
        ),
        &[
            its_upgrade_auth.invoke.clone(),
            its_migrate_auth.invoke.clone()
        ]
    );

    upgrader_client
        .mock_auths(&[upgrader_upgrade_auth])
        .upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.into_val(&env)],
        );

    assert_contract_err!(
        its_client.mock_all_auths().try_migrate_token(
            &non_existent_token_id,
            &upgrader_client.address,
            &String::from_str(&env, NEW_VERSION),
        ),
        ContractError::InvalidTokenId
    );
}

#[test]
fn migrate_succeeds_with_multiple_token_ids() {
    let MigrateTestConfig {
        env,
        owner,
        its_client,
        upgrader_client,
        token_id: token_id_1,
        current_epoch,
        its_wasm_hash,
        token_manager: token_manager_1,
        interchain_token: interchain_token_1,
        migration_data,
        ..
    } = setup_migrate_env();

    let token_id_2 = its_client.mock_all_auths().deploy_interchain_token(
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

    let token_manager_2 = its_client.deployed_token_manager(&token_id_2);
    let interchain_token_2 = its_client.interchain_token_address(&token_id_2);

    let token_config_1 = TokenIdConfigValue {
        token_address: interchain_token_1,
        token_manager: token_manager_1,
        token_manager_type: TokenManagerType::LockUnlock,
    };

    let token_config_2 = TokenIdConfigValue {
        token_address: interchain_token_2,
        token_manager: token_manager_2,
        token_manager_type: TokenManagerType::NativeInterchainToken,
    };

    let flow_in_amount_1 = 100i128;
    let flow_out_amount_1 = 50i128;
    let flow_in_amount_2 = 200i128;
    let flow_out_amount_2 = 150i128;

    setup_migrate_storage(
        &env,
        token_config_1,
        &its_client,
        token_id_1.clone(),
        current_epoch,
        flow_in_amount_1,
        flow_out_amount_1,
    );

    setup_migrate_storage(
        &env,
        token_config_2,
        &its_client,
        token_id_2.clone(),
        current_epoch,
        flow_in_amount_2,
        flow_out_amount_2,
    );

    let its_upgrade_auth = mock_auth!(owner, its_client.upgrade(&its_wasm_hash));
    let its_migrate_auth = mock_auth!(owner, its_client.migrate(migration_data.clone()));
    let upgrader_upgrade_auth = mock_auth!(
        owner,
        upgrader_client.upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.clone()],
        ),
        &[
            its_upgrade_auth.invoke.clone(),
            its_migrate_auth.invoke.clone()
        ]
    );

    upgrader_client
        .mock_auths(&[upgrader_upgrade_auth])
        .upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.into_val(&env)],
        );

    its_client
        // .mock_auths(&[its_migrate_token_auth]) // FIXME: Use
        .mock_all_auths_allowing_non_root_auth()
        .migrate_token(
            &token_id_1,
            &upgrader_client.address,
            &String::from_str(&env, NEW_VERSION),
        );

    its_client
        // .mock_auths(&[its_migrate_token_auth]) // FIXME: Use
        .mock_all_auths_allowing_non_root_auth()
        .migrate_token(
            &token_id_2,
            &upgrader_client.address,
            &String::from_str(&env, NEW_VERSION),
        );

    assert_migrate_storage(
        &env,
        &its_client,
        migration_data.clone(),
        token_id_1,
        current_epoch,
        flow_in_amount_1,
        flow_out_amount_1,
    );
    assert_migrate_storage(
        &env,
        &its_client,
        migration_data,
        token_id_2,
        current_epoch,
        flow_in_amount_2,
        flow_out_amount_2,
    );
}

#[test]
fn migrate_succeeds_with_empty_migration_data() {
    let MigrateTestConfig {
        env,
        owner,
        its_client,
        upgrader_client,
        its_wasm_hash,
        migration_data,
        ..
    } = setup_migrate_env();

    let its_upgrade_auth = mock_auth!(owner, its_client.upgrade(&its_wasm_hash));
    let its_migrate_auth = mock_auth!(owner, its_client.migrate(migration_data.clone()));
    let upgrader_upgrade_auth = mock_auth!(
        owner,
        upgrader_client.upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.clone()],
        ),
        &[
            its_upgrade_auth.invoke.clone(),
            its_migrate_auth.invoke.clone()
        ]
    );

    upgrader_client
        .mock_auths(&[upgrader_upgrade_auth])
        .upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.into_val(&env)],
        );
}

#[test]
fn migrate_with_native_interchain_token_legacy_flow_data() {
    let MigrateTestConfig {
        env,
        owner,
        its_client,
        upgrader_client,
        token_id,
        current_epoch,
        its_wasm_hash,
        token_manager,
        interchain_token,
        migration_data,
        ..
    } = setup_migrate_env();

    let token_config = TokenIdConfigValue {
        token_address: interchain_token,
        token_manager,
        token_manager_type: TokenManagerType::NativeInterchainToken,
    };

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    setup_migrate_storage(
        &env,
        token_config,
        &its_client,
        token_id.clone(),
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );

    let its_upgrade_auth = mock_auth!(owner, its_client.upgrade(&its_wasm_hash));
    let its_migrate_auth = mock_auth!(owner, its_client.migrate(migration_data.clone()));
    let upgrader_upgrade_auth = mock_auth!(
        owner,
        upgrader_client.upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.clone()],
        ),
        &[
            its_upgrade_auth.invoke.clone(),
            its_migrate_auth.invoke.clone()
        ]
    );

    upgrader_client
        .mock_auths(&[upgrader_upgrade_auth])
        .upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.into_val(&env)],
        );

    its_client
        // .mock_auths(&[its_migrate_token_auth]) // FIXME: Use
        .mock_all_auths_allowing_non_root_auth()
        .migrate_token(
            &token_id,
            &upgrader_client.address,
            &String::from_str(&env, NEW_VERSION),
        );

    assert_migrate_storage(
        &env,
        &its_client,
        migration_data,
        token_id,
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );
}

#[test]
fn migrate_with_lock_unlock_legacy_flow_data() {
    let MigrateTestConfig {
        env,
        owner,
        its_client,
        upgrader_client,
        token_id,
        current_epoch,
        its_wasm_hash,
        token_manager,
        interchain_token,
        migration_data,
        ..
    } = setup_migrate_env();

    let token_config = TokenIdConfigValue {
        token_address: interchain_token,
        token_manager,
        token_manager_type: TokenManagerType::LockUnlock,
    };

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    setup_migrate_storage(
        &env,
        token_config,
        &its_client,
        token_id.clone(),
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );

    let its_upgrade_auth = mock_auth!(owner, its_client.upgrade(&its_wasm_hash));
    let its_migrate_auth = mock_auth!(owner, its_client.migrate(migration_data.clone()));
    let upgrader_upgrade_auth = mock_auth!(
        owner,
        upgrader_client.upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.clone()],
        ),
        &[
            its_upgrade_auth.invoke.clone(),
            its_migrate_auth.invoke.clone()
        ]
    );

    upgrader_client
        .mock_auths(&[upgrader_upgrade_auth])
        .upgrade(
            &its_client.address,
            &String::from_str(&env, NEW_VERSION),
            &its_wasm_hash,
            &vec![&env, migration_data.into_val(&env)],
        );

    its_client
        // .mock_auths(&[its_migrate_token_auth]) // FIXME: Use
        .mock_all_auths_allowing_non_root_auth()
        .migrate_token(
            &token_id,
            &upgrader_client.address,
            &String::from_str(&env, NEW_VERSION),
        );

    assert_migrate_storage(
        &env,
        &its_client,
        migration_data,
        token_id,
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );
}
