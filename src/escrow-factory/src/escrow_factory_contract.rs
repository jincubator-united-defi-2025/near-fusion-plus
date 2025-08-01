// Find all our documentation at https://docs.near.org
use near_sdk::{
    env, log, near, AccountId, Gas, NearToken,
    collections::UnorderedMap,
    borsh::{BorshSerialize, BorshDeserialize},
};
use crate::types::{Order, Immutables, FactoryError};
use crate::utils::{validate_order, parse_extra_data_args, create_immutables};

// Gas for cross-contract calls
const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

/// Escrow Factory contract for cross-chain atomic swap
#[near(contract_state)]
pub struct EscrowFactory {
    limit_order_protocol: AccountId,
    fee_token: AccountId,
    access_token: AccountId,
    owner: AccountId,
    rescue_delay_src: u32,
    rescue_delay_dst: u32,
    escrow_src_implementation: AccountId,
    escrow_dst_implementation: AccountId,
    proxy_src_bytecode_hash: [u8; 32],
    proxy_dst_bytecode_hash: [u8; 32],
    validated_data: UnorderedMap<[u8; 32], ValidationData>,
}

/// Validation data for tracking validated orders
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
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

impl Default for EscrowFactory {
    fn default() -> Self {
        Self {
            limit_order_protocol: AccountId::try_from("test.near".to_string()).unwrap(),
            fee_token: AccountId::try_from("test.near".to_string()).unwrap(),
            access_token: AccountId::try_from("test.near".to_string()).unwrap(),
            owner: AccountId::try_from("test.near".to_string()).unwrap(),
            rescue_delay_src: 3600,
            rescue_delay_dst: 3600,
            escrow_src_implementation: AccountId::try_from("test.near".to_string()).unwrap(),
            escrow_dst_implementation: AccountId::try_from("test.near".to_string()).unwrap(),
            proxy_src_bytecode_hash: [0u8; 32],
            proxy_dst_bytecode_hash: [0u8; 32],
            validated_data: UnorderedMap::new(b"v"),
        }
    }
}

#[near]
impl EscrowFactory {
    /// Initialize the contract
    #[init]
    pub fn new(
        limit_order_protocol: AccountId,
        fee_token: AccountId,
        access_token: AccountId,
        rescue_delay_src: u32,
        rescue_delay_dst: u32,
    ) -> Self {
        Self {
            limit_order_protocol,
            fee_token,
            access_token,
            owner: env::predecessor_account_id(),
            rescue_delay_src,
            rescue_delay_dst,
            escrow_src_implementation: AccountId::try_from("test.near".to_string()).unwrap(),
            escrow_dst_implementation: AccountId::try_from("test.near".to_string()).unwrap(),
            proxy_src_bytecode_hash: [0u8; 32],
            proxy_dst_bytecode_hash: [0u8; 32],
            validated_data: UnorderedMap::new(b"v"),
        }
    }

    /// Post interaction for creating source escrow
    #[handle_result]
    pub fn post_interaction(
        &mut self,
        order: Order,
        _extension: Vec<u8>,
        order_hash: [u8; 32],
        taker: AccountId,
        making_amount: u128,
        _taking_amount: u128,
        _remaining_making_amount: u128,
        extra_data: Vec<u8>,
    ) -> Result<(), FactoryError> {
        // Validate order
        if !validate_order(&order) {
            return Err(FactoryError::InvalidOrder);
        }

        // Parse extra data
        let extra_data_args = parse_extra_data_args(&extra_data)?;

        // Calculate hashlock
        let hashlock = if order.maker_traits.use_bit_invalidator() {
            // In a real implementation, this would handle multiple fills
            // For now, we'll use the hashlock info directly
            extra_data_args.hashlock_info
        } else {
            extra_data_args.hashlock_info
        };

        // Create immutable values
        let timelocks = extra_data_args.timelocks.set_deployed_at(env::block_timestamp_ms() / 1000);
        let safety_deposit = extra_data_args.deposits.value; // Simplified - in real implementation this would extract high 128 bits

        let immutables = create_immutables(
            &order,
            order_hash,
            hashlock,
            taker.clone(),
            making_amount,
            safety_deposit,
            timelocks,
        );

        // Create source escrow
        self.create_src_escrow(immutables)?;

        log!("Source escrow created: order_hash={:?}, taker={}", order_hash, taker);

        Ok(())
    }

    /// Create destination escrow
    #[handle_result]
    pub fn create_dst_escrow(
        &mut self,
        order: Order,
        _extension: Vec<u8>,
        order_hash: [u8; 32],
        taker: AccountId,
        making_amount: u128,
        _taking_amount: u128,
        _remaining_making_amount: u128,
        extra_data: Vec<u8>,
    ) -> Result<(), FactoryError> {
        // Validate order
        if !validate_order(&order) {
            return Err(FactoryError::InvalidOrder);
        }

        // Parse extra data
        let extra_data_args = parse_extra_data_args(&extra_data)?;

        // Calculate hashlock
        let hashlock = if order.maker_traits.use_bit_invalidator() {
            // In a real implementation, this would handle multiple fills
            // For now, we'll use the hashlock info directly
            extra_data_args.hashlock_info
        } else {
            extra_data_args.hashlock_info
        };

        // Create immutable values
        let timelocks = extra_data_args.timelocks.set_deployed_at(env::block_timestamp_ms() / 1000);
        let safety_deposit = extra_data_args.deposits.value; // Simplified - in real implementation this would extract high 128 bits

        let immutables = create_immutables(
            &order,
            order_hash,
            hashlock,
            taker.clone(),
            making_amount,
            safety_deposit,
            timelocks,
        );

        // Create destination escrow
        self.create_dst_escrow_internal(immutables)?;

        log!("Destination escrow created: order_hash={:?}, taker={}", order_hash, taker);

        Ok(())
    }

    /// Get limit order protocol address
    pub fn get_limit_order_protocol(&self) -> AccountId {
        self.limit_order_protocol.clone()
    }

    /// Get fee token address
    pub fn get_fee_token(&self) -> AccountId {
        self.fee_token.clone()
    }

    /// Get access token address
    pub fn get_access_token(&self) -> AccountId {
        self.access_token.clone()
    }

    /// Get owner
    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    /// Get rescue delay for source
    pub fn get_rescue_delay_src(&self) -> u32 {
        self.rescue_delay_src
    }

    /// Get rescue delay for destination
    pub fn get_rescue_delay_dst(&self) -> u32 {
        self.rescue_delay_dst
    }

    /// Get escrow source implementation
    pub fn get_escrow_src_implementation(&self) -> AccountId {
        self.escrow_src_implementation.clone()
    }

    /// Get escrow destination implementation
    pub fn get_escrow_dst_implementation(&self) -> AccountId {
        self.escrow_dst_implementation.clone()
    }

    // Internal helper functions
    fn create_src_escrow(&self, immutables: Immutables) -> Result<(), FactoryError> {
        // In a real implementation, this would deploy a new escrow contract
        // For now, we'll just log the creation
        log!("Creating source escrow with immutables: {:?}", immutables);
        Ok(())
    }

    fn create_dst_escrow_internal(&self, immutables: Immutables) -> Result<(), FactoryError> {
        // In a real implementation, this would deploy a new escrow contract
        // For now, we'll just log the creation
        log!("Creating destination escrow with immutables: {:?}", immutables);
        Ok(())
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .predecessor_account_id(predecessor_account_id)
            .attached_deposit(NearToken::from_yoctonear(1));
        builder
    }

    fn create_test_order() -> Order {
        Order {
            salt: 12345,
            maker: accounts(0),
            receiver: accounts(1),
            maker_asset: accounts(2),
            taker_asset: accounts(3),
            making_amount: 1000,
            taking_amount: 1000,
            maker_traits: crate::types::MakerTraits::default(),
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = EscrowFactory::new(
            accounts(1),
            accounts(2),
            accounts(3),
            3600,
            3600,
        );
        assert_eq!(contract.get_limit_order_protocol(), accounts(1));
        assert_eq!(contract.get_fee_token(), accounts(2));
        assert_eq!(contract.get_access_token(), accounts(3));
        assert_eq!(contract.get_rescue_delay_src(), 3600);
        assert_eq!(contract.get_rescue_delay_dst(), 3600);
    }

    #[test]
    fn test_default() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = EscrowFactory::default();
        assert_eq!(contract.get_rescue_delay_src(), 3600);
        assert_eq!(contract.get_rescue_delay_dst(), 3600);
    }
} 