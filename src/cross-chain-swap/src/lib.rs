// Find all our documentation at https://docs.near.org
// Cross-chain atomic swap contracts for NEAR
// Migrated from Solidity contracts

pub mod types;
pub mod utils;

// Re-export the main contract for easy access
pub use base_escrow_contract::BaseEscrow;

// Include the base escrow contract
mod base_escrow_contract;
