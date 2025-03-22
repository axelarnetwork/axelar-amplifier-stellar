use soroban_sdk::testutils::BytesN as _;
use soroban_sdk::{log, vec, Address, BytesN, Env, String, Val};
use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_std::interfaces::{CustomMigratableInterface, UpgradableClient};
use stellar_axelar_std::{assert_auth, assert_err, mock_auth};
use stellar_interchain_token::InterchainTokenClient;
use stellar_token_manager::TokenManagerClient;

use crate::error::ContractError;
use crate::migrate::{legacy_storage, CustomMigrationData};
use crate::storage::{self, TokenIdConfigValue};
use crate::tests::utils::setup_env;
use crate::testutils::{setup_its_token, setup_upgrader};
use crate::types::TokenManagerType;
use crate::{InterchainTokenService, InterchainTokenServiceClient};

const ITS_WASM_V110: &[u8] =
    include_bytes!("testdata/stellar_interchain_token_service_v110.optimized.wasm");
const TOKEN_MANAGER_WASM_V110: &[u8] =
    include_bytes!("testdata/stellar_token_manager_v110.optimized.wasm");
const INTERCHAIN_TOKEN_WASM_V110: &[u8] =
    include_bytes!("testdata/stellar_interchain_token_v110.optimized.wasm");

#[test]
fn migrate_succeeds() {
    let (env, its_client_v100, _, _, _) = setup_env();
    let upgrader_client = setup_upgrader(&env);
    let owner: Address = its_client_v100.owner();
    let (token_id, _) = setup_its_token(&env, &its_client_v100, &owner, 100);

    let its_wasm_hash_v110 = env.deployer().upload_contract_wasm(ITS_WASM_V110);
    let token_manager_wasm_hash_v110 = env.deployer().upload_contract_wasm(TOKEN_MANAGER_WASM_V110);
    let interchain_token_wasm_hash_v110 = env
        .deployer()
        .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V110);

    let current_epoch = 123u64;

    let its_v100 = &its_client_v100.address;
    let upgradable_its_v100_client = UpgradableClient::new(&env, its_v100);
    log!(
        &env,
        "upgradable_its_v100_client.version(): {}",
        upgradable_its_v100_client.version()
    );

    let token_manager_v100 = its_client_v100.token_manager_address(&token_id);
    let token_manager_v100_client = TokenManagerClient::new(&env, &token_manager_v100);
    let upgradable_token_manager_v100_client = UpgradableClient::new(&env, &token_manager_v100);
    log!(
        &env,
        "upgradable_token_manager_v100_client.version(): {}",
        upgradable_token_manager_v100_client.version()
    );

    let interchain_token_v100 = its_client_v100.interchain_token_address(&token_id);
    let interchain_token_v100_client = InterchainTokenClient::new(&env, &interchain_token_v100);
    let upgradable_interchain_token_v100_client = UpgradableClient::new(&env, &interchain_token_v100);
    log!(
        &env,
        "upgradable_interchain_token_v100_client.version(): {}",
        upgradable_interchain_token_v100_client.version()
    );

    let token_config = TokenIdConfigValue {
        token_address: interchain_token_v100.clone(),
        token_manager: token_manager_v100.clone(),
        token_manager_type: TokenManagerType::LockUnlock,
    };

    let flow_in_amount = 100i128;
    let flow_out_amount = 50i128;

    env.as_contract(&its_client_v100.address, || {
        let flow_key = legacy_storage::FlowKey {
            token_id: token_id.clone(),
            epoch: current_epoch,
        };

        legacy_storage::set_flow_in(&env, flow_key.clone(), &flow_in_amount);
        legacy_storage::set_flow_out(&env, flow_key, &flow_out_amount);

        storage::set_token_id_config(&env, token_id.clone(), &token_config);
        storage::set_token_manager_wasm_hash(&env, &token_manager_wasm_hash_v110);
        storage::set_interchain_token_wasm_hash(&env, &interchain_token_wasm_hash_v110);
    });

    assert_auth!(owner, its_client_v100.upgrade(&its_wasm_hash_v110));

    let its_client_v110 = InterchainTokenServiceClient::new(&env, its_v100);
    let upgradable_its_v110_client = UpgradableClient::new(&env, its_v100);
    log!(
        &env,
        "upgradable_its_v110_client.version(): {}",
        upgradable_its_v110_client.version()
    );

    let version_v110 = String::from_str(&env, "1.1.0");

    let migration_data = CustomMigrationData {
        new_token_manager_wasm_hash: token_manager_wasm_hash_v110.clone(),
        new_interchain_token_wasm_hash: interchain_token_wasm_hash_v110.clone(),
        token_ids: vec![&env, token_id.clone()],
        upgrader_client: upgrader_client.address.clone(),
        new_version: version_v110.clone(),
    };

    its_client_v110.mock_all_auths_allowing_non_root_auth();
    upgrader_client.mock_all_auths_allowing_non_root_auth();
    token_manager_v100_client.mock_all_auths_allowing_non_root_auth();
    interchain_token_v100_client.mock_all_auths_allowing_non_root_auth();

    env.mock_all_auths_allowing_non_root_auth();

    let token_manager_upgrader_auth = mock_auth!(
        owner,
        upgrader_client.upgrade(
            token_manager_v100,
            version_v110,
            token_manager_wasm_hash_v110,
            soroban_sdk::Vec::<Val>::new(&env)
        )
    );

    let interchain_token_upgrader_auth = mock_auth!(
        owner,
        upgrader_client.upgrade(
            interchain_token_v100,
            version_v110,
            interchain_token_wasm_hash_v110,
            soroban_sdk::Vec::<Val>::new(&env)
        )
    );

    env.set_auths(&[
        token_manager_upgrader_auth.into(),
        interchain_token_upgrader_auth.into(),
    ]);

    log!(env, "Calling into client.migrate(&migration_data)...");
    assert_auth!(owner, its_client_v110.migrate(&migration_data));

    assert_eq!(
        env.as_contract(&its_client_v110.address, || {
            storage::token_manager_wasm_hash(&env)
        }),
        token_manager_wasm_hash_v110,
        "token manager WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&its_client_v110.address, || {
            storage::interchain_token_wasm_hash(&env)
        }),
        interchain_token_wasm_hash_v110,
        "interchain token WASM hash should be updated"
    );
}

// #[test]
// fn migrate_fails_with_invalid_token_id() {
//     let (env, client, _, _, _) = setup_env();

//     env.mock_all_auths();

//     let upgrader_client = setup_upgrader(&env);
//     let owner = client.owner();

//     let new_its_wasm_hash = env.deployer().upload_contract_wasm(NEW_ITS_WASM);
//     let new_token_manager_wasm_hash = env.deployer().upload_contract_wasm(NEW_TOKEN_MANAGER_WASM);
//     let new_interchain_token_wasm_hash = env
//         .deployer()
//         .upload_contract_wasm(NEW_INTERCHAIN_TOKEN_WASM);

//     let non_existent_token_id = BytesN::random(&env);

//     assert_auth!(owner, client.upgrade(&new_its_wasm_hash));

//     let migration_data = CustomMigrationData {
//         new_token_manager_wasm_hash,
//         new_interchain_token_wasm_hash,
//         token_ids: vec![&env, non_existent_token_id],
//         upgrader_client: upgrader_client.address,
//         new_version: String::from_str(&env, "1.1.0"),
//     };

//     assert_err!(
//         env.as_contract(&client.address, || {
//             <InterchainTokenService as CustomMigratableInterface>::__migrate(&env, migration_data)
//         }),
//         ContractError::InvalidTokenId
//     );
// }

// #[test]
// fn migrate_succeeds_with_multiple_token_ids() {
//     let (env, client, _, _, _) = setup_env();
//     let owner = client.owner();
//     let (token_id_1, _) = setup_its_token(&env, &client, &owner, 100);
//     let token_id_2 = client.mock_all_auths().deploy_interchain_token(
//         &owner,
//         &BytesN::random(&env),
//         &TokenMetadata {
//             name: String::from_str(&env, "Token2"),
//             symbol: String::from_str(&env, "TOKEN2"),
//             decimal: 6,
//         },
//         &200,
//         &None,
//     );

//     let new_its_wasm_hash = env.deployer().upload_contract_wasm(NEW_ITS_WASM);
//     let new_token_manager_wasm_hash = env.deployer().upload_contract_wasm(NEW_TOKEN_MANAGER_WASM);
//     let new_interchain_token_wasm_hash = env
//         .deployer()
//         .upload_contract_wasm(NEW_INTERCHAIN_TOKEN_WASM);

//     let current_epoch = 123u64;

//     let token_manager_1 = client.token_manager_address(&token_id_1);
//     let token_manager_2 = client.token_manager_address(&token_id_2);
//     let interchain_token_1 = client.interchain_token_address(&token_id_1);
//     let interchain_token_2 = client.interchain_token_address(&token_id_2);

//     let token_config_1 = TokenIdConfigValue {
//         token_address: interchain_token_1,
//         token_manager: token_manager_1,
//         token_manager_type: TokenManagerType::LockUnlock,
//     };

//     let token_config_2 = TokenIdConfigValue {
//         token_address: interchain_token_2,
//         token_manager: token_manager_2,
//         token_manager_type: TokenManagerType::NativeInterchainToken,
//     };

//     let flow_in_amount_1 = 100i128;
//     let flow_out_amount_1 = 50i128;
//     let flow_in_amount_2 = 200i128;
//     let flow_out_amount_2 = 150i128;

//     env.as_contract(&client.address, || {
//         let flow_key_1 = legacy_storage::FlowKey {
//             token_id: token_id_1.clone(),
//             epoch: current_epoch,
//         };
//         let flow_key_2 = legacy_storage::FlowKey {
//             token_id: token_id_2.clone(),
//             epoch: current_epoch,
//         };

//         legacy_storage::set_flow_in(&env, flow_key_1.clone(), &flow_in_amount_1);
//         legacy_storage::set_flow_out(&env, flow_key_1, &flow_out_amount_1);
//         legacy_storage::set_flow_in(&env, flow_key_2.clone(), &flow_in_amount_2);
//         legacy_storage::set_flow_out(&env, flow_key_2, &flow_out_amount_2);

//         storage::set_token_id_config(&env, token_id_1.clone(), &token_config_1);
//         storage::set_token_id_config(&env, token_id_2.clone(), &token_config_2);
//         storage::set_token_manager_wasm_hash(&env, &new_token_manager_wasm_hash);
//         storage::set_interchain_token_wasm_hash(&env, &new_interchain_token_wasm_hash);
//     });

//     assert_auth!(owner, client.upgrade(&new_its_wasm_hash));

//     let migration_data = CustomMigrationData {
//         new_token_manager_wasm_hash,
//         new_interchain_token_wasm_hash,
//         token_ids: vec![&env, token_id_1.clone(), token_id_2.clone()],
//         current_epoch,
//         upgrader_client: upgrader_client.address,
//         new_version: String::from_str(&env, "1.1.0"),
//     };

//     assert_auth!(owner, client.migrate(&migration_data));

//     assert_eq!(
//         env.as_contract(&client.address, || {
//             storage::flow_in(&env, token_id_1.clone(), current_epoch)
//         }),
//         flow_in_amount_1
//     );
//     assert_eq!(
//         env.as_contract(&client.address, || {
//             storage::flow_out(&env, token_id_1, current_epoch)
//         }),
//         flow_out_amount_1
//     );
//     assert_eq!(
//         env.as_contract(&client.address, || {
//             storage::flow_in(&env, token_id_2.clone(), current_epoch)
//         }),
//         flow_in_amount_2
//     );
//     assert_eq!(
//         env.as_contract(&client.address, || {
//             storage::flow_out(&env, token_id_2, current_epoch)
//         }),
//         flow_out_amount_2
//     );
// }

// #[test]
// fn migrate_succeeds_with_empty_migration_data() {
//     let (env, client, _, _, _) = setup_env();

//     let owner = client.owner();

//     let new_its_wasm_hash = env.deployer().upload_contract_wasm(NEW_ITS_WASM);
//     let new_token_manager_wasm_hash = env.deployer().upload_contract_wasm(NEW_TOKEN_MANAGER_WASM);
//     let new_interchain_token_wasm_hash = env
//         .deployer()
//         .upload_contract_wasm(NEW_INTERCHAIN_TOKEN_WASM);

//     let current_epoch = 123u64;

//     assert_auth!(owner, client.upgrade(&new_its_wasm_hash));

//     let migration_data = CustomMigrationData {
//         new_token_manager_wasm_hash,
//         new_interchain_token_wasm_hash,
//         token_ids: vec![&env],
//         current_epoch,
//         upgrader_client: upgrader_client.address,
//         new_version: String::from_str(&env, "1.1.0"),
//     };

//     assert_auth!(owner, client.migrate(&migration_data));
// }

// #[test]
// fn migrate_with_legacy_flow_data() {
//     let (env, client, _, _, _) = setup_env();
//     let owner = client.owner();
//     let (token_id, _) = setup_its_token(&env, &client, &owner, 100);

//     let new_its_wasm_hash = env.deployer().upload_contract_wasm(NEW_ITS_WASM);
//     let new_token_manager_wasm_hash = env.deployer().upload_contract_wasm(NEW_TOKEN_MANAGER_WASM);
//     let new_interchain_token_wasm_hash = env
//         .deployer()
//         .upload_contract_wasm(NEW_INTERCHAIN_TOKEN_WASM);

//     let current_epoch = 123u64;

//     let token_manager = client.token_manager_address(&token_id);
//     let interchain_token = client.interchain_token_address(&token_id);

//     let token_config = TokenIdConfigValue {
//         token_address: interchain_token,
//         token_manager,
//         token_manager_type: TokenManagerType::LockUnlock,
//     };

//     let flow_in_amount = 100i128;
//     let flow_out_amount = 50i128;

//     env.as_contract(&client.address, || {
//         let flow_key = legacy_storage::FlowKey {
//             token_id: token_id.clone(),
//             epoch: current_epoch,
//         };

//         legacy_storage::set_flow_in(&env, flow_key.clone(), &flow_in_amount);
//         legacy_storage::set_flow_out(&env, flow_key, &flow_out_amount);

//         storage::set_token_id_config(&env, token_id.clone(), &token_config);
//     });

//     assert_auth!(owner, client.upgrade(&new_its_wasm_hash));

//     let migration_data = CustomMigrationData {
//         new_token_manager_wasm_hash: new_token_manager_wasm_hash.clone(),
//         new_interchain_token_wasm_hash: new_interchain_token_wasm_hash.clone(),
//         token_ids: vec![&env, token_id.clone()],
//         current_epoch,
//         upgrader_client: upgrader_client.address,
//         new_version: String::from_str(&env, "1.1.0"),
//     };

//     assert_auth!(owner, client.migrate(&migration_data));

//     assert_eq!(
//         env.as_contract(&client.address, || {
//             storage::flow_in(&env, token_id.clone(), current_epoch)
//         }),
//         flow_in_amount,
//         "flow in value should be migrated correctly"
//     );

//     assert_eq!(
//         env.as_contract(&client.address, || {
//             storage::flow_out(&env, token_id, current_epoch)
//         }),
//         flow_out_amount,
//         "flow out value should be migrated correctly"
//     );

//     assert_eq!(
//         env.as_contract(&client.address, || {
//             storage::token_manager_wasm_hash(&env)
//         }),
//         new_token_manager_wasm_hash,
//         "token manager WASM hash should be updated"
//     );

//     assert_eq!(
//         env.as_contract(&client.address, || {
//             storage::interchain_token_wasm_hash(&env)
//         }),
//         new_interchain_token_wasm_hash,
//         "interchain token WASM hash should be updated"
//     );
// }
