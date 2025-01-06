extern crate std;

use crate::auth::{self, epoch};
use crate::{AxelarGateway, AxelarGatewayClient};
use axelar_soroban_std::{assert_last_emitted_event, assert_ok};
use ed25519_dalek::{Signature, Signer, SigningKey};
use rand::Rng;

use soroban_sdk::Symbol;
use soroban_sdk::{testutils::Address as _, Address};
use soroban_sdk::{testutils::BytesN as _, vec, xdr::ToXdr, Bytes, BytesN, Env, String, Vec};

use crate::types::{
    CommandType, Message, Proof, ProofSignature, ProofSigner, WeightedSigner, WeightedSigners,
};

use axelar_soroban_std::traits::IntoVec;

const DESTINATION_CHAIN: &str = "ethereum";
const DESTINATION_ADDRESS: &str = "0x4EFE356BEDeCC817cb89B4E9b796dB8bC188DC59";

#[derive(Clone, Debug)]
pub struct TestSignerSet {
    pub signer_keys: std::vec::Vec<SigningKey>,
    pub signers: WeightedSigners,
    pub domain_separator: BytesN<32>,
}

pub fn setup_gateway<'a>(
    env: &Env,
    previous_signers_retention: u32,
    num_signers: u32,
) -> (TestSignerSet, AxelarGatewayClient<'a>) {
    let owner = Address::generate(env);
    let operator = Address::generate(env);
    let signer_set = generate_signers_set(env, num_signers, BytesN::random(env));
    let initial_signers = vec![&env, signer_set.signers.clone()];
    let minimum_rotation_delay: u64 = 0;

    let contract_id = env.register(
        AxelarGateway,
        (
            owner,
            operator,
            &signer_set.domain_separator,
            minimum_rotation_delay,
            previous_signers_retention as u64,
            initial_signers,
        ),
    );

    let client = AxelarGatewayClient::new(env, &contract_id);
    (signer_set, client)
}

pub fn get_approve_hash(env: &Env, messages: Vec<Message>) -> BytesN<32> {
    env.crypto()
        .keccak256(&(CommandType::ApproveMessages, messages).to_xdr(env))
        .into()
}

pub fn generate_test_message(env: &Env) -> (Message, Bytes) {
    generate_test_message_with_randomness(env, true)
}

pub fn generate_deterministic_test_message(env: &Env) -> (Message, Bytes) {
    generate_test_message_with_randomness(env, false)
}

fn generate_test_message_with_randomness(env: &Env, use_rng: bool) -> (Message, Bytes) {
    let payload = if use_rng {
        let mut rng = rand::thread_rng();
        let len = rng.gen_range(0..20);
        let mut payload = std::vec![0u8; len];
        rng.fill(&mut payload[..]);
        Bytes::from_slice(env, &payload[..])
    } else {
        Bytes::from_array(env, &[0xde, 0xad, 0xbe, 0xef])
    };

    (
        Message {
            source_chain: String::from_str(env, DESTINATION_CHAIN),
            message_id: String::from_str(env, "test"),
            source_address: String::from_str(env, DESTINATION_ADDRESS),
            contract_address: Address::generate(env),
            payload_hash: env.crypto().keccak256(&payload).into(),
        },
        payload,
    )
}

pub fn randint(a: u32, b: u32) -> u32 {
    rand::thread_rng().gen_range(a..b)
}

pub fn generate_signers_set(
    env: &Env,
    num_signers: u32,
    domain_separator: BytesN<32>,
) -> TestSignerSet {
    generate_signers_set_with_randomness(env, num_signers, domain_separator, true)
}

pub fn generate_deterministic_signers_set(
    env: &Env,
    num_signers: u32,
    domain_separator: BytesN<32>,
) -> TestSignerSet {
    generate_signers_set_with_randomness(env, num_signers, domain_separator, false)
}

fn generate_signers_set_with_randomness(
    env: &Env,
    num_signers: u32,
    domain_separator: BytesN<32>,
    use_rng: bool,
) -> TestSignerSet {
    let mut rng = rand::thread_rng();

    let mut signer_keypair: std::vec::Vec<_> = (0..num_signers)
        .map(|s| {
            let signing_key = if use_rng {
                SigningKey::generate(&mut rng)
            } else {
                let mut seed = [0u8; 32];
                seed[0] = s as u8;
                SigningKey::from_bytes(&seed)
            };

            let weight = if use_rng {
                rng.gen_range(1..10) as u128
            } else {
                (s as u128 % 9) + 1
            };

            (signing_key, weight)
        })
        .collect();

    // Sort signers by public key
    signer_keypair.sort_by(|a, b| {
        a.0.verifying_key()
            .to_bytes()
            .cmp(&b.0.verifying_key().to_bytes())
    });

    let total_weight = signer_keypair.iter().map(|(_, w)| w).sum::<u128>();

    let signer_vec: std::vec::Vec<WeightedSigner> = signer_keypair
        .iter()
        .map(|(signing_key, w)| WeightedSigner {
            signer: BytesN::<32>::from_array(env, &signing_key.verifying_key().to_bytes()),
            weight: *w,
        })
        .collect();

    let threshold = if use_rng {
        rng.gen_range(1..=total_weight)
    } else {
        total_weight / 2 + 1
    };

    let signers = WeightedSigners {
        signers: signer_vec.into_vec(env),
        threshold,
        nonce: BytesN::<32>::from_array(env, &[0; 32]),
    };

    TestSignerSet {
        signer_keys: signer_keypair
            .into_iter()
            .map(|(signing_key, _)| signing_key)
            .collect(),
        signers,
        domain_separator,
    }
}

pub fn generate_proof(env: &Env, data_hash: BytesN<32>, signer_set: TestSignerSet) -> Proof {
    let signers_hash = env
        .crypto()
        .keccak256(&signer_set.signers.clone().to_xdr(env));

    let mut msg: Bytes = signer_set.domain_separator.into();
    msg.extend_from_array(&signers_hash.to_array());
    msg.extend_from_array(&data_hash.to_array());

    let msg_hash = env.crypto().keccak256(&msg);
    let threshold = signer_set.signers.threshold as usize;

    let proof_signers: std::vec::Vec<_> = signer_set
        .signer_keys
        .iter()
        .zip(signer_set.signers.signers.iter())
        .enumerate()
        .map(|(i, (signing_key, weighted_signer))| {
            if i > threshold {
                return ProofSigner {
                    signer: weighted_signer,
                    signature: ProofSignature::Unsigned,
                };
            }

            let signature: Signature = signing_key.sign(&msg_hash.to_array());
            ProofSigner {
                signer: weighted_signer,
                signature: ProofSignature::Signed(BytesN::<64>::from_array(
                    env,
                    &signature.to_bytes(),
                )),
            }
        })
        .collect();

    Proof {
        signers: proof_signers.into_vec(env),
        threshold: signer_set.signers.threshold,
        nonce: signer_set.signers.nonce,
    }
}

pub fn rotate_signers(env: &Env, contract_id: &Address, new_signers: TestSignerSet) {
    let mut epoch_val: u64 = 0;
    env.as_contract(contract_id, || {
        epoch_val = epoch(env) + 1;
        assert_ok!(auth::rotate_signers(env, &new_signers.signers, false));
    });

    assert_last_emitted_event(
        env,
        contract_id,
        (
            Symbol::new(env, "signers_rotated"),
            epoch_val,
            new_signers.signers.hash(env),
        ),
        (),
    );
}

pub fn rotate_to_new_signers<'a>(
    env: &Env,
    signers: TestSignerSet,
    client: &AxelarGatewayClient<'a>,
    bypass_rotation_delay: bool,
    num_signers: u32,
    use_rng: bool,
) -> TestSignerSet {
    let (new_signers, proof) = generate_new_signers(env, signers, num_signers, use_rng);

    client.rotate_signers(&new_signers.signers, &proof, &bypass_rotation_delay);
    new_signers
}

pub fn generate_new_signers(
    env: &Env,
    signers: TestSignerSet,
    num_signers: u32,
    use_rng: bool,
) -> (TestSignerSet, Proof) {
    let new_signers = if use_rng {
        generate_signers_set(env, num_signers, signers.domain_separator.clone())
    } else {
        generate_deterministic_signers_set(env, num_signers, signers.domain_separator.clone())
    };

    let data_hash = new_signers.signers.signers_rotation_hash(env);
    let proof = generate_proof(env, data_hash, signers);

    (new_signers, proof)
}
