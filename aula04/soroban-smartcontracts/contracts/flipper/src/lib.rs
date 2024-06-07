#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const STATE: Symbol = symbol_short!("STATE");


// enum Result {
//     OK(bool),
//     Error(Symbol),
// }


#[contract]
pub struct Flipper;

#[contractimpl]
impl Flipper {

    pub fn flip(env: Env) {
        let mut state: bool = env.storage().instance().get(&STATE).unwrap_or(false);

        state = !state;

        env.storage().instance().set(&STATE, &state);
    }

    pub fn get_state(env: Env) -> bool {
        env.storage().instance().get(&STATE).unwrap_or(false)
    }
}

mod test;
