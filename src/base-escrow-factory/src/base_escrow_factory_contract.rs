// Find all our documentation at https://docs.near.org
use near_sdk::{
    env, log, near, AccountId, Gas,
    collections::UnorderedMap,
};
use crate::types::{Order, Immutables, FactoryError, ValidationData};
use crate::utils::{validate_order, parse_extra_data_args, create_immutables, allow_multiple_fills, is_valid_partial_fill, validate_merkle_proof, extract_parts_amount, extract_root, create_validation_key};

// Gas for cross-contract calls
const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

/// Base Escrow Factory contract for cross-chain atomic swap
/// Advanced factory with Merkle validation and multiple fill support
#[near(contract_state)]
pub struct BaseEscrowFactory {
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

impl Default for BaseEscrowFactory {
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
impl BaseEscrowFactory {
    /// Initialize the contract
    #[init]
    pub fn new(
        limit_order_protocol: AccountId,
        fee_token: AccountId,
        access_token: AccountId,
        rescue_delay_src: u32,
        rescue_delay_dst: u32,
        escrow_src_implementation: AccountId,
        escrow_dst_implementation: AccountId,
    ) -> Self {
        Self {
            limit_order_protocol,
            fee_token,
            access_token,
            owner: env::predecessor_account_id(),
            rescue_delay_src,
            rescue_delay_dst,
            escrow_src_implementation,
            escrow_dst_implementation,
            proxy_src_bytecode_hash: [0u8; 32],
            proxy_dst_bytecode_hash: [0u8; 32],
            validated_data: UnorderedMap::new(b"v"),
        }
    }

    /// Post interaction for creating source escrow with advanced features
    #[handle_result]
    pub fn post_interaction(
        &mut self,
        order: Order,
        _extension: Vec<u8>,
        order_hash: [u8; 32],
        taker: AccountId,
        making_amount: u128,
        _taking_amount: u128,
        remaining_making_amount: u128,
        extra_data: Vec<u8>,
    ) -> Result<(), FactoryError> {
        // Validate order
        if !validate_order(&order) {
            return Err(FactoryError::InvalidOrder);
        }

        // Parse extra data
        let extra_data_args = parse_extra_data_args(&extra_data)?;

        // Calculate hashlock based on maker traits
        let hashlock = if allow_multiple_fills(&order.maker_traits) {
            // Handle multiple fills with Merkle validation
            let parts_amount = extract_parts_amount(&extra_data_args.hashlock_info);
            if parts_amount < 2 {
                return Err(FactoryError::InvalidSecretsAmount);
            }

            let root_shortened = extract_root(&extra_data_args.hashlock_info);
            let key = create_validation_key(&order_hash, &root_shortened);
            
            // Get validation data
            let validated = self.validated_data.get(&key).unwrap_or_default();
            
            if !is_valid_partial_fill(
                making_amount,
                remaining_making_amount,
                order.making_amount,
                parts_amount,
                validated.index,
            ) {
                return Err(FactoryError::InvalidPartialFill);
            }

            validated.leaf
        } else {
            // Single fill - use hashlock info directly
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

        log!("Source escrow created with advanced features: order_hash={:?}, taker={}", order_hash, taker);

        Ok(())
    }

    /// Taker interaction for Merkle proof validation
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
    ) -> Result<(), FactoryError> {
        // Only limit order protocol can call this
        if env::predecessor_account_id() != self.limit_order_protocol {
            return Err(FactoryError::AccessDenied);
        }

        // Parse extra data
        let extra_data_args = parse_extra_data_args(&extra_data)?;

        // Extract proof data from extension
        let proof_data = self.extract_proof_data(&extension)?;
        
        // Validate Merkle proof
        let root_shortened = extract_root(&extra_data_args.hashlock_info);
        let key = create_validation_key(&order_hash, &root_shortened);
        
        let computed_root = validate_merkle_proof(
            &proof_data.proof,
            proof_data.secret_hash,
            proof_data.index,
            root_shortened,
        );

        if !computed_root {
            return Err(FactoryError::InvalidProof);
        }

        // Store validation data
        let validation_data = ValidationData {
            leaf: proof_data.secret_hash,
            index: proof_data.index + 1,
        };
        
        self.validated_data.insert(&key, &validation_data);

        log!("Merkle proof validated: order_hash={:?}, index={}", order_hash, proof_data.index);

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

    /// Get validation data for a key
    pub fn get_validation_data(&self, key: [u8; 32]) -> Option<ValidationData> {
        self.validated_data.get(&key)
    }

    // Internal helper functions
    fn create_src_escrow(&self, immutables: Immutables) -> Result<(), FactoryError> {
        // In a real implementation, this would deploy a new escrow contract
        // For now, we'll just log the creation
        log!("Creating source escrow with immutables: {:?}", immutables);
        Ok(())
    }

    fn extract_proof_data(&self, extension: &[u8]) -> Result<ProofData, FactoryError> {
        // In a real implementation, this would properly extract proof data from extension
        // For now, we'll create a simplified version
        if extension.len() < 32 {
            return Err(FactoryError::InvalidExtension);
        }

        let mut secret_hash = [0u8; 32];
        secret_hash.copy_from_slice(&extension[0..32]);

        let index = if extension.len() >= 40 {
            u64::from_le_bytes(extension[32..40].try_into().unwrap())
        } else {
            0
        };

        let proof = if extension.len() > 40 {
            // Extract proof elements (simplified)
            let proof_data = &extension[40..];
            let proof_elements = proof_data.len() / 32;
            let mut proof = Vec::new();
            
            for i in 0..proof_elements {
                let start = i * 32;
                let end = start + 32;
                if end <= proof_data.len() {
                    let mut element = [0u8; 32];
                    element.copy_from_slice(&proof_data[start..end]);
                    proof.push(element);
                }
            }
            
            proof
        } else {
            Vec::new()
        };

        Ok(ProofData {
            secret_hash,
            index,
            proof,
        })
    }
}

/// Proof data structure for Merkle validation
#[derive(Debug)]
struct ProofData {
    secret_hash: [u8; 32],
    index: u64,
    proof: Vec<[u8; 32]>,
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

        let contract = BaseEscrowFactory::new(
            accounts(1),
            accounts(2),
            accounts(3),
            3600,
            3600,
            accounts(4),
            accounts(5),
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

        let contract = BaseEscrowFactory::default();
        assert_eq!(contract.get_rescue_delay_src(), 3600);
        assert_eq!(contract.get_rescue_delay_dst(), 3600);
    }
} 