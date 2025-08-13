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
        // For NativeInterchainToken and MintBurn, burn tokens directly from the sender
        TokenManagerType::NativeInterchainToken | TokenManagerType::MintBurn => {
            token.burn(sender, &amount)
        }

        // In EVM, `MintBurnFrom` would require explicit approval (allowance) before burning.
        // However, in Stellar's account abstraction model, when a user signs a transaction,
        // they can authorize all sub-invocations within that transaction by default.
        // Therefore, we can directly burn from the sender without requiring a separate approval,
        // as the user has already authorized this action by signing the transaction.
        TokenManagerType::MintBurnFrom => token.burn(sender, &amount),

        // For LockUnlock, transfer tokens from the sender to the token manager
        TokenManagerType::LockUnlock => token.transfer(sender, &token_manager, &amount),
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
        // For NativeInterchainToken and MintBurnFrom,
        // `mint_from` interface allows the token to potentially have multiple minters, one of them being the token manager to mint tokens for ITS
        TokenManagerType::NativeInterchainToken | TokenManagerType::MintBurnFrom => {
            token_manager.mint_from(env, &token_address, recipient, amount)
        }

        // Transfer previously locked tokens from the token manager to the recipient
        TokenManagerType::LockUnlock => {
            token_manager.transfer(env, &token_address, recipient, amount)
        }

        // For MintBurn, use direct mint where the token manager mints new tokens
        // This assumes the token manager has minting privileges on the token contract
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
        // For MintBurnFrom token managers, the user needs to add the token manager as a minter.
        // Stellar Custom Tokens could add the token manager as an additional minter.
        TokenManagerType::MintBurnFrom => {}
        // For LockUnlock token managers, no additional setup is required due to Stellar's
        // account abstraction, which eliminates the need for ERC20-like approvals used on EVM chains.
        // The token manager can directly transfer tokens as needed.
        TokenManagerType::LockUnlock => {}
        // For MintBurn token managers, the user needs to grant mint permission to the token manager
        // Stellar Classic Assets require setting the token manager as the admin to allow minting the token,
        // whereas Stellar Custom Tokens need to add the token manager as a minter.
        TokenManagerType::MintBurn => {}
    }
}
