use axelar_gateway::executable::AxelarExecutableInterface;
use axelar_soroban_std::types::Token;
use soroban_sdk::{contractclient, Address, Bytes, BytesN, Env, String};
use soroban_token_sdk::metadata::TokenMetadata;

use crate::{error::ContractError, types::TokenManagerType};

#[allow(dead_code)]
#[contractclient(name = "InterchainTokenServiceClient")]
pub trait InterchainTokenServiceInterface: AxelarExecutableInterface {
    fn chain_name(env: &Env) -> String;

    fn gas_service(env: &Env) -> Address;

    fn interchain_token_wasm_hash(env: &Env) -> BytesN<32>;

    fn its_hub_address(env: &Env) -> String;

    fn its_hub_chain_name(env: &Env) -> String;

    fn is_trusted_chain(env: &Env, chain: String) -> bool;

    fn set_trusted_chain(env: &Env, chain: String) -> Result<(), ContractError>;

    fn remove_trusted_chain(env: &Env, chain: String) -> Result<(), ContractError>;

    fn interchain_token_deploy_salt(env: &Env, deployer: Address, salt: BytesN<32>) -> BytesN<32>;

    fn interchain_token_id(env: &Env, sender: Address, salt: BytesN<32>) -> BytesN<32>;

    fn token_address(env: &Env, token_id: BytesN<32>) -> Result<Address, ContractError>;

    fn token_manager_type(
        env: &Env,
        token_id: BytesN<32>,
    ) -> Result<TokenManagerType, ContractError>;

    fn deploy_interchain_token(
        env: &Env,
        deployer: Address,
        salt: BytesN<32>,
        token_metadata: TokenMetadata,
        initial_supply: i128,
        minter: Option<Address>,
    ) -> Result<BytesN<32>, ContractError>;

    fn deploy_remote_interchain_token(
        env: &Env,
        caller: Address,
        salt: BytesN<32>,
        destination_chain: String,
        gas_token: Token,
    ) -> Result<BytesN<32>, ContractError>;

    fn interchain_transfer(
        env: &Env,
        caller: Address,
        token_id: BytesN<32>,
        destination_chain: String,
        destination_address: Bytes,
        amount: i128,
        metadata: Option<Bytes>,
        gas_token: Token,
    ) -> Result<(), ContractError>;

    fn register_canonical_token(
        env: &Env,
        token_address: Address,
    ) -> Result<BytesN<32>, ContractError>;
}
