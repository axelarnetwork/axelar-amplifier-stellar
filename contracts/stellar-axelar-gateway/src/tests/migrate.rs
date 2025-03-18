use soroban_sdk::{vec, BytesN, String};
use stellar_axelar_std::interfaces::CustomMigratableInterface;
use stellar_axelar_std::{assert_auth, assert_err};

use crate::contract::AxelarGateway;
use crate::error::ContractError;
use crate::migrate::legacy_storage;
use crate::storage::{self, MessageApprovalValue};
use crate::tests::testutils::{
    setup_env, TestConfig,
};

const NEW_WASM: &[u8] = include_bytes!("testdata/stellar_axelar_gateway.optimized.wasm");

#[test]
fn migrate_succeeds() {
    let TestConfig { env, client, .. } = setup_env(1, 5);

    let owner = client.owner();

    let new_wasm_hash = env.deployer().upload_contract_wasm(NEW_WASM);

    let source_chain = String::from_str(&env, "source_chain");
    let message_id = String::from_str(&env, "message_id");
    let hash: BytesN<32> = BytesN::from_array(&env, &[1; 32]);

    env.as_contract(&client.address, || {
        let key = legacy_storage::MessageApprovalKey {
            source_chain: source_chain.clone(),
            message_id: message_id.clone(),
        };

        legacy_storage::set_message_approval(&env, key, &MessageApprovalValue::Approved(hash.clone()));
    });

    assert_auth!(owner, client.upgrade(&new_wasm_hash));

    let migration_data = vec![&env, (source_chain, message_id)];

    env.as_contract(&client.address, || {
        let result = AxelarGateway::__migrate(&env, migration_data);
        assert!(result.is_ok());
    });
}

#[test]
fn migrate_succeeds_with_valid_message_approvals() {
    let TestConfig { env, client, .. } = setup_env(1, 5);

    let owner = client.owner();

    let new_wasm_hash = env.deployer().upload_contract_wasm(NEW_WASM);

    let source_chain_1 = String::from_str(&env, "ethereum");
    let message_id_1 = String::from_str(&env, "message1");
    let hash: BytesN<32> = BytesN::from_array(&env, &[1; 32]);

    let source_chain_2 = String::from_str(&env, "polygon");
    let message_id_2 = String::from_str(&env, "message2");

    env.as_contract(&client.address, || {
        let key_1 = legacy_storage::MessageApprovalKey {
            source_chain: source_chain_1.clone(),
            message_id: message_id_1.clone(),
        };
        legacy_storage::set_message_approval(&env, key_1, &MessageApprovalValue::Approved(hash.clone()));

        let key_2 = legacy_storage::MessageApprovalKey {
            source_chain: source_chain_2.clone(),
            message_id: message_id_2.clone(),
        };
        legacy_storage::set_message_approval(&env, key_2, &MessageApprovalValue::Executed);
    });

    assert_auth!(owner, client.upgrade(&new_wasm_hash));

    let migration_data = vec![
        &env,
        (source_chain_1.clone(), message_id_1.clone()),
        (source_chain_2.clone(), message_id_2.clone()),
    ];

    assert_auth!(owner, client.migrate(&migration_data));

    assert_eq!(
        env.as_contract(&client.address, || {
            storage::message_approval(&env, source_chain_1, message_id_1)
        }),
        MessageApprovalValue::Approved(hash)
    );
    assert_eq!(
        env.as_contract(&client.address, || {
            storage::message_approval(&env, source_chain_2, message_id_2)
        }),
        MessageApprovalValue::Executed
    );
}

#[test]
fn migrate_fails_when_invalid_message_approval() {
    let TestConfig { env, client, .. } = setup_env(1, 5);

    let owner = client.owner();

    let new_wasm_hash = env.deployer().upload_contract_wasm(NEW_WASM);

    let source_chain_1 = String::from_str(&env, "ethereum");
    let message_id_1 = String::from_str(&env, "message1");
    let hash: BytesN<32> = BytesN::from_array(&env, &[1; 32]);

    let source_chain_2 = String::from_str(&env, "polygon");
    let message_id_2 = String::from_str(&env, "non_existent");

    env.as_contract(&client.address, || {
        let key = legacy_storage::MessageApprovalKey {
            source_chain: source_chain_1.clone(),
            message_id: message_id_1.clone(),
        };

        legacy_storage::set_message_approval(&env, key, &MessageApprovalValue::Approved(hash.clone()));
    });

    assert_auth!(owner, client.upgrade(&new_wasm_hash));

    let migration_data = vec![
        &env,
        (source_chain_1, message_id_1),
        (source_chain_2, message_id_2),
    ];

    assert_err!(
        env.as_contract(&client.address, || {
            <AxelarGateway as CustomMigratableInterface>::__migrate(&env, migration_data)
        }),
        ContractError::InvalidMessageApproval
    );
}

#[test]
fn migrate_succeeds_with_empty_migration_data() {
    let TestConfig { env, client, .. } = setup_env(1, 5);

    let owner = client.owner();

    let new_wasm_hash = env.deployer().upload_contract_wasm(NEW_WASM);

    assert_auth!(owner, client.upgrade(&new_wasm_hash));

    let migration_data = vec![&env];

    assert_auth!(owner, client.migrate(&migration_data));
}
