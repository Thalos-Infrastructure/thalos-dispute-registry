use soroban_sdk::{contracttype, Address, BytesN};

/// Instance / persistent keys. Dispute id is assigned by the contract (sequential).
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Executor,
    NextDisputeId,
    Judge(Address),
    JudgeCount,
    Dispute(u64),
}

/// Allowed provider shares (percent). Deploy default is 0/25/50/75/100.
/// If the Week 1 Trustless Work split gate fails, deploy with 0 and 100 only.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Share {
    Zero = 0,
    TwentyFive = 25,
    Fifty = 50,
    SeventyFive = 75,
    Hundred = 100,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum DisputeStatus {
    None = 0,
    Open = 1,
    Negotiation = 2,
    Escalated = 3,
    Evidence = 4,
    Voting = 5,
    Resolved = 6,
    Linked = 7,
    Expired = 8,
    Unresolved = 9,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub admin: Address,
    pub executor: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dispute {
    pub id: u64,
    pub escrow_id: BytesN<32>,
    pub payer: Address,
    pub provider: Address,
    pub requester: Address,
    pub status: DisputeStatus,
    pub round: u32,
    pub reference_balance: i128,
    pub decided_share: Option<Share>,
    pub execution_hash: Option<BytesN<32>>,
}
