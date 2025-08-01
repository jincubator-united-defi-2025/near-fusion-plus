// Find all our documentation at https://docs.near.org
// Fee Taker extension contract for limit order protocol
// Migrated from Solidity contracts

pub mod types;
pub mod utils;

// Re-export the main contract for easy access
pub use fee_taker_contract::FeeTaker;

// Include the fee taker contract
mod fee_taker_contract; 