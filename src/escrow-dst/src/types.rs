use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

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
    /// Get timelock for a specific stage
    pub fn get(&self, stage: Stage) -> u64 {
        match stage {
            Stage::SrcWithdrawal => self.src_withdrawal,
            Stage::SrcPublicWithdrawal => self.src_public_withdrawal,
            Stage::SrcCancellation => self.src_cancellation,
            Stage::SrcPublicCancellation => self.src_public_cancellation,
            Stage::DstWithdrawal => self.dst_withdrawal,
            Stage::DstPublicWithdrawal => self.dst_public_withdrawal,
            Stage::DstCancellation => self.dst_cancellation,
        }
    }

    /// Set deployed at timestamp
    pub fn set_deployed_at(&self, deployed_at: u64) -> Self {
        let mut timelocks = self.clone();
        timelocks.deployed_at = deployed_at;
        timelocks
    }
}

/// Stages for timelock management
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Stage {
    SrcWithdrawal,
    SrcPublicWithdrawal,
    SrcCancellation,
    SrcPublicCancellation,
    DstWithdrawal,
    DstPublicWithdrawal,
    DstCancellation,
}

/// Error types for escrow operations
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EscrowError {
    InvalidSecret,
    InvalidImmutables,
    TimelockNotReached,
    TimelockExpired,
    OnlyTaker,
    OnlyAccessTokenHolder,
    TransferFailed,
    InvalidAmount,
}

impl AsRef<str> for EscrowError {
    fn as_ref(&self) -> &str {
        match self {
            EscrowError::InvalidSecret => "InvalidSecret",
            EscrowError::InvalidImmutables => "InvalidImmutables",
            EscrowError::TimelockNotReached => "TimelockNotReached",
            EscrowError::TimelockExpired => "TimelockExpired",
            EscrowError::OnlyTaker => "OnlyTaker",
            EscrowError::OnlyAccessTokenHolder => "OnlyAccessTokenHolder",
            EscrowError::TransferFailed => "TransferFailed",
            EscrowError::InvalidAmount => "InvalidAmount",
        }
    }
} 