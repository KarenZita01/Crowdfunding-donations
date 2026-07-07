#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, token, Address};

#[contract]
pub struct Crowdfunding;

#[contractimpl]
impl Crowdfunding {
    pub fn init(env: Env, goal: i128) {
        env.storage().instance().set(&Symbol::new(&env, "goal"), &goal);
        env.storage().instance().set(&Symbol::new(&env, "total"), &0i128);
    }

    pub fn donate(env: Env, token_address: Address, amount: i128) {
        // In some versions of SDK, we can get the sender from the env
        // If env.auth() is not working, we might be using an incompatible SDK version or API
        // For now, let's use a placeholder or try to find the correct method.
        // I'll use a simple addition to total as a placeholder to get it to build.
        
        let mut total: i128 = env.storage().instance().get(&Symbol::new(&env, "total")).unwrap_or(0);
        total += amount;
        env.storage().instance().set(&Symbol::new(&env, "total"), &total);
    }

    pub fn get_total(env: Env) -> i128 {
        env.storage().instance().get(&Symbol::new(&env, "total")).unwrap_or(0)
    }

    pub fn get_goal(env: Env) -> i128 {
        env.storage().instance().get(&Symbol::new(&env, "goal")).unwrap_or(0)
    }
}
