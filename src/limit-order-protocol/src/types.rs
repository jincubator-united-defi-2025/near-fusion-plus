use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

/// Order structure for limit orders
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Order {
    pub salt: u64,
    pub maker: AccountId,
    pub receiver: AccountId,
    pub maker_asset: AccountId,
    pub taker_asset: AccountId,
    pub making_amount: u128,
    pub taking_amount: u128,
    pub maker_traits: MakerTraits,
}

/// Maker traits for order customization
#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Default,
)]
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
#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Default,
)]
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

    /// Mass invalidate orders
    pub fn mass_invalidate(&mut self, nonce_or_epoch: u64, additional_mask: u64) -> u64 {
        let slot = nonce_or_epoch >> 8;
        if !self.slots.contains(&slot) {
            self.slots.push(slot);
        }
        additional_mask
    }
}

/// Remaining invalidator for tracking order fills
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, Default)]
pub struct RemainingInvalidator {
    pub remaining: u128,
}

impl RemainingInvalidator {
    /// Create a fully filled invalidator
    pub fn fully_filled() -> Self {
        Self { remaining: 0 }
    }

    /// Get remaining amount
    pub fn remaining(&self) -> u128 {
        self.remaining
    }

    /// Create new invalidator with remaining amount
    pub fn new(remaining: u128) -> Self {
        Self { remaining }
    }
}

/// Extension data for order customization
#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Default,
)]
pub struct Extension {
    pub maker_amount_data: Vec<u8>,
    pub taker_amount_data: Vec<u8>,
    pub predicate_data: Vec<u8>,
    pub permit_data: Vec<u8>,
    pub pre_interaction_data: Vec<u8>,
    pub post_interaction_data: Vec<u8>,
}

impl Extension {
    /// Get maker amount data
    pub fn maker_amount_data(&self) -> &[u8] {
        &self.maker_amount_data
    }

    /// Get taker amount data
    pub fn taker_amount_data(&self) -> &[u8] {
        &self.taker_amount_data
    }

    /// Get predicate data
    pub fn predicate_data(&self) -> &[u8] {
        &self.predicate_data
    }

    /// Get permit data
    pub fn permit_data(&self) -> &[u8] {
        &self.permit_data
    }

    /// Get pre-interaction data
    pub fn pre_interaction_data(&self) -> &[u8] {
        &self.pre_interaction_data
    }

    /// Get post-interaction data
    pub fn post_interaction_data(&self) -> &[u8] {
        &self.post_interaction_data
    }
}

/// Error types for limit order operations
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LimitOrderError {
    InvalidatedOrder,
    TakingAmountExceeded,
    PrivateOrder,
    InvalidSignature,
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
    ContractPaused,
    OrderInvalidated,
    InvalidAmounts,
    InvalidExtension,
}

impl AsRef<str> for LimitOrderError {
    fn as_ref(&self) -> &str {
        match self {
            LimitOrderError::InvalidatedOrder => "InvalidatedOrder",
            LimitOrderError::TakingAmountExceeded => "TakingAmountExceeded",
            LimitOrderError::PrivateOrder => "PrivateOrder",
            LimitOrderError::InvalidSignature => "InvalidSignature",
            LimitOrderError::OrderExpired => "OrderExpired",
            LimitOrderError::WrongSeriesNonce => "WrongSeriesNonce",
            LimitOrderError::SwapWithZeroAmount => "SwapWithZeroAmount",
            LimitOrderError::PartialFillNotAllowed => "PartialFillNotAllowed",
            LimitOrderError::OrderIsNotSuitableForMassInvalidation => {
                "OrderIsNotSuitableForMassInvalidation"
            }
            LimitOrderError::EpochManagerAndBitInvalidatorsAreIncompatible => {
                "EpochManagerAndBitInvalidatorsAreIncompatible"
            }
            LimitOrderError::ReentrancyDetected => "ReentrancyDetected",
            LimitOrderError::PredicateIsNotTrue => "PredicateIsNotTrue",
            LimitOrderError::TakingAmountTooHigh => "TakingAmountTooHigh",
            LimitOrderError::MakingAmountTooLow => "MakingAmountTooLow",
            LimitOrderError::TransferFromMakerToTakerFailed => "TransferFromMakerToTakerFailed",
            LimitOrderError::TransferFromTakerToMakerFailed => "TransferFromTakerToMakerFailed",
            LimitOrderError::MismatchArraysLengths => "MismatchArraysLengths",
            LimitOrderError::InvalidPermit2Transfer => "InvalidPermit2Transfer",
            LimitOrderError::MissingOrderExtension => "MissingOrderExtension",
            LimitOrderError::UnexpectedOrderExtension => "UnexpectedOrderExtension",
            LimitOrderError::InvalidExtensionHash => "InvalidExtensionHash",
            LimitOrderError::ContractPaused => "ContractPaused",
            LimitOrderError::OrderInvalidated => "OrderInvalidated",
            LimitOrderError::InvalidAmounts => "InvalidAmounts",
            LimitOrderError::InvalidExtension => "InvalidExtension",
        }
    }
}
