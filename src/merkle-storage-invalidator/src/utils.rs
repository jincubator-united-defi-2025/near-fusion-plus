use near_sdk::AccountId;
use crate::types::{Order, ValidationData, TakerData, InvalidatorError};

/// Process Merkle proof
pub fn process_merkle_proof(
    proof: &[[u8; 32]],
    leaf: [u8; 32],
    index: u64,
) -> Result<[u8; 32], InvalidatorError> {
    let mut current_hash = leaf;
    
    for (i, proof_element) in proof.iter().enumerate() {
        let bit = (index >> i) & 1;
        if bit == 0 {
            // Left child
            let mut data = Vec::new();
            data.extend_from_slice(&current_hash);
            data.extend_from_slice(proof_element);
            current_hash = near_sdk::env::keccak256(&data).try_into().unwrap();
        } else {
            // Right child
            let mut data = Vec::new();
            data.extend_from_slice(proof_element);
            data.extend_from_slice(&current_hash);
            current_hash = near_sdk::env::keccak256(&data).try_into().unwrap();
        }
    }
    
    Ok(current_hash)
}

/// Validate Merkle proof
pub fn validate_merkle_proof(
    proof: &[[u8; 32]],
    leaf: [u8; 32],
    index: u64,
    root: [u8; 32],
) -> bool {
    match process_merkle_proof(proof, leaf, index) {
        Ok(computed_root) => computed_root == root,
        Err(_) => false,
    }
}

/// Extract post interaction target and data from extension
pub fn extract_post_interaction_data(extension: &[u8]) -> Result<&[u8], InvalidatorError> {
    if extension.len() < 4 {
        return Err(InvalidatorError::InvalidExtension);
    }
    
    // In a real implementation, this would properly extract the post interaction data
    // For now, we'll return the extension data (simplified)
    Ok(extension)
}

/// Parse taker data from extra data
pub fn parse_taker_data(extra_data: &[u8]) -> Result<TakerData, InvalidatorError> {
    if extra_data.len() < 40 { // Minimum size for idx + secret_hash
        return Err(InvalidatorError::InvalidExtraData);
    }
    
    // Extract index (first 8 bytes)
    let idx = u64::from_le_bytes(extra_data[0..8].try_into().unwrap());
    
    // Extract secret hash (next 32 bytes)
    let mut secret_hash = [0u8; 32];
    secret_hash.copy_from_slice(&extra_data[8..40]);
    
    // Extract proof (remaining bytes)
    let proof_data = &extra_data[40..];
    let proof_elements = proof_data.len() / 32;
    let mut proof = Vec::new();
    
    for i in 0..proof_elements {
        let start = i * 32;
        let end = start + 32;
        if end <= proof_data.len() {
            let mut element = [0u8; 32];
            element.copy_from_slice(&proof_data[start..end]);
            proof.push(element);
        }
    }
    
    Ok(TakerData {
        idx,
        secret_hash,
        proof,
    })
}

/// Create validation key from order hash and root
pub fn create_validation_key(order_hash: &[u8; 32], root_shortened: &[u8; 32]) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(order_hash);
    data.extend_from_slice(root_shortened);
    near_sdk::env::keccak256(&data).try_into().unwrap()
}

/// Extract root from hashlock info
pub fn extract_root(hashlock_info: &[u8; 32]) -> [u8; 32] {
    // Extract the low 32 bytes as root
    let mut root = [0u8; 32];
    root.copy_from_slice(&hashlock_info[0..32]);
    root
}

/// Validate that caller is the limit order protocol
pub fn validate_limit_order_protocol(caller: &AccountId, limit_order_protocol: &AccountId) -> Result<(), InvalidatorError> {
    if caller != limit_order_protocol {
        return Err(InvalidatorError::AccessDenied);
    }
    Ok(())
} 