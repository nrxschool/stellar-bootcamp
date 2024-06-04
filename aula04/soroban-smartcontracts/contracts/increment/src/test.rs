#![cfg(test)]

use super::{IncrementContract, IncrementContractClient};
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn simple_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.get_counter(), 0);
    client.increment();
    assert_eq!(client.get_counter(), 1);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn multiple_increments_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.get_counter(), 0);
    client.increment(); // +1 => 1
    client.increment(); // +1 => 2
    client.increment(); // +1 => 3
    client.increment(); // +1 => 4
    client.increment(); // +1 => 5
    client.increment(); // +1 => 6
    client.increment(); // +1 => 7
    assert_eq!(client.get_counter(), 7);

    std::println!("{}", env.logs().all().join("\n"));
}
