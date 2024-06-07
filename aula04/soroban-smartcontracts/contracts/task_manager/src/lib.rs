#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, map, symbol_short, Env, Map, Symbol, Vec,
};

const TASKS: Symbol = symbol_short!("TASKS");
const TTL_THRESHOLD: u32 = 1000;
const TTL_EXTEND_TO: u32 = 5000;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ReadyTwice = 1,
    NotFound = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub name: Symbol,
    pub done: bool,
}

impl Default for Task {
    fn default() -> Task {
        Task {
            id: 0,
            name: symbol_short!("task"),
            done: false,
        }
    }
}

pub trait CRUD {
    fn add_task(env: Env, name: Symbol) -> u32;

    fn get_task(env: Env, id: u32) -> Option<Task>;
    fn get_all_tasks(env: Env) -> Vec<Task>;

    fn complete_task(env: Env, id: u32) -> Result<bool, Error>;

    fn delete_task(env: Env, id: u32) -> Result<bool, Error>;
}

#[contract]
struct Manager;

#[contractimpl]
impl CRUD for Manager {
    fn add_task(env: Env, name: Symbol) -> u32 {
        let mut tasks: Map<u32, Task> = env.storage().instance().get(&TASKS).unwrap_or(map![&env,]);

        let new_id = tasks.len() as u32 + 1;
        let task = Task {
            id: new_id,
            name,
            done: false,
        };

        tasks.set(task.id, task);

        env.storage().instance().set(&TASKS, &tasks);
        // emit event

        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        new_id
    }

    fn get_task(env: Env, id: u32) -> Option<Task> {
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
        env.storage()
            .instance()
            .get::<Symbol, Map<u32, Task>>(&TASKS)
            .unwrap_or(map![&env,])
            .get(id)
    }

    fn get_all_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
        env.storage()
            .instance()
            .get::<Symbol, Map<u32, Task>>(&TASKS)
            .unwrap_or(map![&env,])
            .values()
    }

    fn complete_task(env: Env, id: u32) -> Result<bool, Error> {
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
        let mut tasks = env
            .storage()
            .instance()
            .get::<Symbol, Map<u32, Task>>(&TASKS)
            .unwrap_or(map![&env,]);
        if let Some(mut task) = tasks.get(id) {
            if task.done {
                return Err(Error::ReadyTwice);
            }
            task.done = true;
            tasks.set(id, task);
            env.storage().instance().set(&TASKS, &tasks);
            Ok(true)
        } else {
            Err(Error::NotFound)
        }
    }

    fn delete_task(env: Env, id: u32) -> Result<bool, Error> {
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
        let mut tasks = env
            .storage()
            .instance()
            .get::<Symbol, Map<u32, Task>>(&TASKS)
            .unwrap_or(map![&env,]);
        if tasks.contains_key(id) {
            tasks.remove(id);
            env.storage().instance().set(&TASKS, &tasks);
            Ok(true)
        } else {
            Err(Error::NotFound)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_tasks_in_init() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Manager);
        let client = ManagerClient::new(&env, &contract_id);

        let tasks: Vec<Task> = client.get_all_tasks();
        assert_eq!(tasks.is_empty(), true);
    }

    #[test]
    fn add_task() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Manager);
        let client = ManagerClient::new(&env, &contract_id);

        client.add_task(&symbol_short!("task1"));
        client.add_task(&symbol_short!("task2"));
        client.add_task(&symbol_short!("task3"));
        client.add_task(&symbol_short!("task4"));

        let tasks: Vec<Task> = client.get_all_tasks();
        assert_eq!(tasks.len(), 4);
        assert_eq!(tasks.get(0).unwrap().name, symbol_short!("task1"));
        assert_eq!(tasks.get(1).unwrap().name, symbol_short!("task2"));
        assert_eq!(tasks.get(2).unwrap().name, symbol_short!("task3"));
        assert_eq!(tasks.get(3).unwrap().name, symbol_short!("task4"));
    }

    #[test]
    fn get_task() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Manager);
        let client = ManagerClient::new(&env, &contract_id);

        client.add_task(&symbol_short!("task1"));

        let task = client.get_task(&1).unwrap_or_default();
        assert_eq!(task.name, symbol_short!("task1"));
        assert_eq!(task.done, false);
    }

    #[test]
    fn complete_task() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Manager);
        let client = ManagerClient::new(&env, &contract_id);

        client.add_task(&symbol_short!("task1"));
        client.complete_task(&1);

        let task = client.get_task(&1).unwrap_or_default();
        assert_eq!(task.name, symbol_short!("task1"));
        assert_eq!(task.done, true);

        client.complete_task(&1);
    }

    #[test]
    fn delete_task() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Manager);
        let client = ManagerClient::new(&env, &contract_id);

        client.add_task(&symbol_short!("task1"));
        client.delete_task(&1);

        let task = client.get_task(&1);
        assert_eq!(task.is_none(), true);
    }
}
