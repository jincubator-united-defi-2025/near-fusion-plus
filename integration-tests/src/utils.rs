use escrow_factory::types::{MakerTraits, Order};
use escrow_src::types::{Immutables, Timelocks};
use limit_order_protocol::types::{
    Extension, MakerTraits as LimitMakerTraits, Order as LimitOrder,
};
use near_sdk::{
    test_utils::{accounts, VMContextBuilder},
    testing_env, AccountId, NearToken,
};

/// Create test accounts
pub fn create_test_accounts() -> Vec<AccountId> {
    vec![
        accounts(0), // maker
        accounts(1), // taker
        accounts(2), // token
        accounts(3), // factory
        accounts(4), // limit order protocol
    ]
}

/// Create test immutable values for escrow
pub fn create_test_immutables() -> Immutables {
    let test_accounts = create_test_accounts();
    let current_time = 1000u64;

    Immutables {
        order_hash: [1u8; 32],
        hashlock: [2u8; 32],
        maker: test_accounts[0].clone(),
        taker: test_accounts[1].clone(),
        token: test_accounts[2].clone(),
        amount: 1000,
        safety_deposit: 100,
        timelocks: Timelocks {
            deployed_at: current_time,
            src_withdrawal: current_time + 100,
            src_public_withdrawal: current_time + 200,
            src_cancellation: current_time + 300,
            src_public_cancellation: current_time + 400,
            dst_withdrawal: current_time + 500,
            dst_public_withdrawal: current_time + 600,
            dst_cancellation: current_time + 700,
        },
    }
}

/// Create test order for factory
pub fn create_test_order() -> Order {
    let test_accounts = create_test_accounts();

    Order {
        salt: 12345,
        maker: test_accounts[0].clone(),
        receiver: test_accounts[1].clone(),
        maker_asset: test_accounts[2].clone(),
        taker_asset: accounts(5),
        making_amount: 1000,
        taking_amount: 1000,
        maker_traits: MakerTraits::default(),
    }
}

/// Create test limit order
pub fn create_test_limit_order() -> LimitOrder {
    let test_accounts = create_test_accounts();

    LimitOrder {
        salt: 12345,
        maker: test_accounts[0].clone(),
        receiver: test_accounts[1].clone(),
        maker_asset: test_accounts[2].clone(),
        taker_asset: accounts(5),
        making_amount: 1000,
        taking_amount: 1000,
        maker_traits: LimitMakerTraits::default(),
    }
}

/// Create test extension
pub fn create_test_extension() -> Extension {
    Extension::default()
}

/// Set up test context
pub fn setup_test_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .predecessor_account_id(predecessor)
        .attached_deposit(NearToken::from_yoctonear(1));
    builder
}

/// Create a secret and its hash
pub fn create_test_secret() -> ([u8; 32], [u8; 32]) {
    let secret = [42u8; 32];
    let hash = near_sdk::env::keccak256(&secret).try_into().unwrap();
    (secret, hash)
}

/// Validate that a secret matches its hash
pub fn validate_secret_hash(secret: &[u8; 32], hash: &[u8; 32]) -> bool {
    let computed_hash = near_sdk::env::keccak256(secret);
    computed_hash == *hash
}

/// Create test extra data for factory
pub fn create_test_extra_data() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&[1u8; 32]); // hashlock_info
    data.extend_from_slice(&[0u8; 32]); // deposits (simplified)
    data.extend_from_slice(&[0u8; 64]); // timelocks (simplified)
    data
}

/// Helper to run a test with proper context
pub fn run_test<F>(predecessor: AccountId, test_fn: F)
where
    F: FnOnce(),
{
    let context = setup_test_context(predecessor);
    testing_env!(context.build());
    test_fn();
}
