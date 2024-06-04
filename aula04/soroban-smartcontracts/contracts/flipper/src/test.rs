#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn simple_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Flipper);
    let client = FlipperClient::new(&env, &contract_id);

    let current_state = client.get_state();
    assert_eq!(current_state, false);

    client.flip();

    let current_state = client.get_state();
    assert_eq!(current_state, true);
}

#[test]
fn multiple_flips_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Flipper);
    let client = FlipperClient::new(&env, &contract_id);

    let current_state = client.get_state();
    assert_eq!(current_state, false);

    client.flip(); // to true
    client.flip(); // to false
    client.flip(); // to true
    client.flip(); // to false
    client.flip(); // to true
    client.flip(); // to false

    let current_state = client.get_state();
    assert_eq!(current_state, false);
}
