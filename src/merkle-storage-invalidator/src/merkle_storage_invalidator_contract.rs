// Find all our documentation at https://docs.near.org
use near_sdk::{
    env, log, near, AccountId,
    collections::UnorderedMap,
};
use crate::types::{Order, ValidationData, InvalidatorError};
use crate::utils::{validate_limit_order_protocol, extract_post_interaction_data, parse_taker_data, validate_merkle_proof, create_validation_key, extract_root};

/// Merkle Storage Invalidator contract for cross-chain atomic swap
/// Handles Merkle proof validation for orders that support multiple fills
#[near(contract_state)]
pub struct MerkleStorageInvalidator {
    limit_order_protocol: AccountId,
    last_validated: UnorderedMap<[u8; 32], ValidationData>,
}

impl Default for MerkleStorageInvalidator {
    fn default() -> Self {
        Self {
            limit_order_protocol: AccountId::try_from("test.near".to_string()).unwrap(),
            last_validated: UnorderedMap::new(b"v"),
        }
    }
}

#[near]
impl MerkleStorageInvalidator {
    /// Initialize the contract
    #[init]
    pub fn new(limit_order_protocol: AccountId) -> Self {
        Self {
            limit_order_protocol,
            last_validated: UnorderedMap::new(b"v"),
        }
    }

    /// Taker interaction for Merkle proof validation
    /// Only the limit order protocol can call this function
    #[handle_result]
    pub fn taker_interaction(
        &mut self,
        _order: Order,
        extension: Vec<u8>,
        order_hash: [u8; 32],
        _taker: AccountId,
        _making_amount: u128,
        _taking_amount: u128,
        _remaining_making_amount: u128,
        extra_data: Vec<u8>,
    ) -> Result<(), InvalidatorError> {
        // Only limit order protocol can call this
        validate_limit_order_protocol(&env::predecessor_account_id(), &self.limit_order_protocol)?;

        // Extract post interaction data from extension
        let _post_interaction_data = extract_post_interaction_data(&extension)?;

        // Parse taker data from extra data
        let taker_data = parse_taker_data(&extra_data)?;

        // Extract root from hashlock info (simplified - in real implementation this would come from post interaction data)
        let root_shortened = extract_root(&[0u8; 32]); // Simplified - would come from actual data

        // Create validation key
        let key = create_validation_key(&order_hash, &root_shortened);

        // Validate Merkle proof
        let computed_root = validate_merkle_proof(
            &taker_data.proof,
            taker_data.secret_hash,
            taker_data.idx,
            root_shortened,
        );

        if !computed_root {
            return Err(InvalidatorError::InvalidProof);
        }

        // Store validation data
        let validation_data = ValidationData {
            leaf: taker_data.secret_hash,
            index: taker_data.idx + 1,
        };
        
        self.last_validated.insert(&key, &validation_data);

        log!("Merkle proof validated and stored: order_hash={:?}, index={}", order_hash, taker_data.idx);

        Ok(())
    }

    /// Get limit order protocol address
    pub fn get_limit_order_protocol(&self) -> AccountId {
        self.limit_order_protocol.clone()
    }

    /// Get validation data for a key
    pub fn get_last_validated(&self, key: [u8; 32]) -> Option<ValidationData> {
        self.last_validated.get(&key)
    }

    /// Check if a key has validation data
    pub fn has_validation_data(&self, key: [u8; 32]) -> bool {
        self.last_validated.get(&key).is_some()
    }

    /// Get all validation data (for testing purposes)
    pub fn get_all_validation_data(&self) -> Vec<([u8; 32], ValidationData)> {
        let mut data = Vec::new();
        let keys = self.last_validated.keys_as_vector();
        for i in 0..keys.len() {
            let key = keys.get(i).unwrap();
            if let Some(validation_data) = self.last_validated.get(&key) {
                data.push((key, validation_data));
            }
        }
        data
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
            maker_traits: crate::types::MakerTraits::default(),
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = MerkleStorageInvalidator::new(accounts(1));
        assert_eq!(contract.get_limit_order_protocol(), accounts(1));
    }

    #[test]
    fn test_default() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = MerkleStorageInvalidator::default();
        assert_eq!(contract.get_limit_order_protocol(), AccountId::try_from("test.near".to_string()).unwrap());
    }

    #[test]
    fn test_validation_data_storage() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = MerkleStorageInvalidator::new(accounts(1));
        
        let key = [1u8; 32];
        let validation_data = ValidationData {
            leaf: [2u8; 32],
            index: 5,
        };
        
        contract.last_validated.insert(&key, &validation_data);
        
        assert!(contract.has_validation_data(key));
        let retrieved = contract.get_last_validated(key).unwrap();
        assert_eq!(retrieved.leaf, [2u8; 32]);
        assert_eq!(retrieved.index, 5);
    }
} 