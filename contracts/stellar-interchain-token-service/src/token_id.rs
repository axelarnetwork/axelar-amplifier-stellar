use stellar_axelar_std::address::AddressExt;
use stellar_axelar_std::xdr::ToXdr;
use stellar_axelar_std::{Address, BytesN, Env};

use crate::error::ContractError;
use crate::storage;

const PREFIX_CANONICAL_TOKEN_SALT: &str = "canonical-token-salt";
const PREFIX_INTERCHAIN_TOKEN_SALT: &str = "interchain-token-salt";
const PREFIX_CUSTOM_TOKEN_SALT: &str = "custom-token-salt";
/// This prefix is used along with a salt to generate the token ID
const PREFIX_TOKEN_ID: &str = "its-interchain-token-id";

/// A newtype wrapper that enforces token registration checks have been performed.
/// This prevents calling `deploy_token_manager` without first checking that the token ID
/// is not already registered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnregisteredTokenId(BytesN<32>);

impl UnregisteredTokenId {
    /// Create a new UnregisteredTokenId. This should only be called after verifying
    /// that the token ID is not already registered.
    const fn new(token_id: BytesN<32>) -> Self {
        Self(token_id)
    }
}

impl From<UnregisteredTokenId> for BytesN<32> {
    fn from(unregistered_token_id: UnregisteredTokenId) -> Self {
        unregistered_token_id.0
    }
}

fn token_id(env: &Env, deploy_salt: BytesN<32>) -> BytesN<32> {
    env.crypto()
        .keccak256(&(PREFIX_TOKEN_ID, Address::zero(env), deploy_salt).to_xdr(env))
        .into()
}

fn canonical_token_deploy_salt(
    env: &Env,
    chain_name_hash: BytesN<32>,
    token_address: Address,
) -> BytesN<32> {
    env.crypto()
        .keccak256(&(PREFIX_CANONICAL_TOKEN_SALT, chain_name_hash, token_address).to_xdr(env))
        .into()
}

pub fn canonical_interchain_token_id(
    env: &Env,
    chain_name_hash: BytesN<32>,
    token_address: Address,
) -> BytesN<32> {
    token_id(
        env,
        canonical_token_deploy_salt(env, chain_name_hash, token_address),
    )
}

fn linked_token_deploy_salt(
    env: &Env,
    chain_name_hash: BytesN<32>,
    deployer: Address,
    salt: BytesN<32>,
) -> BytesN<32> {
    env.crypto()
        .keccak256(&(PREFIX_CUSTOM_TOKEN_SALT, chain_name_hash, deployer, salt).to_xdr(env))
        .into()
}

pub fn linked_token_id(
    env: &Env,
    chain_name_hash: BytesN<32>,
    deployer: Address,
    salt: BytesN<32>,
) -> BytesN<32> {
    let deploy_salt = linked_token_deploy_salt(env, chain_name_hash.clone(), deployer, salt);
    interchain_token_id(env, chain_name_hash, Address::zero(env), deploy_salt)
}

fn interchain_token_deploy_salt(
    env: &Env,
    chain_name_hash: BytesN<32>,
    deployer: Address,
    salt: BytesN<32>,
) -> BytesN<32> {
    env.crypto()
        .keccak256(
            &(
                PREFIX_INTERCHAIN_TOKEN_SALT,
                chain_name_hash,
                deployer,
                salt,
            )
                .to_xdr(env),
        )
        .into()
}

pub fn interchain_token_id(
    env: &Env,
    chain_name_hash: BytesN<32>,
    deployer: Address,
    salt: BytesN<32>,
) -> BytesN<32> {
    token_id(
        env,
        interchain_token_deploy_salt(env, chain_name_hash, deployer, salt),
    )
}

pub fn ensure_token_not_registered(
    env: &Env,
    token_id: BytesN<32>,
) -> Result<UnregisteredTokenId, ContractError> {
    stellar_axelar_std::ensure!(
        storage::try_token_id_config(env, token_id.clone()).is_none(),
        ContractError::TokenAlreadyRegistered
    );

    Ok(UnregisteredTokenId::new(token_id))
}
