use stellar_axelar_gas_service::testutils::setup_gas_token;
use stellar_axelar_std::address::AddressExt;
use stellar_axelar_std::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
use stellar_axelar_std::token::{StellarAssetClient, TokenClient};
use stellar_axelar_std::types::Token;
use stellar_axelar_std::{
    assert_contract_err, auth_invocation, events, Address, Bytes, BytesN, IntoVal, String, Symbol,
};

use super::utils::setup_env;
use crate::error::ContractError;
use crate::event::LinkTokenStartedEvent;
use crate::types::{HubMessage, LinkToken, Message, TokenManagerType};

const LINK_TOKEN_STARTED_WITH_GAS_EVENT_IDX: i32 = -4;
const LINK_TOKEN_STARTED_WITHOUT_GAS_EVENT_IDX: i32 = -2;

const TEST_SALT: [u8; 32] = [1; 32];
const TEST_DESTINATION_CHAIN: &str = "ethereum";
const TEST_DESTINATION_TOKEN_ADDRESS: [u8; 32] = [2; 32];
const TEST_TRANSFER_AMOUNT: i128 = 1000;
const TEST_TRANSFER_DESTINATION_ADDRESS: [u8; 32] = [3; 32];
const TEST_TRANSFER_DESTINATION_CHAIN: &str = "avalanche";

struct LinkTokenTestData {
    deployer: Address,
    token: stellar_axelar_std::testutils::StellarAssetContract,
    salt: BytesN<32>,
    destination_chain: String,
    destination_token_address: Bytes,
}

fn setup_link_token_test_data(env: &stellar_axelar_std::Env) -> LinkTokenTestData {
    let deployer = Address::generate(env);
    let owner = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(owner);
    let salt = BytesN::<32>::from_array(env, &TEST_SALT);
    let destination_chain = String::from_str(env, TEST_DESTINATION_CHAIN);
    let destination_token_address = Bytes::from_array(env, &TEST_DESTINATION_TOKEN_ADDRESS);

    LinkTokenTestData {
        deployer,
        token,
        salt,
        destination_chain,
        destination_token_address,
    }
}

#[test]
fn link_token_succeeds_with_token_manager_type_lock_unlock() {
    let (env, client, _, gas_service, _) = setup_env();
    let test_data = setup_link_token_test_data(&env);
    let LinkTokenTestData {
        deployer,
        token,
        salt,
        destination_chain,
        destination_token_address,
    } = test_data;
    let token_manager_type = TokenManagerType::LockUnlock;

    let gas_token = setup_gas_token(&env, &deployer);

    let token_id = client.linked_token_id(&deployer, &salt);
    client.mock_all_auths().register_custom_token(
        &deployer,
        &salt,
        &token.address(),
        &token_manager_type,
    );

    client
        .mock_all_auths()
        .set_trusted_chain(&destination_chain);

    let its_hub_chain = String::from_str(&env, "axelar");
    let its_hub_address = String::from_str(&env, "its_hub_address");

    let message = Message::LinkToken(LinkToken {
        token_id: token_id.clone(),
        token_manager_type,
        source_token_address: token.address().to_string_bytes(),
        destination_token_address: destination_token_address.clone(),
        params: None,
    });
    let payload = HubMessage::SendToHub {
        destination_chain: destination_chain.clone(),
        message,
    }
    .abi_encode(&env);

    let result_token_id = client.mock_all_auths().link_token(
        &deployer,
        &salt,
        &destination_chain,
        &destination_token_address,
        &token_manager_type,
        &None::<Bytes>,
        &Some(gas_token.clone()),
    );

    assert_eq!(result_token_id, token_id);

    goldie::assert!(events::fmt_emitted_event_at_idx::<LinkTokenStartedEvent>(
        &env,
        LINK_TOKEN_STARTED_WITH_GAS_EVENT_IDX
    ));

    let gas_token_client = gas_token.client(&env);
    let transfer_auth = auth_invocation!(
        deployer,
        gas_token_client.transfer(
            deployer.clone(),
            gas_service.address.clone(),
            gas_token.amount
        )
    );

    let gas_service_auth = auth_invocation!(
        deployer,
        gas_service.pay_gas(
            client.address.clone(),
            its_hub_chain,
            its_hub_address,
            payload,
            deployer.clone(),
            gas_token.clone(),
            Bytes::new(&env)
        ),
        transfer_auth
    );

    let link_token_auth = auth_invocation!(
        deployer,
        client.link_token(
            deployer.clone(),
            salt,
            destination_chain,
            destination_token_address,
            token_manager_type,
            None::<Bytes>,
            Some(gas_token)
        ),
        gas_service_auth
    );

    assert_eq!(env.auths(), link_token_auth);

    let transfer_amount = TEST_TRANSFER_AMOUNT;
    let transfer_destination_address = Bytes::from_array(&env, &TEST_TRANSFER_DESTINATION_ADDRESS);
    let transfer_destination_chain = String::from_str(&env, TEST_TRANSFER_DESTINATION_CHAIN);

    client
        .mock_all_auths()
        .set_trusted_chain(&transfer_destination_chain);

    let stellar_token_client = StellarAssetClient::new(&env, &token.address());
    stellar_token_client
        .mock_all_auths()
        .mint(&deployer, &transfer_amount);

    let token_client = TokenClient::new(&env, &token.address());
    let token_manager_address = client.deployed_token_manager(&token_id);

    let initial_deployer_balance = token_client.balance(&deployer);
    let initial_token_manager_balance = token_client.balance(&token_manager_address);

    client.mock_all_auths().interchain_transfer(
        &deployer,
        &token_id,
        &transfer_destination_chain,
        &transfer_destination_address,
        &transfer_amount,
        &None::<Bytes>,
        &None::<Token>,
    );

    let final_deployer_balance = token_client.balance(&deployer);
    let final_token_manager_balance = token_client.balance(&token_manager_address);

    assert_eq!(
        final_deployer_balance,
        initial_deployer_balance - transfer_amount
    );
    assert_eq!(
        final_token_manager_balance,
        initial_token_manager_balance + transfer_amount
    );
}

#[test]
fn link_token_succeeds_with_token_manager_type_mint_burn() {
    let (env, client, _, gas_service, _) = setup_env();
    let test_data = setup_link_token_test_data(&env);
    let LinkTokenTestData {
        deployer,
        token,
        salt,
        destination_chain,
        destination_token_address,
    } = test_data;
    let token_manager_type = TokenManagerType::MintBurn;
    let gas_token = setup_gas_token(&env, &deployer);

    let token_id = client.linked_token_id(&deployer, &salt);
    client.mock_all_auths().register_custom_token(
        &deployer,
        &salt,
        &token.address(),
        &token_manager_type,
    );

    client
        .mock_all_auths()
        .set_trusted_chain(&destination_chain);

    let its_hub_chain = String::from_str(&env, "axelar");
    let its_hub_address = String::from_str(&env, "its_hub_address");

    let message = Message::LinkToken(LinkToken {
        token_id: token_id.clone(),
        token_manager_type,
        source_token_address: token.address().to_string_bytes(),
        destination_token_address: destination_token_address.clone(),
        params: None,
    });
    let payload = HubMessage::SendToHub {
        destination_chain: destination_chain.clone(),
        message,
    }
    .abi_encode(&env);

    let result_token_id = client.mock_all_auths().link_token(
        &deployer,
        &salt,
        &destination_chain,
        &destination_token_address,
        &token_manager_type,
        &None::<Bytes>,
        &Some(gas_token.clone()),
    );

    assert_eq!(result_token_id, token_id);

    goldie::assert!(events::fmt_emitted_event_at_idx::<LinkTokenStartedEvent>(
        &env,
        LINK_TOKEN_STARTED_WITH_GAS_EVENT_IDX
    ));

    let gas_token_client = gas_token.client(&env);
    let transfer_auth = auth_invocation!(
        deployer,
        gas_token_client.transfer(
            deployer.clone(),
            gas_service.address.clone(),
            gas_token.amount
        )
    );

    let gas_service_auth = auth_invocation!(
        deployer,
        gas_service.pay_gas(
            client.address.clone(),
            its_hub_chain,
            its_hub_address,
            payload,
            deployer.clone(),
            gas_token.clone(),
            Bytes::new(&env)
        ),
        transfer_auth
    );

    let link_token_auth = auth_invocation!(
        deployer,
        client.link_token(
            deployer.clone(),
            salt,
            destination_chain,
            destination_token_address,
            token_manager_type,
            None::<Bytes>,
            Some(gas_token)
        ),
        gas_service_auth
    );

    assert_eq!(env.auths(), link_token_auth);

    let transfer_amount = TEST_TRANSFER_AMOUNT;
    let transfer_destination_address = Bytes::from_array(&env, &TEST_TRANSFER_DESTINATION_ADDRESS);
    let transfer_destination_chain = String::from_str(&env, TEST_TRANSFER_DESTINATION_CHAIN);

    client
        .mock_all_auths()
        .set_trusted_chain(&transfer_destination_chain);

    let stellar_token_client = StellarAssetClient::new(&env, &token.address());
    stellar_token_client
        .mock_all_auths()
        .mint(&deployer, &transfer_amount);

    let token_client = TokenClient::new(&env, &token.address());
    let token_manager_address = client.deployed_token_manager(&token_id);

    let initial_deployer_balance = token_client.balance(&deployer);
    let initial_token_manager_balance = token_client.balance(&token_manager_address);

    client.mock_all_auths().interchain_transfer(
        &deployer,
        &token_id,
        &transfer_destination_chain,
        &transfer_destination_address,
        &transfer_amount,
        &None::<Bytes>,
        &None::<Token>,
    );

    let final_deployer_balance = token_client.balance(&deployer);
    let final_token_manager_balance = token_client.balance(&token_manager_address);

    assert_eq!(
        final_deployer_balance,
        initial_deployer_balance - transfer_amount
    );
    assert_eq!(final_token_manager_balance, initial_token_manager_balance);
}

#[test]
fn link_token_succeeds_without_gas_token() {
    let (env, client, _, _, _) = setup_env();
    let test_data = setup_link_token_test_data(&env);
    let LinkTokenTestData {
        deployer,
        token,
        salt,
        destination_chain,
        destination_token_address,
    } = test_data;
    let token_manager_type = TokenManagerType::LockUnlock;
    let link_params: Option<Bytes> = None;
    let gas_token: Option<Token> = None;

    let expected_id = client.linked_token_id(&deployer, &salt);
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
        &link_params,
        &gas_token,
    );

    assert_eq!(token_id, expected_id);

    goldie::assert!(events::fmt_emitted_event_at_idx::<LinkTokenStartedEvent>(
        &env,
        LINK_TOKEN_STARTED_WITHOUT_GAS_EVENT_IDX
    ));

    let link_token_auth = auth_invocation!(
        deployer,
        client.link_token(
            deployer,
            salt,
            destination_chain,
            destination_token_address,
            token_manager_type,
            link_params,
            gas_token
        )
    );

    assert_eq!(env.auths(), link_token_auth);
}

#[test]
fn link_token_fails_when_paused() {
    let (env, client, _, _, _) = setup_env();
    let test_data = setup_link_token_test_data(&env);
    let LinkTokenTestData {
        deployer,
        token: _,
        salt,
        destination_chain,
        destination_token_address,
    } = test_data;
    let token_manager_type = TokenManagerType::LockUnlock;
    let link_params: Option<Bytes> = None;
    let gas_token: Option<Token> = None;

    client.mock_all_auths().pause();

    assert_contract_err!(
        client.try_link_token(
            &deployer,
            &salt,
            &destination_chain,
            &destination_token_address,
            &token_manager_type,
            &link_params,
            &gas_token
        ),
        ContractError::ContractPaused
    );
}

#[test]
fn link_token_fails_with_native_interchain_token_type() {
    let (env, client, _, _, _) = setup_env();
    let test_data = setup_link_token_test_data(&env);
    let LinkTokenTestData {
        deployer,
        token,
        salt,
        destination_chain,
        destination_token_address,
    } = test_data;
    let token_manager_type = TokenManagerType::NativeInterchainToken;
    let link_params: Option<Bytes> = None;

    let token_id = client.linked_token_id(&deployer, &salt);
    let message = Message::LinkToken(LinkToken {
        token_id,
        token_manager_type,
        source_token_address: token.address().to_string_bytes(),
        destination_token_address: destination_token_address.clone(),
        params: link_params.clone(),
    });
    let payload = HubMessage::SendToHub {
        destination_chain: destination_chain.clone(),
        message,
    };
    let err = payload.abi_encode(&env).unwrap_err();
    assert_eq!(err, ContractError::InvalidTokenManagerType);
}

#[test]
fn link_token_fails_with_invalid_destination_chain() {
    let (env, client, _, _, _) = setup_env();
    let test_data = setup_link_token_test_data(&env);
    let LinkTokenTestData {
        deployer,
        token,
        salt,
        destination_chain: _,
        destination_token_address,
    } = test_data;
    let destination_chain = client.chain_name();
    let token_manager_type = TokenManagerType::LockUnlock;
    let link_params: Option<Bytes> = None;
    let gas_token: Option<Token> = None;

    let _ = client.linked_token_id(&deployer, &salt);
    client.mock_all_auths().register_custom_token(
        &deployer,
        &salt,
        &token.address(),
        &token_manager_type,
    );

    assert_contract_err!(
        client.mock_all_auths().try_link_token(
            &deployer,
            &salt,
            &destination_chain,
            &destination_token_address,
            &token_manager_type,
            &link_params,
            &gas_token
        ),
        ContractError::InvalidDestinationChain
    );
}

#[test]
fn link_token_fails_with_invalid_token_id() {
    let (env, client, _, _, _) = setup_env();
    let test_data = setup_link_token_test_data(&env);
    let LinkTokenTestData {
        deployer,
        token: _,
        salt,
        destination_chain,
        destination_token_address,
    } = test_data;
    let token_manager_type = TokenManagerType::LockUnlock;
    let link_params: Option<Bytes> = None;
    let gas_token: Option<Token> = None;

    assert_contract_err!(
        client.mock_all_auths().try_link_token(
            &deployer,
            &salt,
            &destination_chain,
            &destination_token_address,
            &token_manager_type,
            &link_params,
            &gas_token
        ),
        ContractError::InvalidTokenId
    );
}

#[test]
fn link_token_fails_with_untrusted_chain() {
    let (env, client, _, _, _) = setup_env();
    let test_data = setup_link_token_test_data(&env);
    let LinkTokenTestData {
        deployer,
        token,
        salt,
        destination_chain,
        destination_token_address,
    } = test_data;
    let token_manager_type = TokenManagerType::LockUnlock;
    let link_params: Option<Bytes> = None;
    let gas_token: Option<Token> = None;

    client.mock_all_auths().register_custom_token(
        &deployer,
        &salt,
        &token.address(),
        &token_manager_type,
    );

    assert_contract_err!(
        client.mock_all_auths().try_link_token(
            &deployer,
            &salt,
            &destination_chain,
            &destination_token_address,
            &token_manager_type,
            &link_params,
            &gas_token
        ),
        ContractError::UntrustedChain
    );
}

#[test]
fn link_token_fails_with_empty_destination_token_address() {
    let (env, client, _, _, _) = setup_env();

    let test_data = setup_link_token_test_data(&env);
    let LinkTokenTestData {
        deployer,
        token: _,
        salt,
        destination_chain,
        destination_token_address: _,
    } = test_data;
    let token_manager_type = TokenManagerType::LockUnlock;
    let link_params: Option<Bytes> = None;
    let gas_token: Option<Token> = None;

    assert_contract_err!(
        client.mock_all_auths().try_link_token(
            &deployer,
            &salt,
            &destination_chain,
            &Bytes::new(&env),
            &token_manager_type,
            &link_params,
            &gas_token
        ),
        ContractError::InvalidDestinationTokenAddress
    );
}

#[test]
fn linked_token_id_derivation() {
    let (env, client, _, _, _) = setup_env();
    let deployer = Address::generate(&env);
    let salt = BytesN::<32>::from_array(&env, &TEST_SALT);

    let token_id = client.linked_token_id(&deployer, &salt);

    goldie::assert!(hex::encode(token_id.to_array()));
}
