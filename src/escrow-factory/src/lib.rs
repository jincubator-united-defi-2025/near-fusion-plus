// Find all our documentation at https://docs.near.org
// Escrow Factory contract for cross-chain atomic swap
// Migrated from Solidity contracts

pub mod types;
pub mod utils;

// Re-export the main contract for easy access
pub use escrow_factory_contract::EscrowFactory;

// Include the escrow factory contract
mod escrow_factory_contract; 