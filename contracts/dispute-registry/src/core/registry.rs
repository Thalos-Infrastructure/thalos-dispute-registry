use soroban_sdk::{Address, BytesN, Env};

use crate::error::Error;
use crate::storage::types::{Config, DataKey, Dispute, DisputeStatus, Share};

pub fn config(env: &Env) -> Result<Config, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .and_then(|admin| {
            env.storage()
                .instance()
                .get(&DataKey::Executor)
                .map(|executor| Config { admin, executor })
        })
        .ok_or(Error::NotInitialized)
}

pub fn require_admin(env: &Env) -> Result<Address, Error> {
    let cfg = config(env)?;
    cfg.admin.require_auth();
    Ok(cfg.admin)
}

pub fn require_executor(env: &Env) -> Result<Address, Error> {
    let cfg = config(env)?;
    cfg.executor.require_auth();
    Ok(cfg.executor)
}

/// Week 2: register a judge. Duplicate addresses are rejected.
pub fn register_judge(_env: &Env, _judge: Address) -> Result<(), Error> {
    Err(Error::NotImplemented)
}

pub fn remove_judge(_env: &Env, _judge: Address) -> Result<(), Error> {
    Err(Error::NotImplemented)
}

/// Week 2: only the executor may submit; requester must be payer or provider.
pub fn open_dispute(
    _env: &Env,
    _escrow_id: BytesN<32>,
    _payer: Address,
    _provider: Address,
    _requester: Address,
    _reference_balance: i128,
) -> Result<u64, Error> {
    Err(Error::NotImplemented)
}

pub fn accept_seat(_env: &Env, _dispute_id: u64, _judge: Address) -> Result<(), Error> {
    Err(Error::NotImplemented)
}

pub fn decline_seat(_env: &Env, _dispute_id: u64, _judge: Address) -> Result<(), Error> {
    Err(Error::NotImplemented)
}

pub fn replace_unresponsive(_env: &Env, _dispute_id: u64, _judge: Address) -> Result<(), Error> {
    Err(Error::NotImplemented)
}

pub fn submit_vote(
    _env: &Env,
    _dispute_id: u64,
    _judge: Address,
    _share: Share,
    _justification_hash: BytesN<32>,
) -> Result<DisputeStatus, Error> {
    Err(Error::NotImplemented)
}

pub fn add_evidence_commitment(
    _env: &Env,
    _dispute_id: u64,
    _evidence_hash: BytesN<32>,
) -> Result<(), Error> {
    Err(Error::NotImplemented)
}

/// Executor records the Trustless Work `resolve_dispute` tx hash once.
pub fn record_execution(
    _env: &Env,
    _dispute_id: u64,
    _execution_hash: BytesN<32>,
) -> Result<Dispute, Error> {
    Err(Error::NotImplemented)
}
