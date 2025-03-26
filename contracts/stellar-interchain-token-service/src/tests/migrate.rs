use stellar_axelar_std::testutils::BytesN as _;
use stellar_axelar_std::{assert_contract_err, BytesN, String};
use testutils::{
    assert_migrate_storage, migrate_token, setup_migrate_env, setup_migrate_storage, upgrade,
    FlowData, MigrateTestConfig,
};

use crate::error::ContractError;
use crate::storage::TokenIdConfigValue;
use crate::tests::utils::format_auths;
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

    setup_migrate_storage(
        &env,
        token_config,
        &its_client,
        token_id.clone(),
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );

    let upgrader_upgrade_auths = upgrade(
        &env,
        owner,
        &its_client,
        &upgrader_client,
        its_wasm_hash,
        migration_data.clone(),
    );

    let its_migrate_token_auths =
        migrate_token(&env, &its_client, &upgrader_client, token_id.clone());

    goldie::assert!([upgrader_upgrade_auths, its_migrate_token_auths].join("\n\n"));

    assert_migrate_storage(
        &env,
        &its_client,
        migration_data,
        Some(FlowData {
            token_id,
            current_epoch,
            flow_in_amount,
            flow_out_amount,
        }),
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

    let upgrader_upgrade_auths = upgrade(
        &env,
        owner,
        &its_client,
        &upgrader_client,
        its_wasm_hash,
        migration_data.clone(),
    );

    let its_migrate_token_auths =
        migrate_token(&env, &its_client, &upgrader_client, token_id.clone());

    goldie::assert!([upgrader_upgrade_auths, its_migrate_token_auths].join("\n\n"));

    assert_migrate_storage(
        &env,
        &its_client,
        migration_data,
        Some(FlowData {
            token_id,
            current_epoch,
            flow_in_amount,
            flow_out_amount,
        }),
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

    let upgrader_upgrade_auths = upgrade(
        &env,
        owner,
        &its_client,
        &upgrader_client,
        its_wasm_hash,
        migration_data.clone(),
    );

    assert_contract_err!(
        its_client.mock_all_auths().try_migrate_token(
            &non_existent_token_id,
            &upgrader_client.address,
            &String::from_str(&env, NEW_VERSION),
        ),
        ContractError::InvalidTokenId
    );

    let its_migrate_token_auths = format_auths(env.auths(), "its.migrate_token(...)");

    goldie::assert!([upgrader_upgrade_auths, its_migrate_token_auths].join("\n\n"));
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

    let upgrader_upgrade_auths = upgrade(
        &env,
        owner,
        &its_client,
        &upgrader_client,
        its_wasm_hash,
        migration_data.clone(),
    );

    goldie::assert!(upgrader_upgrade_auths);

    assert_migrate_storage(&env, &its_client, migration_data.clone(), None);
}

#[test]
fn migrate_native_interchain_token_with_flow_amount_succeeds() {
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

    let upgrader_upgrade_auths = upgrade(
        &env,
        owner,
        &its_client,
        &upgrader_client,
        its_wasm_hash,
        migration_data.clone(),
    );

    let its_migrate_token_auths =
        migrate_token(&env, &its_client, &upgrader_client, token_id.clone());

    goldie::assert!([upgrader_upgrade_auths, its_migrate_token_auths].join("\n\n"));

    assert_migrate_storage(
        &env,
        &its_client,
        migration_data,
        Some(FlowData {
            token_id,
            current_epoch,
            flow_in_amount,
            flow_out_amount,
        }),
    );
}

#[test]
fn migrate_with_lock_unlock_with_flow_amount_succeeds() {
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

    let upgrader_upgrade_auths = upgrade(
        &env,
        owner,
        &its_client,
        &upgrader_client,
        its_wasm_hash,
        migration_data.clone(),
    );

    let its_migrate_token_auths =
        migrate_token(&env, &its_client, &upgrader_client, token_id.clone());

    goldie::assert!([upgrader_upgrade_auths, its_migrate_token_auths].join("\n\n"));

    assert_migrate_storage(
        &env,
        &its_client,
        migration_data,
        Some(FlowData {
            token_id,
            current_epoch,
            flow_in_amount,
            flow_out_amount,
        }),
    );
}

mod testutils {
    use stellar_axelar_std::{mock_auth, vec, Address, BytesN, Env, IntoVal, String};
    use stellar_upgrader::testutils::setup_upgrader;
    use stellar_upgrader::UpgraderClient;

    use crate::flow_limit::current_epoch;
    use crate::migrate::{legacy_storage, CustomMigrationData};
    use crate::storage::{self, TokenIdConfigValue};
    use crate::tests::utils::{format_auths, setup_env};
    use crate::testutils::setup_its_token;
    use crate::InterchainTokenServiceClient;

    const NEW_INTERCHAIN_TOKEN_SERVICE_WASM: &[u8] =
        include_bytes!("testdata/stellar_interchain_token_service.optimized.wasm");
    const TOKEN_MANAGER_WASM_V110: &[u8] =
        include_bytes!("testdata/stellar_token_manager_v1_1_0.optimized.wasm");
    const INTERCHAIN_TOKEN_WASM_V110: &[u8] =
        include_bytes!("testdata/stellar_interchain_token_v1_1_0.optimized.wasm");

    const NEW_VERSION: &str = "1.1.0";

    pub struct MigrateTestConfig<'a> {
        pub env: Env,
        pub owner: Address,
        pub its_client: InterchainTokenServiceClient<'a>,
        pub upgrader_client: UpgraderClient<'a>,
        pub token_id: BytesN<32>,
        pub current_epoch: u64,
        pub its_wasm_hash: BytesN<32>,
        pub token_manager: Address,
        pub interchain_token: Address,
        pub migration_data: CustomMigrationData,
    }

    pub struct FlowData {
        pub token_id: BytesN<32>,
        pub current_epoch: u64,
        pub flow_in_amount: i128,
        pub flow_out_amount: i128,
    }

    pub fn setup_migrate_env<'a>() -> MigrateTestConfig<'a> {
        let (env, its_client, ..) = setup_env();
        let upgrader_client = setup_upgrader(&env);
        let owner: Address = its_client.owner();
        let (token_id, _) = setup_its_token(&env, &its_client, &owner, 100);

        let its_wasm_hash = env
            .deployer()
            .upload_contract_wasm(NEW_INTERCHAIN_TOKEN_SERVICE_WASM);
        let new_token_manager_wasm_hash =
            env.deployer().upload_contract_wasm(TOKEN_MANAGER_WASM_V110);
        let new_interchain_token_wasm_hash = env
            .deployer()
            .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V110);

        let current_epoch = current_epoch(&env);

        let token_manager = its_client.token_manager_address(&token_id);
        let interchain_token = its_client.interchain_token_address(&token_id);

        MigrateTestConfig {
            env,
            owner,
            its_client,
            upgrader_client,
            token_id,
            current_epoch,
            its_wasm_hash,
            token_manager,
            interchain_token,
            migration_data: CustomMigrationData {
                new_token_manager_wasm_hash,
                new_interchain_token_wasm_hash,
            },
        }
    }

    pub fn setup_migrate_storage<'a>(
        env: &Env,
        token_config: TokenIdConfigValue,
        its_client: &InterchainTokenServiceClient<'a>,
        token_id: BytesN<32>,
        current_epoch: u64,
        flow_in_amount: i128,
        flow_out_amount: i128,
    ) {
        env.as_contract(&its_client.address, || {
            let flow_key = legacy_storage::FlowKey {
                token_id: token_id.clone(),
                epoch: current_epoch,
            };

            legacy_storage::set_flow_in(&env, flow_key.clone(), &flow_in_amount);
            legacy_storage::set_flow_out(&env, flow_key, &flow_out_amount);

            storage::set_token_id_config(&env, token_id.clone(), &token_config);
        });
    }

    pub fn upgrade<'a>(
        env: &Env,
        owner: Address,
        its_client: &InterchainTokenServiceClient<'a>,
        upgrader_client: &UpgraderClient<'a>,
        its_wasm_hash: BytesN<32>,
        migration_data: CustomMigrationData,
    ) -> std::string::String {
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
                &vec![&env, migration_data.into_val(env)],
            );

        format_auths(env.auths(), "upgrader.upgrade(...)")
    }

    pub fn migrate_token<'a>(
        env: &Env,
        its_client: &InterchainTokenServiceClient<'a>,
        upgrader_client: &UpgraderClient<'a>,
        token_id: BytesN<32>,
    ) -> std::string::String {
        its_client
            .mock_all_auths_allowing_non_root_auth()
            .migrate_token(
                &token_id,
                &upgrader_client.address,
                &String::from_str(&env, NEW_VERSION),
            );

        format_auths(env.auths(), "its.migrate_token(...)")
    }

    pub fn assert_migrate_storage<'a>(
        env: &Env,
        its_client: &InterchainTokenServiceClient<'a>,
        migration_data: CustomMigrationData,
        flow_data: Option<FlowData>,
    ) {
        assert_eq!(
            env.as_contract(&its_client.address, || {
                storage::token_manager_wasm_hash(&env)
            }),
            migration_data.new_token_manager_wasm_hash,
            "token manager WASM hash should be updated"
        );
        assert_eq!(
            env.as_contract(&its_client.address, || {
                storage::interchain_token_wasm_hash(&env)
            }),
            migration_data.new_interchain_token_wasm_hash,
            "interchain token WASM hash should be updated"
        );
        if let Some(flow_data) = flow_data {
            assert_eq!(
                env.as_contract(&its_client.address, || {
                    storage::flow_in(&env, flow_data.token_id.clone(), flow_data.current_epoch)
                }),
                flow_data.flow_in_amount
            );
            assert_eq!(
                env.as_contract(&its_client.address, || {
                    storage::flow_out(&env, flow_data.token_id, flow_data.current_epoch)
                }),
                flow_data.flow_out_amount
            );
        }
    }
}
