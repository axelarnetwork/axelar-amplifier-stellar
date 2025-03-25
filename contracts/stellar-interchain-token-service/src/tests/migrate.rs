use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_std::interfaces::CustomMigratableInterface;
use stellar_axelar_std::testutils::BytesN as _;
use stellar_axelar_std::{assert_auth, assert_contract_err, assert_ok, Address, BytesN, String};
use stellar_upgrader::testutils::setup_upgrader;

use crate::error::ContractError;
use crate::flow_limit::current_epoch;
use crate::interface::InterchainTokenServiceInterface;
use crate::migrate::{legacy_storage, CustomMigrationData};
use crate::storage::{self, TokenIdConfigValue};
use crate::tests::utils::setup_env;
use crate::testutils::setup_its_token;
use crate::types::TokenManagerType;
use crate::InterchainTokenService;

const ITS_WASM: &[u8] = include_bytes!("testdata/stellar_interchain_token_service.optimized.wasm");
const TOKEN_MANAGER_WASM_V110: &[u8] =
    include_bytes!("testdata/stellar_token_manager_v1_1_0.optimized.wasm");
const INTERCHAIN_TOKEN_WASM_V110: &[u8] =
    include_bytes!("testdata/stellar_interchain_token_v1_1_0.optimized.wasm");

const VERSION_V110: &str = "1.1.0";

#[test]
fn migrate_native_interchain_token_succeeds() {
    let (env, its_client, _, _, _) = setup_env();
    let upgrader_client = setup_upgrader(&env);
    let owner: Address = its_client.owner();
    let (token_id, _) = setup_its_token(&env, &its_client, &owner, 100);

    let its_wasm_hash = env.deployer().upload_contract_wasm(ITS_WASM);
    let token_manager_wasm_hash_v110 = env.deployer().upload_contract_wasm(TOKEN_MANAGER_WASM_V110);
    let interchain_token_wasm_hash_v110 = env
        .deployer()
        .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V110);

    let current_epoch = current_epoch(&env);

    let token_manager_v100 = its_client.token_manager_address(&token_id);
    let interchain_token_v100 = its_client.interchain_token_address(&token_id);

    let token_config = TokenIdConfigValue {
        token_address: interchain_token_v100,
        token_manager: token_manager_v100,
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

    assert_auth!(owner, its_client.upgrade(&its_wasm_hash));

    let migration_data = CustomMigrationData {
        new_token_manager_wasm_hash: token_manager_wasm_hash_v110.clone(),
        new_interchain_token_wasm_hash: interchain_token_wasm_hash_v110.clone(),
    };

    env.mock_all_auths_allowing_non_root_auth();

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::__migrate(&env, migration_data));
    });

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::migrate_token(
            &env,
            token_id.clone(),
            upgrader_client.address,
            String::from_str(&env, VERSION_V110),
        ));
    });

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::token_manager_wasm_hash(&env)
        }),
        token_manager_wasm_hash_v110,
        "token manager WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::interchain_token_wasm_hash(&env)
        }),
        interchain_token_wasm_hash_v110,
        "interchain token WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_in(&env, token_id.clone(), current_epoch)
        }),
        flow_in_amount
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_out(&env, token_id, current_epoch)
        }),
        flow_out_amount
    );
}

#[test]
fn migrate_lock_unlock_succeeds() {
    let (env, its_client, _, _, _) = setup_env();
    let upgrader_client = setup_upgrader(&env);
    let owner: Address = its_client.owner();
    let (token_id, _) = setup_its_token(&env, &its_client, &owner, 100);

    let its_wasm_hash = env.deployer().upload_contract_wasm(ITS_WASM);
    let token_manager_wasm_hash_v110 = env.deployer().upload_contract_wasm(TOKEN_MANAGER_WASM_V110);
    let interchain_token_wasm_hash_v110 = env
        .deployer()
        .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V110);

    let current_epoch = current_epoch(&env);

    let token_manager_v100 = its_client.token_manager_address(&token_id);
    let interchain_token_v100 = its_client.interchain_token_address(&token_id);

    let token_config = TokenIdConfigValue {
        token_address: interchain_token_v100,
        token_manager: token_manager_v100,
        token_manager_type: TokenManagerType::LockUnlock,
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

    assert_auth!(owner, its_client.upgrade(&its_wasm_hash));

    let migration_data = CustomMigrationData {
        new_token_manager_wasm_hash: token_manager_wasm_hash_v110.clone(),
        new_interchain_token_wasm_hash: interchain_token_wasm_hash_v110.clone(),
    };

    env.mock_all_auths_allowing_non_root_auth();

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::__migrate(&env, migration_data));
    });

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::migrate_token(
            &env,
            token_id.clone(),
            upgrader_client.address,
            String::from_str(&env, VERSION_V110),
        ));
    });

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::token_manager_wasm_hash(&env)
        }),
        token_manager_wasm_hash_v110,
        "token manager WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::interchain_token_wasm_hash(&env)
        }),
        interchain_token_wasm_hash_v110,
        "interchain token WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_in(&env, token_id.clone(), current_epoch)
        }),
        flow_in_amount
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_out(&env, token_id, current_epoch)
        }),
        flow_out_amount
    );
}

#[test]
fn migrate_token_fails_with_invalid_token_id() {
    let (env, its_client, _, _, _) = setup_env();
    let upgrader_client = setup_upgrader(&env);
    let owner = its_client.owner();
    let non_existent_token_id = BytesN::random(&env);

    let its_wasm_hash = env.deployer().upload_contract_wasm(ITS_WASM);
    let token_manager_wasm_hash_v110 = env.deployer().upload_contract_wasm(TOKEN_MANAGER_WASM_V110);
    let interchain_token_wasm_hash_v110 = env
        .deployer()
        .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V110);

    assert_auth!(owner, its_client.upgrade(&its_wasm_hash));

    let migration_data = CustomMigrationData {
        new_token_manager_wasm_hash: token_manager_wasm_hash_v110,
        new_interchain_token_wasm_hash: interchain_token_wasm_hash_v110,
    };

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::__migrate(&env, migration_data));
    });

    assert_contract_err!(
        its_client
            .mock_all_auths_allowing_non_root_auth()
            .try_migrate_token(
                &non_existent_token_id,
                &upgrader_client.address,
                &String::from_str(&env, VERSION_V110),
            ),
        ContractError::InvalidTokenId
    );
}

#[test]
fn migrate_succeeds_with_multiple_token_ids() {
    let (env, its_client, _, _, _) = setup_env();
    let upgrader_client = setup_upgrader(&env);
    let owner = its_client.owner();
    let (token_id_1, _) = setup_its_token(&env, &its_client, &owner, 100);
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

    let its_wasm_hash = env.deployer().upload_contract_wasm(ITS_WASM);
    let token_manager_wasm_hash_v110 = env.deployer().upload_contract_wasm(TOKEN_MANAGER_WASM_V110);
    let interchain_token_wasm_hash_v110 = env
        .deployer()
        .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V110);

    let current_epoch = current_epoch(&env);

    let token_manager_v100_1 = its_client.deployed_token_manager(&token_id_1);
    let token_manager_v100_2 = its_client.deployed_token_manager(&token_id_2);

    let interchain_token_v100_1 = its_client.interchain_token_address(&token_id_1);
    let interchain_token_v100_2 = its_client.interchain_token_address(&token_id_2);

    let token_config_1 = TokenIdConfigValue {
        token_address: interchain_token_v100_1,
        token_manager: token_manager_v100_1,
        token_manager_type: TokenManagerType::LockUnlock,
    };

    let token_config_2 = TokenIdConfigValue {
        token_address: interchain_token_v100_2,
        token_manager: token_manager_v100_2,
        token_manager_type: TokenManagerType::NativeInterchainToken,
    };

    let flow_in_amount_1 = 100i128;
    let flow_out_amount_1 = 50i128;
    let flow_in_amount_2 = 200i128;
    let flow_out_amount_2 = 150i128;

    env.as_contract(&its_client.address, || {
        let flow_key_1 = legacy_storage::FlowKey {
            token_id: token_id_1.clone(),
            epoch: current_epoch,
        };
        let flow_key_2 = legacy_storage::FlowKey {
            token_id: token_id_2.clone(),
            epoch: current_epoch,
        };

        legacy_storage::set_flow_in(&env, flow_key_1.clone(), &flow_in_amount_1);
        legacy_storage::set_flow_out(&env, flow_key_1, &flow_out_amount_1);
        legacy_storage::set_flow_in(&env, flow_key_2.clone(), &flow_in_amount_2);
        legacy_storage::set_flow_out(&env, flow_key_2, &flow_out_amount_2);

        storage::set_token_id_config(&env, token_id_1.clone(), &token_config_1);
        storage::set_token_id_config(&env, token_id_2.clone(), &token_config_2);
    });

    assert_auth!(owner, its_client.upgrade(&its_wasm_hash));

    let migration_data = CustomMigrationData {
        new_token_manager_wasm_hash: token_manager_wasm_hash_v110.clone(),
        new_interchain_token_wasm_hash: interchain_token_wasm_hash_v110.clone(),
    };

    env.mock_all_auths_allowing_non_root_auth();

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::__migrate(&env, migration_data));
    });

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::migrate_token(
            &env,
            token_id_1.clone(),
            upgrader_client.address.clone(),
            String::from_str(&env, VERSION_V110),
        ));
    });

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::migrate_token(
            &env,
            token_id_2.clone(),
            upgrader_client.address,
            String::from_str(&env, VERSION_V110),
        ));
    });

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::token_manager_wasm_hash(&env)
        }),
        token_manager_wasm_hash_v110,
        "token manager WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::interchain_token_wasm_hash(&env)
        }),
        interchain_token_wasm_hash_v110,
        "interchain token WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_in(&env, token_id_1.clone(), current_epoch)
        }),
        flow_in_amount_1
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_out(&env, token_id_1, current_epoch)
        }),
        flow_out_amount_1
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_in(&env, token_id_2.clone(), current_epoch)
        }),
        flow_in_amount_2
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_out(&env, token_id_2, current_epoch)
        }),
        flow_out_amount_2
    );
}

#[test]
fn migrate_succeeds_with_empty_migration_data() {
    let (env, its_client, _, _, _) = setup_env();
    let owner = its_client.owner();

    let its_wasm_hash = env.deployer().upload_contract_wasm(ITS_WASM);
    let token_manager_wasm_hash_v110 = env.deployer().upload_contract_wasm(TOKEN_MANAGER_WASM_V110);
    let interchain_token_wasm_hash_v110 = env
        .deployer()
        .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V110);

    assert_auth!(owner, its_client.upgrade(&its_wasm_hash));

    let migration_data = CustomMigrationData {
        new_token_manager_wasm_hash: token_manager_wasm_hash_v110,
        new_interchain_token_wasm_hash: interchain_token_wasm_hash_v110,
    };

    env.mock_all_auths_allowing_non_root_auth();

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::__migrate(&env, migration_data));
    });
}

#[test]
fn migrate_with_native_interchain_token_legacy_flow_data() {
    let (env, its_client, _, _, _) = setup_env();
    let upgrader_client = setup_upgrader(&env);
    let owner = its_client.owner();
    let (token_id, _) = setup_its_token(&env, &its_client, &owner, 100);

    let its_wasm_hash = env.deployer().upload_contract_wasm(ITS_WASM);
    let token_manager_wasm_hash_v110 = env.deployer().upload_contract_wasm(TOKEN_MANAGER_WASM_V110);
    let interchain_token_wasm_hash_v110 = env
        .deployer()
        .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V110);

    let current_epoch = current_epoch(&env);

    let token_manager_v100 = its_client.token_manager_address(&token_id);
    let interchain_token_v100 = its_client.interchain_token_address(&token_id);

    let token_config = TokenIdConfigValue {
        token_address: interchain_token_v100,
        token_manager: token_manager_v100,
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

    assert_auth!(owner, its_client.upgrade(&its_wasm_hash));

    let migration_data = CustomMigrationData {
        new_token_manager_wasm_hash: token_manager_wasm_hash_v110.clone(),
        new_interchain_token_wasm_hash: interchain_token_wasm_hash_v110.clone(),
    };

    env.mock_all_auths_allowing_non_root_auth();

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::__migrate(&env, migration_data));
    });

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::migrate_token(
            &env,
            token_id.clone(),
            upgrader_client.address,
            String::from_str(&env, VERSION_V110),
        ));
    });

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_in(&env, token_id.clone(), current_epoch)
        }),
        flow_in_amount,
        "flow in value should be migrated correctly"
    );

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_out(&env, token_id, current_epoch)
        }),
        flow_out_amount,
        "flow out value should be migrated correctly"
    );

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::token_manager_wasm_hash(&env)
        }),
        token_manager_wasm_hash_v110,
        "token manager WASM hash should be updated"
    );

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::interchain_token_wasm_hash(&env)
        }),
        interchain_token_wasm_hash_v110,
        "interchain token WASM hash should be updated"
    );
}

#[test]
fn migrate_with_lock_unlock_legacy_flow_data() {
    let (env, its_client, _, _, _) = setup_env();
    let upgrader_client = setup_upgrader(&env);
    let owner = its_client.owner();
    let (token_id, _) = setup_its_token(&env, &its_client, &owner, 100);

    let its_wasm_hash = env.deployer().upload_contract_wasm(ITS_WASM);
    let token_manager_wasm_hash_v110 = env.deployer().upload_contract_wasm(TOKEN_MANAGER_WASM_V110);
    let interchain_token_wasm_hash_v110 = env
        .deployer()
        .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V110);

    let current_epoch = current_epoch(&env);

    let token_manager_v100 = its_client.token_manager_address(&token_id);
    let interchain_token_v100 = its_client.interchain_token_address(&token_id);

    let token_config = TokenIdConfigValue {
        token_address: interchain_token_v100,
        token_manager: token_manager_v100,
        token_manager_type: TokenManagerType::LockUnlock,
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

    assert_auth!(owner, its_client.upgrade(&its_wasm_hash));

    let migration_data = CustomMigrationData {
        new_token_manager_wasm_hash: token_manager_wasm_hash_v110.clone(),
        new_interchain_token_wasm_hash: interchain_token_wasm_hash_v110.clone(),
    };

    env.mock_all_auths_allowing_non_root_auth();

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::__migrate(&env, migration_data));
    });

    env.as_contract(&its_client.address, || {
        assert_ok!(InterchainTokenService::migrate_token(
            &env,
            token_id.clone(),
            upgrader_client.address,
            String::from_str(&env, VERSION_V110),
        ));
    });

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_in(&env, token_id.clone(), current_epoch)
        }),
        flow_in_amount,
        "flow in value should be migrated correctly"
    );

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_out(&env, token_id, current_epoch)
        }),
        flow_out_amount,
        "flow out value should be migrated correctly"
    );

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::token_manager_wasm_hash(&env)
        }),
        token_manager_wasm_hash_v110,
        "token manager WASM hash should be updated"
    );

    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::interchain_token_wasm_hash(&env)
        }),
        interchain_token_wasm_hash_v110,
        "interchain token WASM hash should be updated"
    );
}
