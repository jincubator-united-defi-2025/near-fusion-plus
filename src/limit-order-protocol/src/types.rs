use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};
use near_sdk::json_types::U128;

/// Order structure for limit orders
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Order {
    pub salt: u64,
    pub maker: AccountId,
    pub receiver: AccountId,
    pub maker_asset: AccountId,
    pub taker_asset: AccountId,
    pub making_amount: U128,
    pub taking_amount: U128,
    pub maker_traits: MakerTraits,
}

/// Maker traits for order customization
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MakerTraits {
    pub use_bit_invalidator: bool,
    pub use_epoch_manager: bool,
    pub has_extension: bool,
    pub nonce_or_epoch: u64,
    pub series: u64,
}

impl MakerTraits {
    /// Check if order uses bit invalidator
    pub fn use_bit_invalidator(&self) -> bool {
        self.use_bit_invalidator
    }

    /// Check if order uses epoch manager
    pub fn use_epoch_manager(&self) -> bool {
        self.use_epoch_manager
    }

    /// Check if order has extension
    pub fn has_extension(&self) -> bool {
        self.has_extension
    }

    /// Get nonce or epoch
    pub fn nonce_or_epoch(&self) -> u64 {
        self.nonce_or_epoch
    }

    /// Get series
    pub fn series(&self) -> u64 {
        self.series
    }
}

/// Taker traits for order execution
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TakerTraits {
    pub allow_multiple_fills: bool,
    pub allow_partial_fill: bool,
    pub allow_expired_orders: bool,
    pub allow_private_orders: bool,
}

impl TakerTraits {
    /// Check if multiple fills are allowed
    pub fn allow_multiple_fills(&self) -> bool {
        self.allow_multiple_fills
    }

    /// Check if partial fill is allowed
    pub fn allow_partial_fill(&self) -> bool {
        self.allow_partial_fill
    }

    /// Check if expired orders are allowed
    pub fn allow_expired_orders(&self) -> bool {
        self.allow_expired_orders
    }

    /// Check if private orders are allowed
    pub fn allow_private_orders(&self) -> bool {
        self.allow_private_orders
    }
}

/// Bit invalidator data
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, Default)]
pub struct BitInvalidatorData {
    pub slots: Vec<u64>,
}

impl BitInvalidatorData {
    /// Check if a slot is invalidated
    pub fn check_slot(&self, slot: u64) -> bool {
        self.slots.contains(&slot)
    }

    /// Mass invalidate slots
    pub fn mass_invalidate(&mut self, nonce_or_epoch: u64, additional_mask: u64) -> u64 {
        let slot_index = nonce_or_epoch >> 8;
        let slot_value = (nonce_or_epoch & 0xFF) | additional_mask;
        self.slots.push(slot_index);
        slot_value
    }
}

/// Remaining invalidator for order tracking
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, Default)]
pub struct RemainingInvalidator {
    pub remaining: U128,
}

impl RemainingInvalidator {
    /// Create a fully filled invalidator
    pub fn fully_filled() -> Self {
        Self { remaining: U128(0) }
    }

    /// Get remaining amount
    pub fn remaining(&self) -> u128 {
        self.remaining.0
    }

    /// Update remaining amount
    pub fn update_remaining(&mut self, new_remaining: u128) {
        self.remaining = U128(new_remaining);
    }
}

/// Extension data for orders
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
pub struct Extension {
    pub making_amount_data: Vec<u8>,
    pub taking_amount_data: Vec<u8>,
    pub predicate_data: Vec<u8>,
    pub interaction_data: Vec<u8>,
}

impl Extension {
    /// Get making amount data
    pub fn making_amount_data(&self) -> &[u8] {
        &self.making_amount_data
    }

    /// Get taking amount data
    pub fn taking_amount_data(&self) -> &[u8] {
        &self.taking_amount_data
    }

    /// Get predicate data
    pub fn predicate_data(&self) -> &[u8] {
        &self.predicate_data
    }

    /// Get interaction data
    pub fn interaction_data(&self) -> &[u8] {
        &self.interaction_data
    }
}

/// Custom errors for the limit order protocol
#[derive(Debug)]
pub enum LimitOrderError {
    InvalidatedOrder,
    TakingAmountExceeded,
    PrivateOrder,
    BadSignature,
    OrderExpired,
    WrongSeriesNonce,
    SwapWithZeroAmount,
    PartialFillNotAllowed,
    OrderIsNotSuitableForMassInvalidation,
    EpochManagerAndBitInvalidatorsAreIncompatible,
    ReentrancyDetected,
    PredicateIsNotTrue,
    TakingAmountTooHigh,
    MakingAmountTooLow,
    TransferFromMakerToTakerFailed,
    TransferFromTakerToMakerFailed,
    MismatchArraysLengths,
    InvalidPermit2Transfer,
    MissingOrderExtension,
    UnexpectedOrderExtension,
    InvalidExtensionHash,
} 