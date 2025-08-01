use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId, Balance, Timestamp,
};

/// Immutable data for escrow contracts
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Immutables {
    pub order_hash: [u8; 32],
    pub hashlock: [u8; 32], // Hash of the secret
    pub maker: AccountId,
    pub taker: AccountId,
    pub token: AccountId,
    pub amount: Balance,
    pub safety_deposit: Balance,
    pub timelocks: Timelocks,
}

/// Timelocks for source and destination chains plus deployment timestamp
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Timelocks {
    pub deployed_at: Timestamp,
    pub src_withdrawal: u64,
    pub src_public_withdrawal: u64,
    pub src_cancellation: u64,
    pub src_public_cancellation: u64,
    pub dst_withdrawal: u64,
    pub dst_public_withdrawal: u64,
    pub dst_cancellation: u64,
}

impl Timelocks {
    /// Set the deployment timestamp
    pub fn set_deployed_at(&mut self, timestamp: Timestamp) {
        self.deployed_at = timestamp;
    }

    /// Get the start of rescue period
    pub fn rescue_start(&self, rescue_delay: u64) -> Timestamp {
        self.deployed_at + rescue_delay
    }

    /// Get timelock value for a specific stage
    pub fn get(&self, stage: TimelockStage) -> Timestamp {
        match stage {
            TimelockStage::SrcWithdrawal => self.deployed_at + self.src_withdrawal,
            TimelockStage::SrcPublicWithdrawal => self.deployed_at + self.src_public_withdrawal,
            TimelockStage::SrcCancellation => self.deployed_at + self.src_cancellation,
            TimelockStage::SrcPublicCancellation => self.deployed_at + self.src_public_cancellation,
            TimelockStage::DstWithdrawal => self.deployed_at + self.dst_withdrawal,
            TimelockStage::DstPublicWithdrawal => self.deployed_at + self.dst_public_withdrawal,
            TimelockStage::DstCancellation => self.deployed_at + self.dst_cancellation,
        }
    }
}

/// Timelock stages enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimelockStage {
    SrcWithdrawal,
    SrcPublicWithdrawal,
    SrcCancellation,
    SrcPublicCancellation,
    DstWithdrawal,
    DstPublicWithdrawal,
    DstCancellation,
}

/// Destination immutables complement for cross-chain operations
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
pub struct DstImmutablesComplement {
    pub maker: AccountId,
    pub amount: Balance,
    pub token: AccountId,
    pub safety_deposit: Balance,
    pub chain_id: u64,
}

/// Extra data arguments for escrow creation
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
pub struct ExtraDataArgs {
    pub hashlock_info: [u8; 32],
    pub deposits: u128, // Packed safety deposits
    pub timelocks: Timelocks,
    pub dst_token: AccountId,
    pub dst_chain_id: u64,
}

/// Validation data for partial fills
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
pub struct ValidationData {
    pub leaf: [u8; 32],
    pub index: u64,
}

/// Custom errors
#[derive(Debug)]
pub enum EscrowError {
    InvalidCaller,
    InvalidImmutables,
    InvalidSecret,
    InvalidTime,
    NativeTokenSendingFailure,
    InsufficientEscrowBalance,
    InvalidCreationTime,
    InvalidSecretsAmount,
    InvalidPartialFill,
} 