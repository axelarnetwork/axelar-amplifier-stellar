use stellar_axelar_std::token::TokenClient;
use stellar_axelar_std::{Address, Env};
use stellar_token_manager::TokenManagerClient;

use crate::error::ContractError;
use crate::storage::TokenIdConfigValue;
use crate::token_manager::TokenManagerClientExt;
use crate::types::TokenManagerType;

pub fn take_token(
    env: &Env,
    sender: &Address,
    TokenIdConfigValue {
        token_address,
        token_manager,
        token_manager_type,
    }: TokenIdConfigValue,
    amount: i128,
) -> Result<(), ContractError> {
    let token = TokenClient::new(env, &token_address);

    match token_manager_type {
        TokenManagerType::NativeInterchainToken => token.burn(sender, &amount),
        TokenManagerType::LockUnlock => token.transfer(sender, &token_manager, &amount),
        TokenManagerType::MintBurn => token.burn(sender, &amount),
    }

    Ok(())
}

pub fn give_token(
    env: &Env,
    recipient: &Address,
    TokenIdConfigValue {
        token_address,
        token_manager,
        token_manager_type,
    }: TokenIdConfigValue,
    amount: i128,
) -> Result<(), ContractError> {
    let token_manager_client = TokenManagerClient::new(env, &token_manager);

    match token_manager_type {
        TokenManagerType::NativeInterchainToken => {
            token_manager_client.mint(env, &token_address, recipient, amount)
        }
        TokenManagerType::LockUnlock => {
            token_manager_client.transfer(env, &token_address, recipient, amount)
        }
        TokenManagerType::MintBurn => {
            if stellar_interchain_token::InterchainTokenClient::new(&env, &token_address)
                .try_is_minter(&token_manager)
                .is_ok()
            {
                token_manager_client.mint_from(env, &token_address, recipient, amount);
            } else {
                token_manager_client.mint(env, &token_address, recipient, amount);
            }
        }
    }

    Ok(())
}

pub fn test_execute(
    env: &Env,
    recipient: &Address,
    token_address: &Address,
    token_manager: &Address,
    amount: i128,
) -> Result<(), ContractError> {
    let token_manager_client = TokenManagerClient::new(env, &token_manager);

    if stellar_interchain_token::InterchainTokenClient::new(&env, &token_address)
        .try_is_minter(&token_manager)
        .is_ok()
    {
        token_manager_client.mint_from(env, &token_address, recipient, amount);
    } else {
        token_manager_client.mint(env, &token_address, recipient, amount);
    }

    Ok(())
}
