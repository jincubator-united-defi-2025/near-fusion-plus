use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

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

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Default,
)]
pub struct Timelocks {
    pub deployed_at: u64,
    pub src_withdrawal: u64,
    pub dst_withdrawal: u64,
}

impl Timelocks {
    pub fn set_deployed_at(&mut self, timestamp: u64) {
        self.deployed_at = timestamp;
    }

    pub fn get(&self, stage: TimelockStage) -> u64 {
        match stage {
            TimelockStage::SrcWithdrawal => self.deployed_at + self.src_withdrawal,
            TimelockStage::DstWithdrawal => self.deployed_at + self.dst_withdrawal,
            _ => self.deployed_at, // Default case
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ValidationData {
    pub valid: bool,
    pub reason: Option<String>,
}

impl Default for ValidationData {
    fn default() -> Self {
        Self {
            valid: false,
            reason: None,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TimelockStage {
    SrcWithdrawal,
    SrcPublicWithdrawal,
    SrcCancellation,
    SrcPublicCancellation,
    DstWithdrawal,
    DstPublicWithdrawal,
    DstCancellation,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EscrowError {
    pub message: String,
}

impl Default for EscrowError {
    fn default() -> Self {
        Self {
            message: "Unknown error".to_string(),
        }
    }
}

// Additional types needed for the factory
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DstImmutablesComplement {
    pub maker: AccountId,
    pub amount: u128,
    pub token: AccountId,
    pub safety_deposit: u128,
    pub chain_id: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExtraDataArgs {
    pub hashlock_info: [u8; 32],
    pub deposits: u128,
    pub timelocks: Timelocks,
    pub dst_token: AccountId,
    pub dst_chain_id: u64,
}
