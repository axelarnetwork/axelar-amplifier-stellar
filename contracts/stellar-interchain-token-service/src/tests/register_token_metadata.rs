use stellar_axelar_gas_service::testutils::setup_gas_token;
use stellar_axelar_std::address::AddressExt;
use stellar_axelar_std::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
use stellar_axelar_std::token::StellarAssetClient;
use stellar_axelar_std::{
    assert_auth_err, assert_contract_err, auth_invocation, events, Address, Bytes, IntoVal, String,
    Symbol,
};

use super::utils::setup_env;
use crate::error::ContractError;
use crate::event::TokenMetadataRegisteredEvent;
use crate::types::{HubMessage, Message, RegisterTokenMetadata};

pub const REGISTER_TOKEN_METADATA_EVENT_IDX: i32 = -4;

#[test]
fn register_token_metadata_succeeds() {
    let (env, client, _, gas_service, _) = setup_env();
    let owner = Address::generate(&env);
    let token = &env.register_stellar_asset_contract_v2(owner.clone());
    let spender = Address::generate(&env);
    let gas_token = setup_gas_token(&env, &spender);
    let gas_token_client = gas_token.client(&env);

    StellarAssetClient::new(&env, &token.address())
        .mock_all_auths()
        .mint(&owner, &1000);

    let its_hub_chain = String::from_str(&env, "axelar");
    let its_hub_address = String::from_str(&env, "its_hub_address");
    client.mock_all_auths().set_trusted_chain(&its_hub_chain);

    client.mock_all_auths().register_token_metadata(
        &token.address(),
        &spender,
        &Some(gas_token.clone()),
    );

    goldie::assert!(events::fmt_emitted_event_at_idx::<
        TokenMetadataRegisteredEvent,
    >(&env, REGISTER_TOKEN_METADATA_EVENT_IDX));

    let message = Message::RegisterTokenMetadata(RegisterTokenMetadata {
        token_address: token.address().to_string_bytes(),
        decimals: 7,
    });
    let payload = HubMessage::SendToHub {
        destination_chain: its_hub_chain.clone(),
        message,
    }
    .abi_encode(&env);

    let transfer_auth = auth_invocation!(
        spender,
        gas_token_client.transfer(&spender, gas_service.address.clone(), gas_token.amount)
    );

    let pay_gas_auth = auth_invocation!(
        spender,
        gas_service.pay_gas(
            client.address.clone(),
            its_hub_chain,
            its_hub_address,
            payload,
            &spender,
            gas_token.clone(),
            &Bytes::new(&env)
        ),
        transfer_auth
    );

    let register_token_metadata_auth = auth_invocation!(
        spender,
        client.register_token_metadata(token.address(), spender, Some(gas_token)),
        pay_gas_auth
    );

    assert_eq!(env.auths(), register_token_metadata_auth);
}

#[test]
fn register_token_metadata_fails_when_paused() {
    let (env, client, _, _, _) = setup_env();
    let owner = Address::generate(&env);
    let token = &env.register_stellar_asset_contract_v2(owner.clone());
    let spender = Address::generate(&env);
    let gas_token = setup_gas_token(&env, &spender);

    client.mock_all_auths().pause();

    assert_contract_err!(
        client.try_register_token_metadata(&token.address(), &spender, &Some(gas_token.clone())),
        ContractError::ContractPaused
    );
}

#[test]
fn register_token_metadata_fails_with_invalid_token() {
    let (env, client, _, _, _) = setup_env();
    let spender = Address::generate(&env);
    let gas_token = setup_gas_token(&env, &spender);
    let token_address = Address::generate(&env);

    assert_contract_err!(
        client.mock_all_auths().try_register_token_metadata(
            &token_address,
            &spender,
            &Some(gas_token.clone())
        ),
        ContractError::InvalidTokenAddress
    );
}

#[test]
fn register_token_metadata_fails_with_unauthorized() {
    let (env, client, _, _, _) = setup_env();
    let owner = Address::generate(&env);
    let token = &env.register_stellar_asset_contract_v2(owner.clone());
    let spender = Address::generate(&env);
    let gas_token = setup_gas_token(&env, &spender);

    assert_auth_err!(
        spender,
        client.register_token_metadata(&token.address(), &spender, &Some(gas_token.clone()))
    );
}
