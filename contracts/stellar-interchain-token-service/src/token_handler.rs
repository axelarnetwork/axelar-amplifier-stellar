use stellar_axelar_std::token::TokenClient;
use stellar_axelar_std::{Address, Env};
use stellar_interchain_token::InterchainTokenClient;
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
    let token_manager = TokenManagerClient::new(env, &token_manager);

    match token_manager_type {
        TokenManagerType::NativeInterchainToken => {
            token_manager.mint_from(env, &token_address, recipient, amount)
        }
        TokenManagerType::LockUnlock => {
            token_manager.transfer(env, &token_address, recipient, amount)
        }
        TokenManagerType::MintBurn => token_manager.mint(env, &token_address, recipient, amount),
    }

    Ok(())
}

/// Prepares a token manager after it is deployed.
/// This function handles the post-deployment setup based on the token manager type.
///
/// # Arguments
/// * `token_manager_type` - The type of token manager that was deployed
/// * `token_manager` - The address of the deployed token manager
/// * `token_address` - The address of the token contract
pub fn post_token_manager_deploy(
    env: &Env,
    token_manager_type: TokenManagerType,
    token_manager: Address,
    token_address: Address,
) {
    match token_manager_type {
        // For native interchain token managers, we add the token manager as an additional minter.
        TokenManagerType::NativeInterchainToken => {
            let interchain_token_client = InterchainTokenClient::new(env, &token_address);
            // Check if token_manager is already a minter before adding to avoid MinterAlreadyExists error
            if !interchain_token_client.is_minter(&token_manager) {
                interchain_token_client.add_minter(&token_manager);
            }
        }
        // For lock/unlock token managers, no additional setup is required due to Stellar's
        // account abstraction, which eliminates the need for ERC20-like approvals used on EVM chains.
        // The token manager can directly transfer tokens as needed.
        TokenManagerType::LockUnlock => {}
        // For mint/burn token managers, the user needs to grant mint permission to the token manager.
        // Stellar Classic Assets require setting the token manager as the admin to allow minting the token.
        // whereas Stellar custom tokens can add the token manager as an additional minter,
        // this requires implementing support to allow the token manager to call `mint`.
        TokenManagerType::MintBurn => {}
    }
}
