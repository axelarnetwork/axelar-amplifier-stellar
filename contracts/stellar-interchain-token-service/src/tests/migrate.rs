use stellar_axelar_std::testutils::BytesN as _;
use stellar_axelar_std::{assert_contract_err, BytesN, String};
use testutils::{
    assert_migrate_storage, migrate, migrate_token, setup_migrate_env, setup_migrate_storage,
    upgrade, FlowData, MigrateTestConfig,
};

use crate::error::ContractError;
use crate::types::TokenManagerType;

const NEW_VERSION: &str = "0.0.0"; // Avoids Upgrader::ContractError::SameVersion during release.

mod testutils {
    use stellar_axelar_std::interfaces::CustomMigratableInterface;
    use stellar_axelar_std::{assert_ok, mock_auth, vec, Address, BytesN, Env, IntoVal, String};
    use stellar_upgrader::testutils::setup_upgrader;
    use stellar_upgrader::UpgraderClient;

    use super::NEW_VERSION;
    use crate::flow_limit::current_epoch;
    use crate::migrate::{legacy_storage, CustomMigrationData};
    use crate::tests::utils::setup_env;
    use crate::testutils::setup_its_token;
    use crate::types::TokenManagerType;
    use crate::{InterchainTokenService, InterchainTokenServiceClient};

    const NEW_INTERCHAIN_TOKEN_SERVICE_WASM: &[u8] =
        include_bytes!("testdata/stellar_interchain_token_service.optimized.wasm");
    const TOKEN_MANAGER_WASM_V0_0_0: &[u8] =
        include_bytes!("testdata/stellar_token_manager-v0.0.0.optimized.wasm"); // Avoids Upgrader::ContractError::SameVersion during release.
    const INTERCHAIN_TOKEN_WASM_V0_0_0: &[u8] =
        include_bytes!("testdata/stellar_interchain_token-v0.0.0.optimized.wasm"); // Avoids Upgrader::ContractError::SameVersion during release.

    pub struct MigrateTestConfig<'a> {
        pub env: Env,
        pub owner: Address,
        pub its_client: InterchainTokenServiceClient<'a>,
        pub upgrader_client: UpgraderClient<'a>,
        pub token_id: BytesN<32>,
        pub current_epoch: u64,
        pub its_wasm_hash: BytesN<32>,
        pub migration_data: CustomMigrationData,
    }

    pub struct FlowData {
        pub token_id: BytesN<32>,
        pub flow_in_amount: i128,
        pub flow_out_amount: i128,
    }

    pub fn setup_migrate_env<'a>(token_manager_type: TokenManagerType) -> MigrateTestConfig<'a> {
        let (env, its_client, ..) = setup_env();
        let upgrader_client = setup_upgrader(&env);
        let owner: Address = its_client.owner();
        let token_id;

        match token_manager_type {
            TokenManagerType::NativeInterchainToken => {
                (token_id, _) = setup_its_token(&env, &its_client, &owner, 100);
            }
            TokenManagerType::LockUnlock => {
                let token = env.register_stellar_asset_contract_v2(owner.clone());
                token_id = its_client.register_canonical_token(&token.address());
            }
        }

        let its_wasm_hash = env
            .deployer()
            .upload_contract_wasm(NEW_INTERCHAIN_TOKEN_SERVICE_WASM);
        let new_token_manager_wasm_hash = env
            .deployer()
            .upload_contract_wasm(TOKEN_MANAGER_WASM_V0_0_0);
        let new_interchain_token_wasm_hash = env
            .deployer()
            .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V0_0_0);

        let current_epoch = current_epoch(&env);

        MigrateTestConfig {
            env,
            owner,
            its_client,
            upgrader_client,
            token_id,
            current_epoch,
            its_wasm_hash,
            migration_data: CustomMigrationData {
                new_token_manager_wasm_hash,
                new_interchain_token_wasm_hash,
            },
        }
    }

    pub fn setup_migrate_storage(
        env: &Env,
        its_client: &InterchainTokenServiceClient<'_>,
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

            legacy_storage::set_flow_in(env, flow_key.clone(), &flow_in_amount);
            legacy_storage::set_flow_out(env, flow_key, &flow_out_amount);
        });
    }

    pub fn upgrade<'a>(
        env: &Env,
        owner: Address,
        its_client: &InterchainTokenServiceClient<'a>,
        upgrader_client: &UpgraderClient<'a>,
        its_wasm_hash: BytesN<32>,
        migration_data: CustomMigrationData,
    ) {
        let its_upgrade_auth = mock_auth!(owner, its_client.upgrade(&its_wasm_hash));
        let its_migrate_auth = mock_auth!(owner, its_client.migrate(migration_data));
        let upgrader_upgrade_auth = mock_auth!(
            owner,
            upgrader_client.upgrade(
                &its_client.address,
                &String::from_str(env, NEW_VERSION),
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
                &String::from_str(env, NEW_VERSION),
                &its_wasm_hash,
                &vec![&env, migration_data.into_val(env)],
            );
    }

    pub fn migrate(
        env: &Env,
        its_client: &InterchainTokenServiceClient<'_>,
        migration_data: CustomMigrationData,
    ) {
        env.as_contract(&its_client.address, || {
            assert_ok!(InterchainTokenService::__migrate(
                env,
                migration_data.clone()
            ));
        });
    }

    pub fn migrate_token<'a>(
        env: &Env,
        its_client: &InterchainTokenServiceClient<'a>,
        upgrader_client: &UpgraderClient<'a>,
        token_id: BytesN<32>,
    ) {
        let its_migrate_token_auth = mock_auth!(
            its_client.owner(),
            its_client.migrate_token(
                &token_id,
                &upgrader_client.address,
                &String::from_str(env, NEW_VERSION)
            )
        );

        its_client
            .mock_auths(&[its_migrate_token_auth])
            .migrate_token(
                &token_id,
                &upgrader_client.address,
                &String::from_str(env, NEW_VERSION),
            );
    }

    pub fn assert_migrate_storage(
        its_client: &InterchainTokenServiceClient<'_>,
        migration_data: CustomMigrationData,
        flow_data: Option<FlowData>,
    ) {
        assert_eq!(
            its_client.token_manager_wasm_hash(),
            migration_data.new_token_manager_wasm_hash,
            "token manager WASM hash should be updated"
        );
        assert_eq!(
            its_client.interchain_token_wasm_hash(),
            migration_data.new_interchain_token_wasm_hash,
            "interchain token WASM hash should be updated"
        );

        if let Some(flow_data) = flow_data {
            assert_eq!(
                its_client.flow_in_amount(&flow_data.token_id),
                flow_data.flow_in_amount
            );
            assert_eq!(
                its_client.flow_out_amount(&flow_data.token_id),
                flow_data.flow_out_amount
            );
        }
    }
}

#[test]
fn upgrade_and_migrate_succeeds() {
    let MigrateTestConfig {
        env,
        owner,
        its_client,
        upgrader_client,
        its_wasm_hash,
        migration_data,
        ..
    } = setup_migrate_env(TokenManagerType::NativeInterchainToken);

    upgrade(
        &env,
        owner,
        &its_client,
        &upgrader_client,
        its_wasm_hash,
        migration_data.clone(),
    );

    assert_migrate_storage(&its_client, migration_data, None);
}

#[test]
fn upgrade_and_migrate_native_interchain_token_succeeds() {
    let MigrateTestConfig {
        env,
        owner,
        its_client,
        upgrader_client,
        token_id,
        current_epoch,
        its_wasm_hash,
        migration_data,
        ..
    } = setup_migrate_env(TokenManagerType::NativeInterchainToken);

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    setup_migrate_storage(
        &env,
        &its_client,
        token_id.clone(),
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );

    upgrade(
        &env,
        owner,
        &its_client,
        &upgrader_client,
        its_wasm_hash,
        migration_data.clone(),
    );

    migrate_token(&env, &its_client, &upgrader_client, token_id.clone());

    assert_migrate_storage(
        &its_client,
        migration_data,
        Some(FlowData {
            token_id,
            flow_in_amount,
            flow_out_amount,
        }),
    );
}

#[test]
fn migrate_native_interchain_token_succeeds() {
    let MigrateTestConfig {
        env,
        its_client,
        upgrader_client,
        token_id,
        current_epoch,
        migration_data,
        ..
    } = setup_migrate_env(TokenManagerType::NativeInterchainToken);

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    setup_migrate_storage(
        &env,
        &its_client,
        token_id.clone(),
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );
    migrate(&env, &its_client, migration_data.clone());
    migrate_token(&env, &its_client, &upgrader_client, token_id.clone());

    assert_migrate_storage(
        &its_client,
        migration_data,
        Some(FlowData {
            token_id,
            flow_in_amount,
            flow_out_amount,
        }),
    );
}

#[test]
fn migrate_lock_unlock_succeeds() {
    let MigrateTestConfig {
        env,
        its_client,
        upgrader_client,
        token_id,
        current_epoch,
        migration_data,
        ..
    } = setup_migrate_env(TokenManagerType::LockUnlock);

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    setup_migrate_storage(
        &env,
        &its_client,
        token_id.clone(),
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );
    migrate(&env, &its_client, migration_data.clone());

    migrate_token(&env, &its_client, &upgrader_client, token_id.clone());

    assert_migrate_storage(
        &its_client,
        migration_data,
        Some(FlowData {
            token_id,
            flow_in_amount,
            flow_out_amount,
        }),
    );
}

#[test]
fn migrate_token_fails_with_invalid_token_id() {
    let MigrateTestConfig {
        env,
        its_client,
        upgrader_client,
        migration_data,
        ..
    } = setup_migrate_env(TokenManagerType::NativeInterchainToken);

    let non_existent_token_id = BytesN::random(&env);

    migrate(&env, &its_client, migration_data);

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
fn migrate_native_interchain_token_with_flow_amount_succeeds() {
    let MigrateTestConfig {
        env,
        its_client,
        upgrader_client,
        token_id,
        current_epoch,
        migration_data,
        ..
    } = setup_migrate_env(TokenManagerType::NativeInterchainToken);

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    setup_migrate_storage(
        &env,
        &its_client,
        token_id.clone(),
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );
    migrate(&env, &its_client, migration_data.clone());

    migrate_token(&env, &its_client, &upgrader_client, token_id.clone());

    assert_migrate_storage(
        &its_client,
        migration_data,
        Some(FlowData {
            token_id,
            flow_in_amount,
            flow_out_amount,
        }),
    );
}

#[test]
fn migrate_with_lock_unlock_with_flow_amount_succeeds() {
    let MigrateTestConfig {
        env,

        its_client,
        upgrader_client,
        token_id,
        current_epoch,

        migration_data,
        ..
    } = setup_migrate_env(TokenManagerType::LockUnlock);

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    setup_migrate_storage(
        &env,
        &its_client,
        token_id.clone(),
        current_epoch,
        flow_in_amount,
        flow_out_amount,
    );
    migrate(&env, &its_client, migration_data.clone());

    migrate_token(&env, &its_client, &upgrader_client, token_id.clone());

    assert_migrate_storage(
        &its_client,
        migration_data,
        Some(FlowData {
            token_id,
            flow_in_amount,
            flow_out_amount,
        }),
    );
}
