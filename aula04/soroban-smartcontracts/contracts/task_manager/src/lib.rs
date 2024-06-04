#![no_std]

use core::task;

use soroban_sdk::{contract, contractimpl, contracttype, map, symbol_short, Env, Map, Symbol, Vec};

const TASKS: Symbol = symbol_short!("TASKS");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub name: Symbol,
    pub done: bool,
}

pub trait CRUD {
    fn add_task(env: Env, name: Symbol);

    fn get_task(env: Env, id: u32) -> Option<Task>;
    fn get_all_tasks(env: Env) -> Vec<Task>;

    fn complete_task(env: Env, id: u32) -> bool;

    fn delete_task(env: Env, id: u32) -> bool;
}

#[contract]
struct Manager;

#[contractimpl]
impl CRUD for Manager {
    fn add_task(env: Env, name: Symbol) {
        let mut tasks: Map<u32, Task> = env
            .storage()
            .persistent()
            .get(&TASKS)
            .unwrap_or(map![&env,]);

        let task = Task {
            id: tasks.len() + 1 as u32,
            name,
            done: false,
        };

        tasks.set(task.id, task);

        env.storage().persistent().set(&TASKS, &tasks);
        // emit event
    }

    fn get_task(env: Env, id: u32) -> Option<Task> {
        env.storage()
            .persistent()
            .get::<Symbol, Map<u32, Task>>(&TASKS)
            .unwrap_or(map![&env,])
            .get(id)
    }

    fn get_all_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .persistent()
            .get::<Symbol, Map<u32, Task>>(&TASKS)
            .unwrap_or(map![&env,])
            .iter()
            .collect()
    }

    fn complete_task(env: Env, id: u32) -> bool {
        todo!()
    }

    fn delete_task(env: Env, id: u32) -> bool {
        todo!()
    }
}
