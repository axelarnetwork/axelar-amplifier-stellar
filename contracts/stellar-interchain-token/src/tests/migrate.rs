use stellar_axelar_std::interfaces::CustomMigratableInterface;
use stellar_axelar_std::{assert_auth, assert_ok};

use crate::migrate::CustomMigrationData;
use crate::tests::testutils::{setup_env, TestConfig, INITIAL_TOTAL_SUPPLY};
use crate::InterchainToken;

const NEW_WASM: &[u8] = include_bytes!("testdata/stellar_interchain_token.optimized.wasm");

#[test]
fn migrate_succeeds() {
    let TestConfig { env, owner, client } = setup_env();

    let new_wasm_hash = env.deployer().upload_contract_wasm(NEW_WASM);

    assert_auth!(owner, client.upgrade(&new_wasm_hash));

    let total_supply = INITIAL_TOTAL_SUPPLY;
    let migration_data = CustomMigrationData { total_supply };

    assert_auth!(owner, client.migrate(&migration_data));

    assert_eq!(client.total_supply(), total_supply);
}

#[test]
fn coverage_migrate_succeeds() {
    let TestConfig { env, owner, client } = setup_env();

    let new_wasm_hash = env.deployer().upload_contract_wasm(NEW_WASM);

    assert_auth!(owner, client.upgrade(&new_wasm_hash));

    let total_supply = INITIAL_TOTAL_SUPPLY;
    let migration_data = CustomMigrationData { total_supply };

    env.as_contract(&client.address, || {
        assert_ok!(InterchainToken::__migrate(&env, migration_data));
    });
}
