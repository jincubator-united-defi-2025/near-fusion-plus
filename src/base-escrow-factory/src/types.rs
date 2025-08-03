use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

/// Order information for escrow creation
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

/// Extra data arguments for escrow creation
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExtraDataArgs {
    pub hashlock_info: [u8; 32],
    pub deposits: U256,
    pub timelocks: Timelocks,
}

impl Default for ExtraDataArgs {
    fn default() -> Self {
        Self {
            hashlock_info: [0u8; 32],
            deposits: U256::default(),
            timelocks: Timelocks::default(),
        }
    }
}

/// Timelocks for different stages of the escrow
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Timelocks {
    pub deployed_at: u64,
    pub src_withdrawal: u64,
    pub src_public_withdrawal: u64,
    pub src_cancellation: u64,
    pub src_public_cancellation: u64,
    pub dst_withdrawal: u64,
    pub dst_public_withdrawal: u64,
    pub dst_cancellation: u64,
}

impl Default for Timelocks {
    fn default() -> Self {
        Self {
            deployed_at: 0,
            src_withdrawal: 0,
            src_public_withdrawal: 0,
            src_cancellation: 0,
            src_public_cancellation: 0,
            dst_withdrawal: 0,
            dst_public_withdrawal: 0,
            dst_cancellation: 0,
        }
    }
}

impl Timelocks {
    /// Set deployed at timestamp
    pub fn set_deployed_at(&self, deployed_at: u64) -> Self {
        let mut timelocks = self.clone();
        timelocks.deployed_at = deployed_at;
        timelocks
    }
}

/// Immutable values for escrow contracts
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Immutables {
    pub order_hash: [u8; 32],
    pub hashlock: [u8; 32],
    pub maker: AccountId,
    pub taker: AccountId,
    pub token: AccountId,
    pub amount: u128,
    pub safety_deposit: u128,
    pub timelocks: Timelocks,
}

impl Default for Immutables {
    fn default() -> Self {
        Self {
            order_hash: [0u8; 32],
            hashlock: [0u8; 32],
            maker: AccountId::try_from("test.near".to_string()).unwrap(),
            taker: AccountId::try_from("test.near".to_string()).unwrap(),
            token: AccountId::try_from("test.near".to_string()).unwrap(),
            amount: 0,
            safety_deposit: 0,
            timelocks: Timelocks::default(),
        }
    }
}

/// 256-bit unsigned integer (simplified for NEAR)
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct U256 {
    pub value: u128,
}

impl Default for U256 {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl From<u128> for U256 {
    fn from(value: u128) -> Self {
        Self { value }
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

/// Error types for factory operations
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FactoryError {
    InvalidOrder,
    InvalidExtension,
    InvalidExtraData,
    InvalidSecretsAmount,
    InvalidPartialFill,
    TransferFailed,
    InvalidAmount,
    OnlyOwner,
    InvalidAccessToken,
    AccessDenied,
    InvalidProof,
}

impl AsRef<str> for FactoryError {
    fn as_ref(&self) -> &str {
        match self {
            FactoryError::InvalidOrder => "InvalidOrder",
            FactoryError::InvalidExtension => "InvalidExtension",
            FactoryError::InvalidExtraData => "InvalidExtraData",
            FactoryError::InvalidSecretsAmount => "InvalidSecretsAmount",
            FactoryError::InvalidPartialFill => "InvalidPartialFill",
            FactoryError::TransferFailed => "TransferFailed",
            FactoryError::InvalidAmount => "InvalidAmount",
            FactoryError::OnlyOwner => "OnlyOwner",
            FactoryError::InvalidAccessToken => "InvalidAccessToken",
            FactoryError::AccessDenied => "AccessDenied",
            FactoryError::InvalidProof => "InvalidProof",
        }
    }
}
