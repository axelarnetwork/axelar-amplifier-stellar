use stellar_axelar_std::interfaces::CustomMigratableInterface;
use stellar_axelar_std::{assert_auth, assert_ok};

use crate::{
    tests::testutils::{setup_env, TestConfig},
    TokenManager,
};

const NEW_WASM: &[u8] = include_bytes!("testdata/stellar_token_manager.optimized.wasm");

#[test]
fn migrate_succeeds() {
    let TestConfig { env, owner, client } = setup_env();

    let new_wasm_hash = env.deployer().upload_contract_wasm(NEW_WASM);

    assert_auth!(owner, client.upgrade(&new_wasm_hash));

    client.mock_all_auths().migrate(&());

    env.as_contract(&client.address, || {
        assert_ok!(TokenManager::__migrate(&env, ()));
    });
}
