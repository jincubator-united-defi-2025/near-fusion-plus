// Find all our documentation at https://docs.near.org
// Base Escrow Factory contract for cross-chain atomic swap
// Migrated from Solidity contracts

pub mod types;
pub mod utils;

// Re-export the main contract for easy access
pub use base_escrow_factory_contract::BaseEscrowFactory;

// Include the base escrow factory contract
mod base_escrow_factory_contract;
