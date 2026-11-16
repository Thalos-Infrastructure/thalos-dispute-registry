#![no_std]

mod contract;
mod error;
#[allow(dead_code)]
mod events {
    pub mod handler;
}
mod storage {
    pub mod types;
}

/// Business logic lives here, same split Trustless Work uses in
/// `trustlesswork-smart-contract-stellar` (`core/` + `contract.rs`).
mod core {
    pub mod registry;
}

#[cfg(test)]
mod tests;

pub use crate::contract::DisputeRegistryContract;
