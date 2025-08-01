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

/// Fee configuration
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FeeConfig {
    pub fee_amount: u128,
    pub fee_receiver: AccountId,
    pub custom_receiver: bool,
}

impl Default for FeeConfig {
    fn default() -> Self {
        Self {
            fee_amount: 0,
            fee_receiver: AccountId::try_from("test.near".to_string()).unwrap(),
            custom_receiver: false,
        }
    }
}

/// Error types for fee taker operations
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FeeTakerError {
    OnlyLimitOrderProtocol,
    OnlyWhitelistOrAccessToken,
    EthTransferFailed,
    InconsistentFee,
    TransferFailed,
    InvalidAmount,
    OnlyOwner,
}

impl AsRef<str> for FeeTakerError {
    fn as_ref(&self) -> &str {
        match self {
            FeeTakerError::OnlyLimitOrderProtocol => "OnlyLimitOrderProtocol",
            FeeTakerError::OnlyWhitelistOrAccessToken => "OnlyWhitelistOrAccessToken",
            FeeTakerError::EthTransferFailed => "EthTransferFailed",
            FeeTakerError::InconsistentFee => "InconsistentFee",
            FeeTakerError::TransferFailed => "TransferFailed",
            FeeTakerError::InvalidAmount => "InvalidAmount",
            FeeTakerError::OnlyOwner => "OnlyOwner",
        }
    }
} 