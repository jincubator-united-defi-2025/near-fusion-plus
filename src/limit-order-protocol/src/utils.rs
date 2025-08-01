use crate::types::{Extension, LimitOrderError, Order};

/// Compute hash of an order
pub fn hash_order(order: &Order, domain_separator: &[u8; 32]) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(domain_separator);
    data.extend_from_slice(&order.salt.to_le_bytes());
    data.extend_from_slice(order.maker.as_bytes());
    data.extend_from_slice(order.receiver.as_bytes());
    data.extend_from_slice(order.maker_asset.as_bytes());
    data.extend_from_slice(order.taker_asset.as_bytes());
    data.extend_from_slice(&order.making_amount.to_le_bytes());
    data.extend_from_slice(&order.taking_amount.to_le_bytes());

    // Hash maker traits
    let traits_hash = hash_maker_traits(&order.maker_traits);
    data.extend_from_slice(&traits_hash);

    near_sdk::env::keccak256(&data).try_into().unwrap()
}

/// Hash maker traits
pub fn hash_maker_traits(traits: &crate::types::MakerTraits) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(&(traits.use_bit_invalidator as u8).to_le_bytes());
    data.extend_from_slice(&(traits.use_epoch_manager as u8).to_le_bytes());
    data.extend_from_slice(&(traits.has_extension as u8).to_le_bytes());
    data.extend_from_slice(&traits.nonce_or_epoch.to_le_bytes());
    data.extend_from_slice(&traits.series.to_le_bytes());

    near_sdk::env::keccak256(&data).try_into().unwrap()
}

/// Calculate making amount based on taking amount
pub fn calculate_making_amount(
    order: &Order,
    extension: &Extension,
    requested_taking_amount: u128,
    _remaining_making_amount: u128,
    _order_hash: &[u8; 32],
) -> Result<u128, LimitOrderError> {
    let making_amount_data = extension.maker_amount_data();

    if making_amount_data.is_empty() {
        // Linear proportion
        if order.taking_amount == 0 {
            return Err(LimitOrderError::SwapWithZeroAmount);
        }
        return Ok((order.making_amount * requested_taking_amount) / order.taking_amount);
    }

    // In a real implementation, we would call an external getter contract
    // For now, return a simplified calculation
    Ok(requested_taking_amount)
}

/// Calculate taking amount based on making amount
pub fn calculate_taking_amount(
    order: &Order,
    extension: &Extension,
    requested_making_amount: u128,
    _remaining_making_amount: u128,
    _order_hash: &[u8; 32],
) -> Result<u128, LimitOrderError> {
    let taking_amount_data = extension.taker_amount_data();

    if taking_amount_data.is_empty() {
        // Linear proportion
        if order.making_amount == 0 {
            return Err(LimitOrderError::SwapWithZeroAmount);
        }
        return Ok((order.taking_amount * requested_making_amount) / order.making_amount);
    }

    // In a real implementation, we would call an external getter contract
    // For now, return a simplified calculation
    Ok(requested_making_amount)
}

/// Validate extension for an order
pub fn validate_extension(order: &Order, extension: &Extension) -> Result<bool, LimitOrderError> {
    if order.maker_traits.has_extension() {
        if extension.maker_amount_data().is_empty()
            && extension.taker_amount_data().is_empty()
            && extension.predicate_data().is_empty()
            && extension.permit_data().is_empty()
            && extension.pre_interaction_data().is_empty()
            && extension.post_interaction_data().is_empty()
        {
            return Err(LimitOrderError::MissingOrderExtension);
        }

        // Validate extension hash
        let extension_hash = hash_extension(extension);
        let order_salt_lower = order.salt & 0xFFFFFFFFFFFFFFFF;
        let hash_lower = u64::from_le_bytes(extension_hash[0..8].try_into().unwrap());

        if hash_lower != order_salt_lower {
            return Err(LimitOrderError::InvalidExtensionHash);
        }
    } else {
        if !extension.maker_amount_data().is_empty()
            || !extension.taker_amount_data().is_empty()
            || !extension.predicate_data().is_empty()
            || !extension.permit_data().is_empty()
            || !extension.pre_interaction_data().is_empty()
            || !extension.post_interaction_data().is_empty()
        {
            return Err(LimitOrderError::UnexpectedOrderExtension);
        }
    }

    Ok(true)
}

/// Hash extension data
pub fn hash_extension(extension: &Extension) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(extension.maker_amount_data());
    data.extend_from_slice(extension.taker_amount_data());
    data.extend_from_slice(extension.predicate_data());
    data.extend_from_slice(extension.permit_data());
    data.extend_from_slice(extension.pre_interaction_data());
    data.extend_from_slice(extension.post_interaction_data());

    near_sdk::env::keccak256(&data).try_into().unwrap()
}

/// Get receiver for an order
pub fn get_receiver(order: &Order) -> near_sdk::AccountId {
    if order.receiver.as_str() == "0x0000000000000000000000000000000000000000" {
        order.maker.clone()
    } else {
        order.receiver.clone()
    }
}

/// Validate signature for an order
pub fn validate_signature(
    _order: &Order,
    _signature: &[u8],
    _signer: &near_sdk::AccountId,
) -> bool {
    // In a real implementation, we would validate the EIP-712 signature
    // For now, return true for testing
    true
}

/// Check if order is expired
pub fn is_order_expired(_order: &Order) -> bool {
    // In a real implementation, we would check the order's expiration
    // For now, return false for testing
    false
}

/// Validate order amounts
pub fn validate_order_amounts(order: &Order, taking_amount: u128) -> bool {
    // Check if taking amount is valid
    if taking_amount == 0 {
        return false;
    }

    if taking_amount > order.taking_amount {
        return false;
    }

    true
}
