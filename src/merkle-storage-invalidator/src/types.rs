use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

/// Order information for limit orders
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

impl Default for Order {
    fn default() -> Self {
        Self {
            salt: 0,
            maker: AccountId::try_from("test.near".to_string()).unwrap(),
            receiver: AccountId::try_from("test.near".to_string()).unwrap(),
            maker_asset: AccountId::try_from("test.near".to_string()).unwrap(),
            taker_asset: AccountId::try_from("test.near".to_string()).unwrap(),
            making_amount: 0,
            taking_amount: 0,
            maker_traits: MakerTraits::default(),
        }
    }
}

/// Maker traits for order customization
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct MakerTraits {
    pub use_bit_invalidator: bool,
    pub use_epoch_manager: bool,
    pub has_extension: bool,
    pub nonce_or_epoch: u64,
    pub series: u64,
}

impl MakerTraits {
    /// Check if bit invalidator is used
    pub fn use_bit_invalidator(&self) -> bool {
        self.use_bit_invalidator
    }

    /// Check if epoch manager is used
    pub fn use_epoch_manager(&self) -> bool {
        self.use_epoch_manager
    }

    /// Check if extension is used
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

/// Validation data for tracking validated orders
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ValidationData {
    pub leaf: [u8; 32],
    pub index: u64,
}

impl Default for ValidationData {
    fn default() -> Self {
        Self {
            leaf: [0u8; 32],
            index: 0,
        }
    }
}

/// Taker data for Merkle proof validation
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct TakerData {
    pub idx: u64,
    pub secret_hash: [u8; 32],
    pub proof: Vec<[u8; 32]>,
}

impl Default for TakerData {
    fn default() -> Self {
        Self {
            idx: 0,
            secret_hash: [0u8; 32],
            proof: Vec::new(),
        }
    }
}

/// Error types for merkle storage invalidator operations
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum InvalidatorError {
    AccessDenied,
    InvalidProof,
    InvalidExtension,
    InvalidExtraData,
}

impl AsRef<str> for InvalidatorError {
    fn as_ref(&self) -> &str {
        match self {
            InvalidatorError::AccessDenied => "AccessDenied",
            InvalidatorError::InvalidProof => "InvalidProof",
            InvalidatorError::InvalidExtension => "InvalidExtension",
            InvalidatorError::InvalidExtraData => "InvalidExtraData",
        }
    }
} 