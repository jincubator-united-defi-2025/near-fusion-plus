use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near, AccountId, Balance, Gas, Promise,
    serde::{Deserialize, Serialize},
    collections::UnorderedMap,
};
use crate::types::{
    Immutables, DstImmutablesComplement, ExtraDataArgs, ValidationData, EscrowError, TimelockStage
};
use crate::utils::{hash_immutables, validate_partial_fill};

// Gas for cross-contract calls
const GAS_FOR_ESCROW_CREATION: Gas = Gas(50_000_000_000_000);

/// Escrow Factory contract for cross-chain atomic swap
#[near(contract_state)]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct EscrowFactory {
    pub escrow_src_implementation: AccountId,
    pub escrow_dst_implementation: AccountId,
    pub proxy_src_bytecode_hash: [u8; 32],
    pub proxy_dst_bytecode_hash: [u8; 32],
    pub last_validated: UnorderedMap<[u8; 32], ValidationData>,
}

impl Default for EscrowFactory {
    fn default() -> Self {
        Self {
            escrow_src_implementation: AccountId::new_unchecked("".to_string()),
            escrow_dst_implementation: AccountId::new_unchecked("".to_string()),
            proxy_src_bytecode_hash: [0u8; 32],
            proxy_dst_bytecode_hash: [0u8; 32],
            last_validated: UnorderedMap::new(b"last_validated"),
        }
    }
}

#[near]
impl EscrowFactory {
    /// Initialize the contract
    #[init]
    pub fn new(
        escrow_src_implementation: AccountId,
        escrow_dst_implementation: AccountId,
        proxy_src_bytecode_hash: [u8; 32],
        proxy_dst_bytecode_hash: [u8; 32],
    ) -> Self {
        Self {
            escrow_src_implementation,
            escrow_dst_implementation,
            proxy_src_bytecode_hash,
            proxy_dst_bytecode_hash,
            last_validated: UnorderedMap::new(b"last_validated"),
        }
    }

    /// Create destination escrow
    pub fn create_dst_escrow(&mut self, dst_immutables: Immutables, src_cancellation_timestamp: u64) {
        let token = dst_immutables.token.clone();
        let native_amount = if token.as_str() == "near" {
            dst_immutables.safety_deposit + dst_immutables.amount
        } else {
            dst_immutables.safety_deposit
        };

        // Validate attached deposit
        if env::attached_deposit() != native_amount {
            env::panic_str("Insufficient escrow balance");
        }

        let mut immutables = dst_immutables;
        immutables.timelocks.set_deployed_at(env::block_timestamp());
        
        // Check that the escrow cancellation will start not later than the cancellation time on the source chain
        let dst_cancellation_start = immutables.timelocks.get(TimelockStage::DstCancellation);
        if dst_cancellation_start > src_cancellation_timestamp {
            env::panic_str("Invalid creation time");
        }

        let salt = hash_immutables(&immutables);
        let escrow = self.deploy_escrow(salt, env::attached_deposit(), self.escrow_dst_implementation.clone());

        // Transfer tokens if not native
        if token.as_str() != "near" {
            // In a real implementation, we would transfer tokens from sender to escrow
            log!("Transferring {} tokens from {} to {}", immutables.amount, env::predecessor_account_id(), escrow);
        }

        log!("Dst escrow created: escrow={}, hashlock={:?}, taker={}", 
             escrow, immutables.hashlock, immutables.taker);
    }

    /// Post interaction for source escrow creation
    pub fn post_interaction(
        &mut self,
        order_hash: [u8; 32],
        hashlock: [u8; 32],
        maker: AccountId,
        taker: AccountId,
        token: AccountId,
        amount: Balance,
        safety_deposit: Balance,
        timelocks: crate::types::Timelocks,
        dst_token: AccountId,
        dst_chain_id: u64,
        dst_amount: Balance,
        dst_safety_deposit: Balance,
        dst_maker: AccountId,
    ) {
        let immutables = Immutables {
            order_hash,
            hashlock,
            maker,
            taker,
            token,
            amount,
            safety_deposit,
            timelocks,
        };

        let dst_complement = DstImmutablesComplement {
            maker: dst_maker,
            amount: dst_amount,
            token: dst_token,
            safety_deposit: dst_safety_deposit,
            chain_id: dst_chain_id,
        };

        log!("Src escrow created: immutables={:?}, dst_complement={:?}", immutables, dst_complement);

        let salt = hash_immutables(&immutables);
        let escrow = self.deploy_escrow(salt, 0, self.escrow_src_implementation.clone());

        // Validate escrow has sufficient balance
        if escrow.as_str() != env::current_account_id().as_str() {
            // In a real implementation, we would check the escrow balance
            log!("Escrow balance validation would happen here");
        }
    }

    /// Get address of source escrow
    pub fn address_of_escrow_src(&self, immutables: Immutables) -> AccountId {
        let salt = hash_immutables(&immutables);
        // In a real implementation, we would compute the deterministic address
        // For now, return a placeholder
        AccountId::new_unchecked(format!("escrow_src_{:?}", salt))
    }

    /// Get address of destination escrow
    pub fn address_of_escrow_dst(&self, immutables: Immutables) -> AccountId {
        let salt = hash_immutables(&immutables);
        // In a real implementation, we would compute the deterministic address
        // For now, return a placeholder
        AccountId::new_unchecked(format!("escrow_dst_{:?}", salt))
    }

    /// Deploy escrow contract
    fn deploy_escrow(&self, salt: [u8; 32], value: Balance, implementation: AccountId) -> AccountId {
        // In a real implementation, we would use NEAR's contract deployment mechanism
        // For now, return a deterministic address based on salt
        let mut address_bytes = [0u8; 32];
        address_bytes.copy_from_slice(&salt);
        AccountId::new_unchecked(format!("escrow_{:?}", address_bytes))
    }

    /// Validate partial fill for multiple secrets
    pub fn validate_partial_fill(
        &mut self,
        making_amount: u128,
        remaining_making_amount: u128,
        order_making_amount: u128,
        parts_amount: u64,
        validated_index: u64,
        order_hash: [u8; 32],
        hashlock_info: [u8; 32],
    ) -> Result<bool, EscrowError> {
        let key = self.compute_validation_key(order_hash, hashlock_info);
        
        // In a real implementation, we would store and retrieve validation data
        // For now, we'll use a simplified approach
        let validation_data = ValidationData {
            leaf: hashlock_info,
            index: validated_index,
        };
        
        self.last_validated.insert(&key, &validation_data);
        
        validate_partial_fill(
            making_amount,
            remaining_making_amount,
            order_making_amount,
            parts_amount,
            validated_index,
        )
    }

    /// Compute validation key
    fn compute_validation_key(&self, order_hash: [u8; 32], hashlock_info: [u8; 32]) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(&order_hash);
        data.extend_from_slice(&hashlock_info);
        near_sdk::hash::hash(&data).try_into().unwrap()
    }

    // Getter methods
    pub fn get_escrow_src_implementation(&self) -> AccountId {
        self.escrow_src_implementation.clone()
    }

    pub fn get_escrow_dst_implementation(&self) -> AccountId {
        self.escrow_dst_implementation.clone()
    }

    pub fn get_proxy_src_bytecode_hash(&self) -> [u8; 32] {
        self.proxy_src_bytecode_hash
    }

    pub fn get_proxy_dst_bytecode_hash(&self) -> [u8; 32] {
        self.proxy_dst_bytecode_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let src_impl = accounts(2);
        let dst_impl = accounts(3);
        let src_hash = [1u8; 32];
        let dst_hash = [2u8; 32];
        
        let contract = EscrowFactory::new(
            src_impl.clone(),
            dst_impl.clone(),
            src_hash,
            dst_hash,
        );
        
        assert_eq!(contract.get_escrow_src_implementation(), src_impl);
        assert_eq!(contract.get_escrow_dst_implementation(), dst_impl);
        assert_eq!(contract.get_proxy_src_bytecode_hash(), src_hash);
        assert_eq!(contract.get_proxy_dst_bytecode_hash(), dst_hash);
    }

    #[test]
    fn test_address_of_escrow() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let contract = EscrowFactory::new(
            accounts(2),
            accounts(3),
            [1u8; 32],
            [2u8; 32],
        );
        
        let immutables = Immutables {
            order_hash: [0u8; 32],
            hashlock: [0u8; 32],
            maker: accounts(4),
            taker: accounts(5),
            token: accounts(6),
            amount: 1000,
            safety_deposit: 100,
            timelocks: Timelocks {
                deployed_at: 0,
                src_withdrawal: 0,
                src_public_withdrawal: 0,
                src_cancellation: 0,
                src_public_cancellation: 0,
                dst_withdrawal: 0,
                dst_public_withdrawal: 0,
                dst_cancellation: 0,
            },
        };
        
        let src_address = contract.address_of_escrow_src(immutables.clone());
        let dst_address = contract.address_of_escrow_dst(immutables);
        
        assert!(!src_address.as_str().is_empty());
        assert!(!dst_address.as_str().is_empty());
    }
} 