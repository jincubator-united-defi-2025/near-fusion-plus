use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Default,
)]
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

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Default,
)]
pub struct Timelocks {
    pub deployed_at: u64,
    pub src_withdrawal: u64,
    pub dst_withdrawal: u64,
}

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Default,
)]
pub struct ValidationData {
    pub valid: bool,
    pub reason: Option<String>,
}

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Default,
)]
pub struct TimelockStage {
    pub stage: u8,
}

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Default,
)]
pub struct EscrowError {
    pub message: String,
}
