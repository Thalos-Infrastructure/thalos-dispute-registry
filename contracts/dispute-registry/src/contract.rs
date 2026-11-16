use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

use crate::core::registry;
use crate::error::Error;
use crate::storage::types::{Config, DataKey, Dispute, DisputeStatus, Share};

#[contract]
pub struct DisputeRegistryContract;

#[contractimpl]
impl DisputeRegistryContract {
    /// Runs once at deploy. `admin` registers judges; `executor` opens disputes
    /// and records the Trustless Work resolution hash. Neither holds USDC.
    pub fn __constructor(env: Env, admin: Address, executor: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Executor, &executor);
        env.storage().instance().set(&DataKey::NextDisputeId, &1u64);
        env.storage().instance().set(&DataKey::JudgeCount, &0u32);
    }

    pub fn get_config(env: Env) -> Result<Config, Error> {
        registry::config(&env)
    }

    pub fn register_judge(env: Env, judge: Address) -> Result<(), Error> {
        registry::require_admin(&env)?;
        registry::register_judge(&env, judge)
    }

    pub fn remove_judge(env: Env, judge: Address) -> Result<(), Error> {
        registry::require_admin(&env)?;
        registry::remove_judge(&env, judge)
    }

    pub fn open_dispute(
        env: Env,
        escrow_id: BytesN<32>,
        payer: Address,
        provider: Address,
        requester: Address,
        reference_balance: i128,
    ) -> Result<u64, Error> {
        registry::require_executor(&env)?;
        registry::open_dispute(
            &env,
            escrow_id,
            payer,
            provider,
            requester,
            reference_balance,
        )
    }

    pub fn accept_seat(env: Env, dispute_id: u64, judge: Address) -> Result<(), Error> {
        judge.require_auth();
        registry::accept_seat(&env, dispute_id, judge)
    }

    pub fn decline_seat(env: Env, dispute_id: u64, judge: Address) -> Result<(), Error> {
        judge.require_auth();
        registry::decline_seat(&env, dispute_id, judge)
    }

    pub fn replace_unresponsive(env: Env, dispute_id: u64, judge: Address) -> Result<(), Error> {
        registry::replace_unresponsive(&env, dispute_id, judge)
    }

    pub fn submit_vote(
        env: Env,
        dispute_id: u64,
        judge: Address,
        share: Share,
        justification_hash: BytesN<32>,
    ) -> Result<DisputeStatus, Error> {
        judge.require_auth();
        registry::submit_vote(&env, dispute_id, judge, share, justification_hash)
    }

    pub fn add_evidence_commitment(
        env: Env,
        dispute_id: u64,
        evidence_hash: BytesN<32>,
    ) -> Result<(), Error> {
        registry::require_executor(&env)?;
        registry::add_evidence_commitment(&env, dispute_id, evidence_hash)
    }

    pub fn record_execution(
        env: Env,
        dispute_id: u64,
        execution_hash: BytesN<32>,
    ) -> Result<Dispute, Error> {
        registry::require_executor(&env)?;
        registry::record_execution(&env, dispute_id, execution_hash)
    }
}
