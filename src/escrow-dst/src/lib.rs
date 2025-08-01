// Find all our documentation at https://docs.near.org
// Destination Escrow contract for cross-chain atomic swap
// Migrated from Solidity contracts

pub mod types;
pub mod utils;

// Re-export the main contract for easy access
pub use escrow_dst_contract::EscrowDst;

// Include the escrow dst contract
mod escrow_dst_contract; 