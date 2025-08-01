// Find all our documentation at https://docs.near.org
use crate::types::{Extension, LimitOrderError, MakerTraits, Order, BitInvalidatorData, RemainingInvalidator};
use near_sdk::{env, near, AccountId, collections::UnorderedMap, log, Gas, Promise, NearToken, ext_contract};

// Gas for cross-contract calls
const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

/// Main Limit Order Protocol contract
#[near(contract_state)]
pub struct LimitOrderProtocol {
    domain_separator: [u8; 32],
    weth: AccountId,
    bit_invalidator: UnorderedMap<AccountId, BitInvalidatorData>,
    remaining_invalidator: UnorderedMap<(AccountId, [u8; 32]), RemainingInvalidator>,
    paused: bool,
    owner: AccountId,
}

impl Default for LimitOrderProtocol {
    fn default() -> Self {
        Self {
            domain_separator: [0u8; 32],
            weth: AccountId::try_from("test.near".to_string()).unwrap(),
            bit_invalidator: UnorderedMap::new(b"b"),
            remaining_invalidator: UnorderedMap::new(b"r"),
            paused: false,
            owner: AccountId::try_from("test.near".to_string()).unwrap(),
        }
    }
}

#[near]
impl LimitOrderProtocol {
    /// Initialize the contract
    #[init]
    pub fn new(domain_separator: [u8; 32], weth: AccountId) -> Self {
        Self {
            domain_separator,
            weth,
            bit_invalidator: UnorderedMap::new(b"b"),
            remaining_invalidator: UnorderedMap::new(b"r"),
            paused: false,
            owner: env::predecessor_account_id(),
        }
    }

    /// Get domain separator
    pub fn domain_separator(&self) -> [u8; 32] {
        self.domain_separator
    }

    /// Pause all trading functionality
    pub fn pause(&mut self) {
        self.only_owner();
        self.paused = true;
        log!("Contract paused");
    }

    /// Unpause all trading functionality
    pub fn unpause(&mut self) {
        self.only_owner();
        self.paused = false;
        log!("Contract unpaused");
    }

    /// Check if contract is paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Get bit invalidator for order
    pub fn bit_invalidator_for_order(&self, maker: AccountId, slot: u64) -> bool {
        if let Some(data) = self.bit_invalidator.get(&maker) {
            data.check_slot(slot)
        } else {
            false
        }
    }

    /// Get remaining invalidator for order
    pub fn remaining_invalidator_for_order(&self, maker: AccountId, order_hash: [u8; 32]) -> u128 {
        if let Some(invalidator) = self.remaining_invalidator.get(&(maker, order_hash)) {
            invalidator.remaining()
        } else {
            0
        }
    }

    /// Get raw remaining invalidator for order
    pub fn raw_remaining_invalidator_for_order(
        &self,
        maker: AccountId,
        order_hash: [u8; 32],
    ) -> u128 {
        self.remaining_invalidator_for_order(maker, order_hash)
    }

    /// Simulate order execution
    pub fn simulate(&self, target: AccountId, data: Vec<u8>) {
        // In a real implementation, we would delegate the call
        // For now, we'll just log the simulation
        log!("Simulating call to {} with data: {:?}", target, data);
        env::panic_str("SimulationResults");
    }

    /// Cancel an order
    pub fn cancel_order(&mut self, maker_traits: MakerTraits, order_hash: [u8; 32]) {
        let maker = env::predecessor_account_id();
        
        if maker_traits.use_bit_invalidator() {
            let mut data = self.bit_invalidator.get(&maker).unwrap_or_default();
            let invalidator = data.mass_invalidate(maker_traits.nonce_or_epoch(), 0);
            self.bit_invalidator.insert(&maker, &data);
            
            log!("Bit invalidator updated: maker={}, slot_index={}, slot_value={}", 
                 maker, maker_traits.nonce_or_epoch() >> 8, invalidator);
        } else {
            let invalidator = RemainingInvalidator::fully_filled();
            self.remaining_invalidator.insert(&(maker, order_hash), &invalidator);
            
            log!("Order cancelled: order_hash={:?}", order_hash);
        }
    }

    /// Cancel multiple orders
    pub fn cancel_orders(&mut self, maker_traits: Vec<MakerTraits>, order_hashes: Vec<[u8; 32]>) {
        assert_eq!(maker_traits.len(), order_hashes.len(), "Arrays must have same length");
        
        for (maker_traits, order_hash) in maker_traits.iter().zip(order_hashes.iter()) {
            self.cancel_order(maker_traits.clone(), *order_hash);
        }
    }

    /// Fill order
    #[handle_result]
    pub fn fill_order(
        &mut self,
        order: Order,
        extension: Extension,
        _signature: Vec<u8>,
        taker: AccountId,
        taking_amount: u128,
    ) -> Result<u128, LimitOrderError> {
        // Check if contract is paused
        if self.paused {
            return Err(LimitOrderError::ContractPaused);
        }

        // Validate order amounts
        if taking_amount == 0 {
            return Err(LimitOrderError::SwapWithZeroAmount);
        }

        // Calculate making amount
        let order_hash = self.hash_order(&order);
        let making_amount = self.calculate_making_amount(
            &order,
            &extension,
            taking_amount,
            order.making_amount,
            &order_hash,
        )?;

        // Validate extension
        if !self.validate_extension(&order, &extension)? {
            return Err(LimitOrderError::InvalidExtension);
        }

        // Execute the swap
        self.execute_swap(&order, &taker, making_amount, taking_amount)?;

        log!(
            "Order filled: making_amount={}, taking_amount={}",
            making_amount,
            taking_amount
        );

        Ok(making_amount)
    }

    /// Get owner
    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    /// Get WETH address
    pub fn get_weth(&self) -> AccountId {
        self.weth.clone()
    }

    /// Only owner modifier
    fn only_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only owner can call this"
        );
    }

    // Internal helper functions
    fn hash_order(&self, order: &Order) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(&self.domain_separator);
        data.extend_from_slice(&order.salt.to_le_bytes());
        data.extend_from_slice(order.maker.as_bytes());
        data.extend_from_slice(order.receiver.as_bytes());
        data.extend_from_slice(order.maker_asset.as_bytes());
        data.extend_from_slice(order.taker_asset.as_bytes());
        data.extend_from_slice(&order.making_amount.to_le_bytes());
        data.extend_from_slice(&order.taking_amount.to_le_bytes());

        // Hash maker traits
        let traits_hash = self.hash_maker_traits(&order.maker_traits);
        data.extend_from_slice(&traits_hash);

        near_sdk::env::keccak256(&data).try_into().unwrap()
    }

    fn hash_maker_traits(&self, traits: &MakerTraits) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(&(traits.use_bit_invalidator as u8).to_le_bytes());
        data.extend_from_slice(&(traits.use_epoch_manager as u8).to_le_bytes());
        data.extend_from_slice(&(traits.has_extension as u8).to_le_bytes());
        data.extend_from_slice(&traits.nonce_or_epoch.to_le_bytes());
        data.extend_from_slice(&traits.series.to_le_bytes());

        near_sdk::env::keccak256(&data).try_into().unwrap()
    }

    fn calculate_making_amount(
        &self,
        order: &Order,
        extension: &Extension,
        requested_taking_amount: u128,
        _remaining_making_amount: u128,
        _order_hash: &[u8; 32],
    ) -> Result<u128, LimitOrderError> {
        let making_amount_data = extension.maker_amount_data();

        if making_amount_data.is_empty() {
            // Linear proportion
            if order.taking_amount == 0 {
                return Err(LimitOrderError::SwapWithZeroAmount);
            }
            return Ok((order.making_amount * requested_taking_amount) / order.taking_amount);
        }

        // In a real implementation, we would call an external getter contract
        // For now, return a simplified calculation
        Ok(requested_taking_amount)
    }

    fn validate_extension(&self, order: &Order, extension: &Extension) -> Result<bool, LimitOrderError> {
        if order.maker_traits.has_extension() {
            if extension.maker_amount_data().is_empty()
                && extension.taker_amount_data().is_empty()
                && extension.predicate_data().is_empty()
                && extension.permit_data().is_empty()
                && extension.pre_interaction_data().is_empty()
                && extension.post_interaction_data().is_empty()
            {
                return Err(LimitOrderError::MissingOrderExtension);
            }

            // Validate extension hash
            let extension_hash = self.hash_extension(extension);
            let order_salt_lower = order.salt & 0xFFFFFFFFFFFFFFFF;
            let hash_lower = u64::from_le_bytes(extension_hash[0..8].try_into().unwrap());

            if hash_lower != order_salt_lower {
                return Err(LimitOrderError::InvalidExtensionHash);
            }
        } else {
            if !extension.maker_amount_data().is_empty()
                || !extension.taker_amount_data().is_empty()
                || !extension.predicate_data().is_empty()
                || !extension.permit_data().is_empty()
                || !extension.pre_interaction_data().is_empty()
                || !extension.post_interaction_data().is_empty()
            {
                return Err(LimitOrderError::UnexpectedOrderExtension);
            }
        }

        Ok(true)
    }

    fn hash_extension(&self, extension: &Extension) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(extension.maker_amount_data());
        data.extend_from_slice(extension.taker_amount_data());
        data.extend_from_slice(extension.predicate_data());
        data.extend_from_slice(extension.permit_data());
        data.extend_from_slice(extension.pre_interaction_data());
        data.extend_from_slice(extension.post_interaction_data());

        near_sdk::env::keccak256(&data).try_into().unwrap()
    }

    fn execute_swap(
        &self,
        order: &Order,
        taker: &AccountId,
        making_amount: u128,
        taking_amount: u128,
    ) -> Result<(), LimitOrderError> {
        // Transfer tokens from taker to maker
        self.transfer_tokens(&order.taker_asset, taker, &order.maker, taking_amount)?;

        // Transfer tokens from maker to taker
        self.transfer_tokens(&order.maker_asset, &order.maker, taker, making_amount)?;

        Ok(())
    }

    fn transfer_tokens(
        &self,
        token: &AccountId,
        from: &AccountId,
        to: &AccountId,
        amount: u128,
    ) -> Result<(), LimitOrderError> {
        if token.as_str() == "near" {
            // Native NEAR transfer
            Promise::new(to.clone()).transfer(NearToken::from_yoctonear(amount));
        } else {
            // Fungible token transfer
            ext_ft::ext(token.clone())
                .with_static_gas(GAS_FOR_FT_TRANSFER)
                .with_attached_deposit(NearToken::from_yoctonear(1))
                .ft_transfer_from(from.clone(), to.clone(), amount, None);
        }
        Ok(())
    }
}

// External contract trait for fungible token transfers
#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer_from(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: u128,
        memo: Option<String>,
    );
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
            .attached_deposit(near_sdk::NearToken::from_yoctonear(1));
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
            maker_traits: MakerTraits::default(),
        }
    }

    fn create_test_extension() -> Extension {
        Extension {
            maker_amount_data: vec![],
            taker_amount_data: vec![],
            predicate_data: vec![],
            permit_data: vec![],
            pre_interaction_data: vec![],
            post_interaction_data: vec![],
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = LimitOrderProtocol::new([0u8; 32], accounts(1));
        assert_eq!(contract.get_owner(), accounts(0));
        assert_eq!(contract.get_weth(), accounts(1));
    }

    #[test]
    fn test_pause_unpause() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = LimitOrderProtocol::new([0u8; 32], accounts(1));
        assert!(!contract.is_paused());

        contract.pause();
        assert!(contract.is_paused());

        contract.unpause();
        assert!(!contract.is_paused());
    }

    #[test]
    fn test_cancel_order() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = LimitOrderProtocol::new([0u8; 32], accounts(1));
        let maker_traits = MakerTraits::default();
        let order_hash = [1u8; 32];

        contract.cancel_order(maker_traits, order_hash);
        // Test passes if no panic occurs
    }

    #[test]
    fn test_bit_invalidator() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = LimitOrderProtocol::new([0u8; 32], accounts(1));
        let result = contract.bit_invalidator_for_order(accounts(1), 0);
        assert!(!result); // Should be false for default state
    }
}
