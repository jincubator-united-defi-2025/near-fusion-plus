// Find all our documentation at https://docs.near.org
// Source Escrow contract for cross-chain atomic swap
// Migrated from Solidity contracts

pub mod types;
pub mod utils;

// Re-export the main contract for easy access
pub use escrow_src_contract::EscrowSrc;

// Include the escrow src contract
mod escrow_src_contract; 