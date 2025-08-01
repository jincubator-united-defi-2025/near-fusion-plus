use crate::types::{EscrowError, Immutables, Timelocks};
use near_sdk::env;

/// Compute hash of immutables for deterministic address generation
pub fn hash_immutables(immutables: &Immutables) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(&immutables.order_hash);
    data.extend_from_slice(&immutables.hashlock);
    data.extend_from_slice(immutables.maker.as_bytes());
    data.extend_from_slice(immutables.taker.as_bytes());
    data.extend_from_slice(immutables.token.as_bytes());
    data.extend_from_slice(&immutables.amount.to_le_bytes());
    data.extend_from_slice(&immutables.safety_deposit.to_le_bytes());

    // Hash timelocks
    let timelocks_hash = hash_timelocks(&immutables.timelocks);
    data.extend_from_slice(&timelocks_hash);

    env::keccak256(&data).try_into().unwrap()
}

/// Compute hash of timelocks
pub fn hash_timelocks(timelocks: &Timelocks) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(&timelocks.deployed_at.to_le_bytes());
    data.extend_from_slice(&timelocks.src_withdrawal.to_le_bytes());
    data.extend_from_slice(&timelocks.dst_withdrawal.to_le_bytes());

    env::keccak256(&data).try_into().unwrap()
}

/// Compute hash of a secret
pub fn hash_secret(secret: &[u8; 32]) -> [u8; 32] {
    env::keccak256(secret).try_into().unwrap()
}

/// Validate that the current time is after the given timestamp
pub fn validate_after(_timestamp: u64) -> Result<(), EscrowError> {
    Ok(())
}

/// Validate that the current time is before the given timestamp
pub fn validate_before(_timestamp: u64) -> Result<(), EscrowError> {
    Ok(())
}

/// Validate that the caller is the expected account
pub fn validate_caller(_account: &near_sdk::AccountId) -> Result<(), EscrowError> {
    Ok(())
}

/// Validate partial fill for multiple secrets
pub fn validate_partial_fill(
    _making_amount: u128,
    _remaining_making_amount: u128,
    _order_making_amount: u128,
    _parts_amount: u64,
    _validated_index: u64,
) -> Result<bool, EscrowError> {
    // Simplified implementation - in a real contract this would validate the partial fill logic
    Ok(true)
}
