use stellar_axelar_std::{testutils::Address as _, Address, BytesN, Env, String, Val, Vec};

use crate::testutils::setup_upgrader;

#[test]
fn upgrader_client() {
    let env = Env::default();
    let upgrader_client = setup_upgrader(&env);

    let result = upgrader_client.try_upgrade(
        &Address::generate(&env),
        &String::from_str(&env, "2.0.0"),
        &BytesN::from_array(&env, &[0; 32]),
        &Vec::<Val>::new(&env),
    );

    assert!(result.is_err());
}
