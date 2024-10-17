#![cfg(test)]
extern crate std;

use axelar_soroban_interfaces::types::{
    ProofSignature, ProofSigner, WeightedSigner, WeightedSigners,
};
use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    Address, Bytes, BytesN, Env, Vec,
};

use axelar_soroban_std::testutils::assert_invocation;

use crate::{
    auth::{self, initialize_auth},
    contract::{AxelarGateway, AxelarGatewayClient},
    testutils::{
        self, generate_proof, generate_random_payload_and_hash, generate_signers_set, initialize,
        randint,
    },
};

fn setup_env<'a>() -> (Env, Address, AxelarGatewayClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AxelarGateway);
    let client = AxelarGatewayClient::new(&env, &contract_id);

    (env, contract_id, client)
}

#[test]
fn test_initialize() {
    let (env, _, client) = setup_env();
    let user = Address::generate(&env);

    initialize(&env, &client, user, randint(0, 10), randint(1, 10));
}

#[test]
#[should_panic(expected = "Already initialized")]
fn fails_if_already_initialized() {
    let (env, _, client) = setup_env();
    let user_one = Address::generate(&env);
    let user_two = Address::generate(&env);

    initialize(&env, &client, user_one, randint(0, 10), randint(1, 10));

    // second initialization should panic
    initialize(&env, &client, user_two, randint(0, 10), randint(1, 10));
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn fails_with_empty_signer_set() {
    let (env, contract_id, _client) = setup_env();

    // create an empty WeightedSigners vector
    let empty_signer_set = Vec::<WeightedSigners>::new(&env);
    let domain_separator: BytesN<32> = BytesN::random(&env);
    let previous_signer_retention = randint(0, 10) as u64;
    let minimum_rotation_delay = 0;
    let initial_signers = empty_signer_set;

    // call should panic because signer set is empty
    env.as_contract(&contract_id, || {
        initialize_auth(
            env.clone(),
            domain_separator,
            minimum_rotation_delay,
            previous_signer_retention,
            initial_signers,
        );
    })

    // assert!(res.is_err());
}

// #[test]
// fn transfer_ownership() {
//     let (env, _, client) = setup_env();

//     let initial_owner = Address::generate(&env);
//     let new_owner = Address::generate(&env);

//     initialize(
//         &env,
//         &client,
//         initial_owner.clone(),
//         randint(1, 10),
//         randint(1, 10),
//     );

//     // transfer ownership to the new owner
//     client.transfer_ownership(&new_owner);

//     assert_invocation(
//         &env,
//         &initial_owner,
//         &client.address,
//         "transfer_ownership",
//         (new_owner.clone(),),
//     );

//     assert_emitted_event(
//         &env,
//         -1,
//         &client.address,
//         (symbol_short!("ownership"), initial_owner, new_owner.clone()),
//         (),
//     );

//     let retrieved_owner = client.owner();
//     assert_eq!(retrieved_owner, new_owner);
// }

#[test]
fn validate_proof() {
    let (env, contract_id, client) = setup_env();
    let user = Address::generate(&env);

    let signers = initialize(&env, &client, user, randint(0, 10), randint(1, 10));

    let msg_hash = generate_random_payload_and_hash(&env);
    let proof = generate_proof(&env, msg_hash.clone(), signers);

    // validate_proof shouldn't panic
    env.as_contract(&contract_id, || {
        let latest_signer_set = auth::validate_proof(&env, msg_hash, proof);
        assert!(latest_signer_set);
    });
    // let latest_signer_set = auth::validate_proof(&env, msg_hash, proof);
    // assert!(latest_signer_set);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn fail_validate_proof_invalid_epoch() {
    let (env, contract_id, client) = setup_env();
    let user = Address::generate(&env);

    initialize(&env, &client, user, randint(0, 10), randint(1, 10));

    let different_signers = generate_signers_set(&env, randint(1, 10), BytesN::random(&env));

    let msg_hash = generate_random_payload_and_hash(&env);
    let proof = generate_proof(&env, msg_hash.clone(), different_signers);

    // // should panic, epoch should return zero for unknown signer set
    env.as_contract(&contract_id, || {
        auth::validate_proof(&env, msg_hash, proof);
    })
}

#[test]
#[should_panic(expected = "failed ED25519 verification")]
fn fail_validate_proof_invalid_signatures() {
    let (env, contract_id, client) = setup_env();
    let user = Address::generate(&env);

    let signers = initialize(&env, &client, user, randint(0, 10), randint(1, 10));

    let msg_hash = generate_random_payload_and_hash(&env);
    let proof = generate_proof(&env, msg_hash.clone(), signers);

    let different_msg = Bytes::from_array(&env, &[0x04, 0x05, 0x06]);
    let different_msg_hash = env.crypto().keccak256(&different_msg).into();

    // should panic, proof is for different message hash
    env.as_contract(&contract_id, || {
        auth::validate_proof(&env, different_msg_hash, proof);
    })
}

#[test]
#[should_panic(expected = "Error(Contract, #8)")]
fn fail_validate_proof_empty_signatures() {
    let (env, contract_id, client) = setup_env();
    let user = Address::generate(&env);

    let signers = initialize(&env, &client, user, randint(0, 10), randint(1, 10));

    let msg_hash = generate_random_payload_and_hash(&env);
    let mut proof = generate_proof(&env, msg_hash.clone(), signers);

    // Modify signatures to make them invalid
    let mut new_signers = Vec::new(&env);
    for signer in proof.signers.iter() {
        new_signers.push_back(ProofSigner {
            signer: signer.signer,
            signature: ProofSignature::Unsigned,
        });
    }
    proof.signers = new_signers;

    // validate_proof should panic, empty signatures
    env.as_contract(&contract_id, || {
        auth::validate_proof(&env, msg_hash, proof);
    })
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn fail_validate_proof_invalid_signer_set() {
    let (env, contract_id, client) = setup_env();
    let user = Address::generate(&env);

    let signers = initialize(&env, &client, user, randint(0, 10), randint(1, 10));
    let new_signers = generate_signers_set(&env, randint(1, 10), signers.domain_separator.clone());

    let msg_hash = generate_random_payload_and_hash(&env);
    let mut proof = generate_proof(&env, msg_hash.clone(), signers);

    let new_proof = generate_proof(&env, msg_hash.clone(), new_signers);

    proof.signers = new_proof.signers;

    // validate_proof should panic, signatures do not match signers
    env.as_contract(&contract_id, || {
        auth::validate_proof(&env, msg_hash, proof);
    })
}

#[test]
#[should_panic(expected = "Error(Contract, #8)")]
fn fail_validate_proof_threshold_not_met() {
    let (env, contract_id, client) = setup_env();
    let user = Address::generate(&env);

    let signers = initialize(&env, &client, user, randint(0, 10), randint(1, 10));

    let env = &env;
    let mut total_weight = 0u128;

    let msg_hash = generate_random_payload_and_hash(env);
    let mut proof = generate_proof(env, msg_hash.clone(), signers);

    // Modify signatures to make them invalid
    let mut new_signers = Vec::new(env);
    for ProofSigner { signer, signature } in proof.signers.iter() {
        total_weight += signer.weight;

        if total_weight < proof.threshold {
            new_signers.push_back(ProofSigner { signer, signature });
        } else {
            new_signers.push_back(ProofSigner {
                signer,
                signature: ProofSignature::Unsigned,
            });
        }
    }
    proof.signers = new_signers;

    // should panic, all signatures are valid but total weight is below threshold
    env.as_contract(&contract_id, || {
        auth::validate_proof(&env, msg_hash, proof);
    })
}
#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn fail_validate_proof_threshold_overflow() {
    let (env, contract_id, client) = setup_env();
    let user = Address::generate(&env);

    let mut signers = initialize(&env, &client, user, randint(0, 10), randint(1, 10));

    let env = &env;

    let last_index = signers.signers.signers.len() - 1;

    // get last signer and modify its weight to max u128 - 1
    if let Some(mut last_signer) = signers.signers.signers.get(last_index) {
        last_signer.weight = u128::MAX - 1;
        signers.signers.signers.set(last_index, last_signer);
    }

    let msg_hash = generate_random_payload_and_hash(env);
    let proof = generate_proof(env, msg_hash.clone(), signers);

    // should panic, as modified signer wouldn't match the epoch
    env.as_contract(&contract_id, || {
        auth::validate_proof(&env, msg_hash, proof);
    });
}

#[test]
fn test_rotate_signers() {
    let (env, contract_id, client) = setup_env();

    let user = Address::generate(&env);
    let previous_signer_retention = 1;

    let signers = initialize(
        &env,
        &client,
        user.clone(),
        previous_signer_retention,
        randint(1, 10),
    );

    let msg_hash = generate_random_payload_and_hash(&env);

    let new_signers = generate_signers_set(&env, randint(1, 10), signers.domain_separator);

    // rotate_signers(&env, &client, new_signers.clone());
    testutils::rotate_signers(&env, &client, new_signers.clone(), &contract_id);
    // env.as_contract(&contract_id, || {
    //     auth::rotate_signers(&env, &new_signers.signers, false);
    // })

    assert_invocation(
        &env,
        &user,
        &client.address,
        "rotate_signers",
        (new_signers.signers.clone(), false),
    );

    let proof = generate_proof(&env, msg_hash.clone(), new_signers);

    env.as_contract(&contract_id, || {
        let latest_signer_set = auth::validate_proof(&env, msg_hash, proof);

        assert!(latest_signer_set);
    });
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn rotate_signers_fail_empty_signers() {
    let (env, _, _client) = setup_env();

    let empty_signers = WeightedSigners {
        signers: Vec::<WeightedSigner>::new(&env),
        threshold: 0u128,
        nonce: BytesN::random(&env),
    };

    // should throw an error, empty signer set
    auth::rotate_signers(&env, &empty_signers, false);
}

#[test]
#[should_panic(expected = "Error(Contract, #9)")]
fn rotate_signers_fail_zero_weight() {
    let (env, _, client) = setup_env();

    let user = Address::generate(&env);
    let previous_signer_retention = 1;

    initialize(
        &env,
        &client,
        user.clone(),
        previous_signer_retention,
        randint(1, 10),
    );

    let mut new_signers = generate_signers_set(&env, randint(1, 10), BytesN::random(&env));

    let last_index = new_signers.signers.signers.len() - 1;

    // get last signer and modify its weight to zero
    if let Some(mut last_signer) = new_signers.signers.signers.get(last_index) {
        last_signer.weight = 0u128;
        new_signers.signers.signers.set(last_index, last_signer);
    }

    // should throw an error, last signer weight is zero
    auth::rotate_signers(&env, &new_signers.signers, false);
    // assert!(res.is_err());
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn rotate_signers_fail_weight_overflow() {
    let (env, _, client) = setup_env();

    let user = Address::generate(&env);
    let previous_signer_retention = 1;

    initialize(
        &env,
        &client,
        user.clone(),
        previous_signer_retention,
        randint(1, 10),
    );

    let mut new_signers = generate_signers_set(&env, randint(3, 10), BytesN::random(&env));

    let last_index = new_signers.signers.signers.len() - 1;

    // get last signer and modify its weight to max u128 - 1
    if let Some(mut last_signer) = new_signers.signers.signers.get(last_index) {
        last_signer.weight = u128::MAX - 1;
        new_signers.signers.signers.set(last_index, last_signer);
    }

    // should throw an error, last signer weight should cause overflow

    auth::rotate_signers(&env, &new_signers.signers, false);
    // client.rotate_signers(&new_signers.signer_set, &false);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn rotate_signers_fail_zero_threshold() {
    let (env, _, client) = setup_env();

    let user = Address::generate(&env);
    let previous_signer_retention = 1;

    initialize(
        &env,
        &client,
        user.clone(),
        previous_signer_retention,
        randint(1, 10),
    );

    let mut new_signers = generate_signers_set(&env, randint(1, 10), BytesN::random(&env));

    // set the threshold to zero
    new_signers.signers.threshold = 0u128;

    // should error because the threshold is set to zero

    auth::rotate_signers(&env, &new_signers.signers, false);
    // let res = client.try_rotate_signers(&new_signers.signer_set, &false);
    // assert!(res.is_err());
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn rotate_signers_fail_low_total_weight() {
    let (env, _, client) = setup_env();

    let user = Address::generate(&env);
    let previous_signer_retention = 1;

    initialize(
        &env,
        &client,
        user.clone(),
        previous_signer_retention,
        randint(1, 10),
    );

    let mut new_signers = generate_signers_set(&env, randint(1, 10), BytesN::random(&env));

    let total_weight = new_signers
        .signers
        .signers
        .iter()
        .map(|WeightedSigner { weight, .. }| weight)
        .reduce(|acc, weight| acc + weight)
        .expect("Empty signers");

    let new_threshold = total_weight + 1;

    // set the threshold to zero
    new_signers.signers.threshold = new_threshold;

    // should error because the threshold is set to zero
    auth::rotate_signers(&env, &new_signers.signers, false);
    // let res = client.try_rotate_signers(&new_signers.signer_set, &false);
    // assert!(res.is_err());
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn rotate_signers_fail_wrong_signer_order() {
    let (env, _, client) = setup_env();

    let user = Address::generate(&env);
    let previous_signer_retention = 1;

    initialize(
        &env,
        &client,
        user.clone(),
        previous_signer_retention,
        randint(1, 10),
    );

    let min_signers = 2; // need at least 2 signers to test incorrect ordering
    let mut new_signers =
        generate_signers_set(&env, randint(min_signers, 10), BytesN::random(&env));

    let len = new_signers.signers.signers.len();

    // create a new vec and reverse signer order
    let mut reversed_signers = Vec::new(&env);
    for i in (0..len).rev() {
        if let Some(item) = new_signers.signers.signers.get(i) {
            reversed_signers.push_back(item);
        }
    }

    new_signers.signers.signers = reversed_signers;

    // should error because signers are in wrong order
    //
    auth::rotate_signers(&env, &new_signers.signers, false);
    // let res = client.try_rotate_signers(&new_signers.signer_set, &false);
    // assert!(res.is_err());
}

#[test]
fn multi_rotate_signers() {
    let (env, contract_id, client) = setup_env();

    let user = Address::generate(&env);
    let previous_signer_retention = randint(1, 5);

    let original_signers = initialize(
        &env,
        &client,
        user,
        previous_signer_retention,
        randint(1, 10),
    );

    let msg_hash = generate_random_payload_and_hash(&env);

    let mut previous_signers = original_signers.clone();

    for _ in 0..previous_signer_retention {
        let new_signers = generate_signers_set(
            &env,
            randint(1, 10),
            original_signers.domain_separator.clone(),
        );

        testutils::rotate_signers(&env, &client, new_signers.clone(), &contract_id);

        let proof = generate_proof(&env, msg_hash.clone(), new_signers.clone());

        env.as_contract(&contract_id, || {
            let latest_signer_set = auth::validate_proof(&env, msg_hash.clone(), proof);
            assert!(latest_signer_set);
        });

        let proof = generate_proof(&env, msg_hash.clone(), previous_signers.clone());

        env.as_contract(&contract_id, || {
            let latest_signer_set = auth::validate_proof(&env, msg_hash.clone(), proof);
            assert!(!latest_signer_set);
        });

        previous_signers = new_signers;
    }

    // Proof from the first signer set should still be valid
    let proof = generate_proof(&env, msg_hash.clone(), original_signers.clone());
    env.as_contract(&contract_id, || {
        let latest_signer_set = auth::validate_proof(&env, msg_hash, proof);
        assert!(!latest_signer_set);
    })
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn rotate_signers_panics_on_outdated_signer_set() {
    let (env, contract_id, client) = setup_env();

    let user = Address::generate(&env);
    let previous_signer_retention = randint(0, 5);

    let original_signers = initialize(
        &env,
        &client,
        user,
        previous_signer_retention,
        randint(1, 10),
    );

    let msg_hash = generate_random_payload_and_hash(&env);

    for _ in 0..(previous_signer_retention + 1) {
        let new_signers = generate_signers_set(
            &env,
            randint(1, 10),
            original_signers.domain_separator.clone(),
        );
        testutils::rotate_signers(&env, &client, new_signers.clone(), &contract_id);
    }

    // Proof from the first signer set should fail
    let proof = generate_proof(&env, msg_hash.clone(), original_signers.clone());

    env.as_contract(&contract_id, || {
        auth::validate_proof(&env, msg_hash, proof);
    });
}
