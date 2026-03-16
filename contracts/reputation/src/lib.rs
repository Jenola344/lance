#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum Role { Client, Freelancer }

#[contracttype]
#[derive(Clone)]
pub struct ReputationScore {
    pub address: Address,
    pub role: Role,
    /// Score in basis points (0–10000 = 0–100%)
    pub score: i32,
    pub total_jobs: u32,
}

#[contracttype]
pub enum DataKey { Score(Address, Role), Admin }

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {
    pub fn initialize(_env: Env, _admin: Address) { todo!() }

    /// Update reputation after a completed job. `delta` in basis points.
    /// Score is clamped to [0, 10000].
    pub fn update_score(_env: Env, _address: Address, _role: Role, _delta: i32) { todo!() }

    /// Slash address for fraud / abandonment — reduces score by 20%.
    pub fn slash(_env: Env, _address: Address, _role: Role, _reason: Symbol) { todo!() }

    pub fn get_score(_env: Env, _address: Address, _role: Role) -> ReputationScore { todo!() }
}
