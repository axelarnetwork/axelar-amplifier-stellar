mod utils;

use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
use soroban_sdk::Address;
use soroban_sdk::{BytesN, Env, String};
use upgrader::contract::{Upgrader, UpgraderClient};
use utils::{DataKey, DummyContract, DummyContractClient};

const WASM_AFTER_UPGRADE: &[u8] = include_bytes!("testdata/dummy.wasm");

#[test]
fn upgrade_and_migrate_are_atomic() {
    let env = Env::default();

    let owner = Address::generate(&env);
    let contract_address = env.register(DummyContract, (&owner,));
    let upgrader_address = env.register(Upgrader, ());

    let dummy_client = DummyContractClient::new(&env, &contract_address);
    let original_version: String = dummy_client.version();
    assert_eq!(original_version, String::from_str(&env, "0.1.0"));

    let hash_after_upgrade = env.deployer().upload_contract_wasm(WASM_AFTER_UPGRADE);
    let expected_data = String::from_str(&env, "migration successful");
    let expected_version = String::from_str(&env, "0.2.0");

    let (upgrade_auth, migrate_auth) =
        build_invocation_auths(&env, &contract_address, &hash_after_upgrade, &expected_data);

    // add the owner to the set of authenticated addresses
    env.mock_auths(&[
        MockAuth {
            address: &owner,
            invoke: &upgrade_auth,
        },
        MockAuth {
            address: &owner,
            invoke: &migrate_auth,
        },
    ]);

    UpgraderClient::new(&env, &upgrader_address).upgrade(
        &contract_address,
        &expected_version,
        &hash_after_upgrade,
        &soroban_sdk::vec![&env, expected_data.to_val()],
    );

    // ensure new version is set correctly
    let upgraded_version: String = dummy_client.version();
    assert_eq!(upgraded_version, expected_version);

    // ensure migration was successful
    env.as_contract(&contract_address, || {
        let data: String = env.storage().instance().get(&DataKey::Data).unwrap();
        assert_eq!(data, expected_data);
    });
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn upgrade_fails_if_caller_is_authenticated_but_not_owner() {
    let env = Env::default();

    let owner = Address::generate(&env);
    let contract_address = env.register(DummyContract, (&owner,));
    let upgrader_address = env.register(Upgrader, ());

    let hash_after_upgrade = env.deployer().upload_contract_wasm(WASM_AFTER_UPGRADE);
    let expected_data = String::from_str(&env, "migration successful");
    let expected_version = String::from_str(&env, "0.2.0");

    let (upgrade_auth, migrate_auth) =
        build_invocation_auths(&env, &contract_address, &hash_after_upgrade, &expected_data);

    // add the caller to the set of authenticated addresses
    let caller = Address::generate(&env);
    env.mock_auths(&[
        MockAuth {
            address: &caller,
            invoke: &upgrade_auth,
        },
        MockAuth {
            address: &caller,
            invoke: &migrate_auth,
        },
    ]);

    // should panic: caller is authenticated, but not the owner
    UpgraderClient::new(&env, &upgrader_address).upgrade(
        &contract_address,
        &expected_version,
        &hash_after_upgrade,
        &soroban_sdk::vec![&env, expected_data.to_val()],
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn upgrade_fails_if_correct_owner_is_not_authenticated_for_full_invocation_tree() {
    let env = Env::default();

    let owner = Address::generate(&env);
    let contract_address = env.register(DummyContract, (&owner,));

    let upgrader_address = env.register(Upgrader, ());

    let hash_after_upgrade = env.deployer().upload_contract_wasm(WASM_AFTER_UPGRADE);
    let expected_data = String::from_str(&env, "migration successful");
    let expected_version = String::from_str(&env, "0.2.0");

    let (upgrade_auth, migrate_auth) =
        build_invocation_auths(&env, &contract_address, &hash_after_upgrade, &expected_data);

    // only add the owner to the set of authenticated addresses for the upgrade function, and the caller for the migrate function
    let caller = Address::generate(&env);
    env.mock_auths(&[
        MockAuth {
            address: &owner,
            invoke: &upgrade_auth,
        },
        MockAuth {
            address: &caller,
            invoke: &migrate_auth,
        },
    ]);

    UpgraderClient::new(&env, &upgrader_address).upgrade(
        &contract_address,
        &expected_version,
        &hash_after_upgrade,
        &soroban_sdk::vec![&env, expected_data.to_val()],
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn upgrade_fails_if_nobody_is_authenticated() {
    let env = Env::default();

    let owner = Address::generate(&env);
    let contract_address = env.register(DummyContract, (&owner,));
    let upgrader_address = env.register(Upgrader, ());

    let hash_after_upgrade = env.deployer().upload_contract_wasm(WASM_AFTER_UPGRADE);
    let expected_data = String::from_str(&env, "migration successful");
    let expected_version = String::from_str(&env, "0.2.0");

    UpgraderClient::new(&env, &upgrader_address).upgrade(
        &contract_address,
        &expected_version,
        &hash_after_upgrade,
        &soroban_sdk::vec![&env, expected_data.to_val()],
    );
}

fn build_invocation_auths<'a>(
    env: &Env,
    contract_address: &'a Address,
    hash_after_upgrade: &'a BytesN<32>,
    expected_data: &'a String,
) -> (MockAuthInvoke<'a>, MockAuthInvoke<'a>) {
    let upgrade = MockAuthInvoke {
        contract: contract_address,
        fn_name: "upgrade",
        args: soroban_sdk::vec![&env, hash_after_upgrade.to_val()],
        sub_invokes: &[],
    };
    let migrate = MockAuthInvoke {
        contract: contract_address,
        fn_name: "migrate",
        args: soroban_sdk::vec![&env, expected_data.to_val()],
        sub_invokes: &[],
    };
    (upgrade, migrate)
}
