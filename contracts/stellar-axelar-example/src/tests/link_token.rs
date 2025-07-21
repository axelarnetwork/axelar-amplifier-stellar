use stellar_axelar_gas_service::testutils::{setup_gas_service, setup_gas_token};
use stellar_axelar_gateway::testutils::{
    generate_proof, get_approve_hash, setup_gateway, TestSignerSet,
};
use stellar_axelar_gateway::types::Message;
use stellar_axelar_gateway::AxelarGatewayClient;
use stellar_axelar_std::address::AddressExt;
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::token::{self, StellarAssetClient};
use stellar_axelar_std::{assert_ok, vec, Address, Bytes, BytesN, Env, String};
use stellar_interchain_token::InterchainTokenClient;
use stellar_interchain_token_service::testutils::setup_its;
use stellar_interchain_token_service::types::TokenManagerType;
use stellar_interchain_token_service::InterchainTokenServiceClient;

use crate::{AxelarExample, AxelarExampleClient};

const SOURCE_CHAIN_NAME: &str = "source";
const DESTINATION_CHAIN_NAME: &str = "destination";

struct TestConfig<'a> {
    signers: TestSignerSet,
    gateway: AxelarGatewayClient<'a>,
    its: InterchainTokenServiceClient<'a>,
    app: AxelarExampleClient<'a>,
}

fn setup_app<'a>(env: &Env, chain_name: String) -> TestConfig<'a> {
    let (signers, gateway) = setup_gateway(env, 0, 5);
    let gas_service = setup_gas_service(env);
    let its = setup_its(env, &gateway, &gas_service, Some(chain_name));
    let app = env.register(
        AxelarExample,
        (&gateway.address, &gas_service.address, &its.address),
    );
    let app = AxelarExampleClient::new(env, &app);

    TestConfig {
        signers,
        gateway,
        its,
        app,
    }
}

struct LinkTokenTestEnv<'a> {
    env: Env,
    deployer: Address,
    source_chain: String,
    destination_chain: String,
    source_config: TestConfig<'a>,
    destination_config: TestConfig<'a>,
    hub_chain: String,
    hub_address: String,
    recipient: Address,
    transfer_amount: i128,
    initial_supply: i128,
    gas_token: stellar_axelar_std::types::Token,
}

fn setup_link_token_test_env() -> LinkTokenTestEnv<'static> {
    let env = Env::default();
    let deployer = Address::generate(&env);
    let recipient = Address::generate(&env);
    let transfer_amount = 1000;
    let initial_supply = 3000;

    let source_chain = String::from_str(&env, SOURCE_CHAIN_NAME);
    let source_config = setup_app(&env, source_chain.clone());
    let destination_chain = String::from_str(&env, DESTINATION_CHAIN_NAME);
    let destination_config = setup_app(&env, destination_chain.clone());
    let hub_chain = source_config.its.its_hub_chain_name();
    let hub_address = source_config.its.its_hub_address();

    let gas_token = setup_gas_token(&env, &deployer);
    StellarAssetClient::new(&env, &gas_token.address)
        .mock_all_auths()
        .mint(&deployer, &3);

    source_config
        .its
        .mock_all_auths()
        .set_trusted_chain(&destination_chain);
    destination_config
        .its
        .mock_all_auths()
        .set_trusted_chain(&source_chain);

    LinkTokenTestEnv {
        env,
        deployer,
        source_chain,
        destination_chain,
        source_config,
        destination_config,
        hub_chain,
        hub_address,
        recipient,
        transfer_amount,
        initial_supply,
        gas_token,
    }
}

struct StellarAssetSetup {
    source_token_address: Address,
    destination_token_address: Address,
    token_id: BytesN<32>,
    salt: BytesN<32>,
}

fn setup_stellar_classic_assets(
    test_env: &LinkTokenTestEnv,
    source_manager_type: TokenManagerType,
) -> StellarAssetSetup {
    let source_token = test_env
        .env
        .register_stellar_asset_contract_v2(test_env.deployer.clone());
    let source_token_client = StellarAssetClient::new(&test_env.env, &source_token.address());
    source_token_client
        .mock_all_auths()
        .mint(&test_env.deployer, &test_env.initial_supply);

    let destination_token = test_env
        .env
        .register_stellar_asset_contract_v2(test_env.deployer.clone());
    let destination_token_address = destination_token.address();

    test_env
        .source_config
        .its
        .mock_all_auths()
        .register_token_metadata(
            &source_token.address(),
            &test_env.deployer,
            &Some(test_env.gas_token.clone()),
        );

    let salt = BytesN::<32>::from_array(&test_env.env, &[1; 32]);
    let token_id = test_env
        .source_config
        .its
        .mock_all_auths()
        .register_custom_token(
            &test_env.deployer,
            &salt,
            &source_token.address(),
            &source_manager_type,
        );

    StellarAssetSetup {
        source_token_address: source_token.address(),
        destination_token_address,
        token_id,
        salt,
    }
}

struct InterchainTokenSetup {
    source_token_address: Address,
    destination_token_address: Address,
    token_id: BytesN<32>,
    salt: BytesN<32>,
}

fn setup_interchain_tokens(
    test_env: &LinkTokenTestEnv,
    source_manager_type: TokenManagerType,
) -> InterchainTokenSetup {
    let source_token_salt = BytesN::<32>::from_array(&test_env.env, &[1; 32]);
    let source_token_metadata = soroban_token_sdk::metadata::TokenMetadata {
        name: String::from_str(&test_env.env, "SourceToken"),
        symbol: String::from_str(&test_env.env, "SRC"),
        decimal: 18,
    };

    let source_token_id = test_env
        .source_config
        .its
        .mock_all_auths()
        .deploy_interchain_token(
            &test_env.deployer,
            &source_token_salt,
            &source_token_metadata,
            &0,
            &Some(test_env.deployer.clone()),
        );
    let source_token_address = test_env
        .source_config
        .its
        .registered_token_address(&source_token_id);

    let salt = BytesN::<32>::from_array(&test_env.env, &[2; 32]);
    let token_id = test_env
        .source_config
        .its
        .mock_all_auths()
        .register_custom_token(
            &test_env.deployer,
            &salt,
            &source_token_address,
            &source_manager_type,
        );

    let destination_token_salt = BytesN::<32>::from_array(&test_env.env, &[3; 32]);
    let destination_token_metadata = soroban_token_sdk::metadata::TokenMetadata {
        name: String::from_str(&test_env.env, "DestToken"),
        symbol: String::from_str(&test_env.env, "DEST"),
        decimal: 18,
    };

    let destination_token_id = test_env
        .destination_config
        .its
        .mock_all_auths()
        .deploy_interchain_token(
            &test_env.deployer,
            &destination_token_salt,
            &destination_token_metadata,
            &test_env.initial_supply,
            &Some(test_env.deployer.clone()),
        );
    let destination_token_address = test_env
        .destination_config
        .its
        .registered_token_address(&destination_token_id);

    InterchainTokenSetup {
        source_token_address,
        destination_token_address,
        token_id,
        salt,
    }
}

fn execute_link_token(
    test_env: &LinkTokenTestEnv,
    salt: &BytesN<32>,
    token_id: &BytesN<32>,
    source_token_address: &Address,
    destination_token_address: &Address,
    destination_manager_type: TokenManagerType,
) -> (BytesN<32>, Address) {
    let linked_token_id = test_env.source_config.its.mock_all_auths().link_token(
        &test_env.deployer,
        salt,
        &test_env.destination_chain,
        &destination_token_address.to_string_bytes(),
        &destination_manager_type,
        &None::<Bytes>,
        &Some(test_env.gas_token.clone()),
    );

    assert_eq!(token_id, &linked_token_id);

    let link_msg_payload = assert_ok!(
        stellar_interchain_token_service::types::HubMessage::ReceiveFromHub {
            source_chain: test_env.source_chain.clone(),
            message: stellar_interchain_token_service::types::Message::LinkToken(
                stellar_interchain_token_service::types::LinkToken {
                    token_id: linked_token_id.clone(),
                    token_manager_type: destination_manager_type,
                    source_token_address: source_token_address.to_string_bytes(),
                    destination_token_address: destination_token_address.to_string_bytes(),
                    params: None,
                },
            ),
        }
        .abi_encode(&test_env.env)
    );

    let message_id = String::from_str(&test_env.env, "link-message-id");
    let link_messages = vec![
        &test_env.env,
        Message {
            source_chain: test_env.hub_chain.clone(),
            message_id: message_id.clone(),
            source_address: test_env.hub_address.clone(),
            contract_address: test_env.destination_config.its.address.clone(),
            payload_hash: test_env.env.crypto().keccak256(&link_msg_payload).into(),
        },
    ];

    let proof = generate_proof(
        &test_env.env,
        get_approve_hash(&test_env.env, link_messages.clone()),
        test_env.destination_config.signers.clone(),
    );

    test_env
        .destination_config
        .gateway
        .approve_messages(&link_messages, &proof);
    test_env.destination_config.its.execute(
        &test_env.hub_chain,
        &message_id,
        &test_env.hub_address,
        &link_msg_payload,
    );

    assert_eq!(
        test_env
            .destination_config
            .its
            .token_manager_type(&linked_token_id),
        destination_manager_type
    );

    let destination_token_manager = test_env
        .destination_config
        .its
        .deployed_token_manager(&linked_token_id);

    (linked_token_id, destination_token_manager)
}

fn execute_transfer_test(test_env: &LinkTokenTestEnv, linked_token_id: &BytesN<32>) {
    test_env.source_config.app.mock_all_auths().send_token(
        &test_env.deployer,
        linked_token_id,
        &test_env.destination_chain,
        &test_env.destination_config.app.address.to_string_bytes(),
        &test_env.transfer_amount,
        &Some(test_env.recipient.to_string_bytes()),
        &Some(test_env.gas_token.clone()),
    );

    let transfer_msg_payload = assert_ok!(
        stellar_interchain_token_service::types::HubMessage::ReceiveFromHub {
            source_chain: test_env.source_chain.clone(),
            message: stellar_interchain_token_service::types::Message::InterchainTransfer(
                stellar_interchain_token_service::types::InterchainTransfer {
                    token_id: linked_token_id.clone(),
                    source_address: test_env.deployer.to_string_bytes(),
                    destination_address: test_env.destination_config.app.address.to_string_bytes(),
                    amount: test_env.transfer_amount,
                    data: Some(test_env.recipient.to_string_bytes()),
                },
            ),
        }
        .abi_encode(&test_env.env)
    );

    let message_id = String::from_str(&test_env.env, "transfer-message-id");
    let transfer_messages = vec![
        &test_env.env,
        Message {
            source_chain: test_env.hub_chain.clone(),
            message_id: message_id.clone(),
            source_address: test_env.hub_address.clone(),
            contract_address: test_env.destination_config.its.address.clone(),
            payload_hash: test_env
                .env
                .crypto()
                .keccak256(&transfer_msg_payload)
                .into(),
        },
    ];

    let proof = generate_proof(
        &test_env.env,
        get_approve_hash(&test_env.env, transfer_messages.clone()),
        test_env.destination_config.signers.clone(),
    );

    test_env
        .destination_config
        .gateway
        .approve_messages(&transfer_messages, &proof);

    test_env.destination_config.its.execute(
        &test_env.hub_chain,
        &message_id,
        &test_env.hub_address,
        &transfer_msg_payload,
    );
}

fn verify_balances(
    test_env: &LinkTokenTestEnv,
    linked_token_id: &BytesN<32>,
    source_token_address: &Address,
    destination_token_address: &Address,
    source_manager_type: TokenManagerType,
    destination_manager_type: TokenManagerType,
) {
    // Destination: tokens should be transferred to recipient
    let destination_token_client =
        token::TokenClient::new(&test_env.env, destination_token_address);
    assert_eq!(
        destination_token_client.balance(&test_env.destination_config.app.address),
        0
    );

    assert_eq!(
        destination_token_client.balance(&test_env.recipient),
        test_env.transfer_amount
    );

    // Check destination token manager balance based on manager type
    let destination_token_manager = test_env
        .destination_config
        .its
        .deployed_token_manager(linked_token_id);

    if destination_manager_type == TokenManagerType::MintBurn {
        // MintBurn: token manager should have 0 tokens (minted directly to recipient)
        assert_eq!(
            destination_token_client.balance(&destination_token_manager),
            0
        );
    } else if destination_manager_type == TokenManagerType::LockUnlock {
        // LockUnlock: token manager should have remaining tokens after unlock
        assert_eq!(
            destination_token_client.balance(&destination_token_manager),
            test_env.initial_supply - test_env.transfer_amount
        );
    }

    // Check source token balances
    let source_token_client = token::TokenClient::new(&test_env.env, source_token_address);
    let source_token_manager = test_env
        .source_config
        .its
        .deployed_token_manager(linked_token_id);

    if source_manager_type == TokenManagerType::LockUnlock {
        // LockUnlock: deployer has remaining tokens, manager has locked tokens
        assert_eq!(
            source_token_client.balance(&test_env.deployer),
            test_env.initial_supply - test_env.transfer_amount
        );
        assert_eq!(
            source_token_client.balance(&source_token_manager),
            test_env.transfer_amount
        );
    } else if source_manager_type == TokenManagerType::MintBurn {
        // MintBurn: tokens burned, manager has 0
        assert_eq!(source_token_client.balance(&source_token_manager), 0);
    }
}

#[test]
fn link_token_with_stellar_classic_asset_source_lock_unlock_destination_mint_burn() {
    let test_env = setup_link_token_test_env();
    let source_manager_type = TokenManagerType::LockUnlock;
    let destination_manager_type = TokenManagerType::MintBurn;
    let asset_setup = setup_stellar_classic_assets(&test_env, source_manager_type);

    let (linked_token_id, destination_token_manager) = execute_link_token(
        &test_env,
        &asset_setup.salt,
        &asset_setup.token_id,
        &asset_setup.source_token_address,
        &asset_setup.destination_token_address,
        destination_manager_type,
    );

    // Transfer ownership to the destination token manager
    StellarAssetClient::new(&test_env.env, &asset_setup.destination_token_address)
        .mock_all_auths()
        .set_admin(&destination_token_manager);

    execute_transfer_test(&test_env, &linked_token_id);

    verify_balances(
        &test_env,
        &linked_token_id,
        &asset_setup.source_token_address,
        &asset_setup.destination_token_address,
        source_manager_type,
        destination_manager_type,
    );
}

#[test]
fn link_token_with_stellar_classic_asset_source_mint_burn_destination_lock_unlock() {
    let test_env = setup_link_token_test_env();
    let source_manager_type = TokenManagerType::MintBurn;
    let destination_manager_type = TokenManagerType::LockUnlock;
    let asset_setup = setup_stellar_classic_assets(&test_env, source_manager_type);

    let (linked_token_id, destination_token_manager) = execute_link_token(
        &test_env,
        &asset_setup.salt,
        &asset_setup.token_id,
        &asset_setup.source_token_address,
        &asset_setup.destination_token_address,
        destination_manager_type,
    );

    // Pre-fund destination token manager for LockUnlock (to unlock the tokens)
    StellarAssetClient::new(&test_env.env, &asset_setup.destination_token_address)
        .mock_all_auths()
        .mint(&destination_token_manager, &test_env.initial_supply);

    execute_transfer_test(&test_env, &linked_token_id);

    verify_balances(
        &test_env,
        &linked_token_id,
        &asset_setup.source_token_address,
        &asset_setup.destination_token_address,
        source_manager_type,
        destination_manager_type,
    );
}

#[test]
fn link_token_with_interchain_token_source_lock_unlock_destination_mint_burn() {
    let test_env = setup_link_token_test_env();
    let source_manager_type = TokenManagerType::LockUnlock;
    let destination_manager_type = TokenManagerType::MintBurn;
    let token_setup = setup_interchain_tokens(&test_env, source_manager_type);

    let (linked_token_id, destination_token_manager) = execute_link_token(
        &test_env,
        &token_setup.salt,
        &token_setup.token_id,
        &token_setup.source_token_address,
        &token_setup.destination_token_address,
        destination_manager_type,
    );

    // Pre-fund source interchain token for LockUnlock (to lock the tokens)
    InterchainTokenClient::new(&test_env.env, &token_setup.source_token_address)
        .mock_all_auths()
        .mint(&test_env.deployer, &test_env.initial_supply);

    // Add minter permission to destination token manager
    InterchainTokenClient::new(&test_env.env, &token_setup.destination_token_address)
        .mock_all_auths()
        .add_minter(&destination_token_manager);

    execute_transfer_test(&test_env, &linked_token_id);

    verify_balances(
        &test_env,
        &linked_token_id,
        &token_setup.source_token_address,
        &token_setup.destination_token_address,
        source_manager_type,
        destination_manager_type,
    );
}

#[test]
fn link_token_with_interchain_token_source_mint_burn_destination_lock_unlock() {
    let test_env = setup_link_token_test_env();
    let source_manager_type = TokenManagerType::MintBurn;
    let destination_manager_type = TokenManagerType::LockUnlock;
    let token_setup = setup_interchain_tokens(&test_env, source_manager_type);

    let (linked_token_id, destination_token_manager) = execute_link_token(
        &test_env,
        &token_setup.salt,
        &token_setup.token_id,
        &token_setup.source_token_address,
        &token_setup.destination_token_address,
        destination_manager_type,
    );

    // Pre-fund source interchain token for MintBurn (to burn the tokens)
    InterchainTokenClient::new(&test_env.env, &token_setup.source_token_address)
        .mock_all_auths()
        .mint(&test_env.deployer, &test_env.transfer_amount);

    // Pre-fund destination interchain token manager for LockUnlock (to unlock the tokens)
    InterchainTokenClient::new(&test_env.env, &token_setup.destination_token_address)
        .mock_all_auths()
        .mint(&destination_token_manager, &test_env.initial_supply);

    execute_transfer_test(&test_env, &linked_token_id);

    verify_balances(
        &test_env,
        &linked_token_id,
        &token_setup.source_token_address,
        &token_setup.destination_token_address,
        source_manager_type,
        destination_manager_type,
    );
}
