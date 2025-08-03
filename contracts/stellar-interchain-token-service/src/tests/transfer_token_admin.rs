use stellar_axelar_gas_service::testutils::setup_gas_token;
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::token::StellarAssetClient;
use stellar_axelar_std::{assert_auth, assert_auth_err, Address, Bytes, BytesN, String};

use super::utils::setup_env;
use crate::testutils::setup_its_token;
use crate::types::TokenManagerType;

#[test]
fn transfer_token_admin_succeeds_with_mint_burn_token_manager_type() {
    let (env, client, _gateway, _gas_service, _signers) = setup_env();
    let owner = client.owner();
    let deployer = Address::generate(&env);
    let new_admin = Address::generate(&env);
    let token_owner = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(token_owner.clone());
    let salt = BytesN::<32>::from_array(&env, &[1; 32]);
    let destination_chain = String::from_str(&env, "ethereum");
    let destination_token_address = Bytes::from_array(&env, &[2; 32]);
    let token_manager_type = TokenManagerType::MintBurn;
    let gas_token = setup_gas_token(&env, &deployer);

    client.mock_all_auths().register_custom_token(
        &deployer,
        &salt,
        &token.address(),
        &token_manager_type,
    );

    client
        .mock_all_auths()
        .set_trusted_chain(&destination_chain);

    let token_id = client.mock_all_auths().link_token(
        &deployer,
        &salt,
        &destination_chain,
        &destination_token_address,
        &token_manager_type,
        &None::<Bytes>,
        &Some(gas_token),
    );

    let token_address = client.registered_token_address(&token_id);
    let token_client = StellarAssetClient::new(&env, &token_address);
    assert_eq!(token_client.admin(), token_owner);

    let token_manager = client.deployed_token_manager(&token_id);
    assert_auth!(token_owner.clone(), token_client.set_admin(&token_manager));
    assert_eq!(token_client.admin(), token_manager);

    assert_auth!(
        owner.clone(),
        client.transfer_token_admin(&token_id, &new_admin)
    );
    assert_eq!(token_client.admin(), new_admin);
}

#[test]
fn transfer_token_admin_fails_with_lock_unlock_token_manager_type() {
    let (env, client, _gateway, _gas_service, _signers) = setup_env();
    let owner = client.owner();
    let new_admin = Address::generate(&env);
    let deployer = Address::generate(&env);
    let token_manager_type = TokenManagerType::LockUnlock;

    let salt = BytesN::<32>::from_array(&env, &[1; 32]);
    let token = env.register_stellar_asset_contract_v2(deployer.clone());

    let token_id = client.mock_all_auths().register_custom_token(
        &deployer,
        &salt,
        &token.address(),
        &token_manager_type,
    );

    assert_auth_err!(owner, client.transfer_token_admin(&token_id, &new_admin));
}

#[test]
fn transfer_token_admin_fails_with_native_interchain_token_manager_type() {
    let (env, client, _gateway, _gas_service, _signers) = setup_env();
    let owner = client.owner();
    let new_admin = Address::generate(&env);
    let deployer = Address::generate(&env);

    let (token_id, _) = setup_its_token(&env, &client, &deployer, 1000);

    assert_auth_err!(owner, client.transfer_token_admin(&token_id, &new_admin));
}

#[test]
fn transfer_token_admin_fails_with_non_owner() {
    let (env, client, _gateway, _gas_service, _signers) = setup_env();
    let deployer = Address::generate(&env);
    let non_owner = Address::generate(&env);
    let new_admin = Address::generate(&env);

    let (token_id, _) = setup_its_token(&env, &client, &deployer, 1000);

    assert_auth_err!(
        non_owner,
        client.transfer_token_admin(&token_id, &new_admin)
    );
}

#[test]
fn transfer_token_admin_fails_with_invalid_token_id() {
    let (env, client, _gateway, _gas_service, _signers) = setup_env();
    let owner = client.owner();
    let new_admin = Address::generate(&env);

    let invalid_token_id = BytesN::<32>::from_array(&env, &[0u8; 32]);

    assert_auth_err!(
        owner,
        client.transfer_token_admin(&invalid_token_id, &new_admin)
    );
}
