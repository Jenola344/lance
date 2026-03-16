#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, Env, Vec};

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum JobStatus { Open, InProgress, DeliverableSubmitted, Completed, Disputed }

#[contracttype]
#[derive(Clone)]
pub struct JobRecord {
    pub client: Address,
    pub freelancer: Option<Address>,
    pub metadata_hash: Bytes, // IPFS CID
    pub budget_stroops: i128,
    pub status: JobStatus,
}

#[contracttype]
#[derive(Clone)]
pub struct BidRecord {
    pub freelancer: Address,
    pub proposal_hash: Bytes,
}

#[contracttype]
pub enum DataKey { Job(u64), Bids(u64), Deliverable(u64) }

#[contract]
pub struct JobRegistryContract;

#[contractimpl]
impl JobRegistryContract {
    /// Client posts a job. `metadata_hash` = IPFS CID bytes.
    pub fn post_job(_env: Env, _job_id: u64, _client: Address, _hash: Bytes, _budget: i128) { todo!() }

    /// Freelancer submits a bid.
    pub fn submit_bid(_env: Env, _job_id: u64, _freelancer: Address, _proposal_hash: Bytes) { todo!() }

    /// Client accepts a bid, locking in the freelancer.
    pub fn accept_bid(_env: Env, _job_id: u64, _client: Address, _freelancer: Address) { todo!() }

    /// Freelancer submits deliverable IPFS hash.
    pub fn submit_deliverable(_env: Env, _job_id: u64, _freelancer: Address, _hash: Bytes) { todo!() }

    /// Mark job disputed (called by escrow via cross-contract invoke).
    pub fn mark_disputed(_env: Env, _job_id: u64) { todo!() }

    pub fn get_job(_env: Env, _job_id: u64) -> JobRecord { todo!() }
    pub fn get_bids(_env: Env, _job_id: u64) -> Vec<BidRecord> { todo!() }
    pub fn get_deliverable(_env: Env, _job_id: u64) -> Bytes { todo!() }
}
