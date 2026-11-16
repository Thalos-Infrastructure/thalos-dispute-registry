use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Skeleton / Week 1: the function is declared and will be implemented in Week 2.
    NotImplemented = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    JudgeAlreadyRegistered = 4,
    JudgeNotRegistered = 5,
    InvalidShare = 6,
    DisputeNotFound = 7,
    InvalidState = 8,
    AlreadyVoted = 9,
    VotingClosed = 10,
    NoQuorum = 11,
    ExecutionAlreadyRecorded = 12,
    NotExecutor = 13,
    NotAdmin = 14,
    NotJudge = 15,
    PartyCannotJudge = 16,
    DuplicateActiveDispute = 17,
    PayerEqualsProvider = 18,
    InsufficientEligibleJudges = 19,
    SeatNotAssigned = 20,
    ReplacementTooEarly = 21,
    UnresolvedHasNoExecution = 22,
}
