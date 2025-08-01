// Find all our documentation at https://docs.near.org
// Merkle Storage Invalidator contract for cross-chain atomic swap
// Migrated from Solidity contracts

pub mod types;
pub mod utils;

// Re-export the main contract for easy access
pub use merkle_storage_invalidator_contract::MerkleStorageInvalidator;

// Include the merkle storage invalidator contract
mod merkle_storage_invalidator_contract;
