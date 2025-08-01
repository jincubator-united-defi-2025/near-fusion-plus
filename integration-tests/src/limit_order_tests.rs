use crate::utils::*;
use limit_order_protocol::LimitOrderProtocol;
use near_sdk::{
    test_utils::{accounts, VMContextBuilder},
    testing_env, AccountId, NearToken,
};

/// Test limit order protocol initialization
#[test]
fn test_limit_order_protocol_initialization() {
    let test_accounts = create_test_accounts();

    run_test(test_accounts[0].clone(), || {
        let protocol = LimitOrderProtocol::new([1u8; 32]);

        assert_eq!(protocol.get_domain_separator(), [1u8; 32]);
        assert_eq!(protocol.get_owner(), test_accounts[0]);
    });
}

/// Test order creation and validation
#[test]
fn test_order_creation_and_validation() {
    let test_accounts = create_test_accounts();
    let order = create_test_limit_order();

    run_test(test_accounts[0].clone(), || {
        let mut protocol = LimitOrderProtocol::new([1u8; 32]);

        // Test order validation
        assert!(limit_order_protocol::utils::validate_order_amounts(
            &order,
            order.taking_amount
        ));

        // Test invalid order (zero amounts)
        let mut invalid_order = order.clone();
        invalid_order.making_amount = 0;
        assert!(!limit_order_protocol::utils::validate_order_amounts(
            &invalid_order,
            invalid_order.taking_amount
        ));
    });
}

/// Test order extension validation
#[test]
fn test_order_extension_validation() {
    let extension = create_test_extension();

    // Test extension validation
    assert!(limit_order_protocol::utils::validate_extension(&extension));

    // Test extension hashing
    let hash = limit_order_protocol::utils::hash_extension(&extension);
    assert_eq!(hash.len(), 32); // Should be 32 bytes
}

/// Test order signature validation
#[test]
fn test_order_signature_validation() {
    let order = create_test_limit_order();
    let signature = vec![1u8; 65]; // Mock signature

    // Test signature validation (simplified)
    let result = limit_order_protocol::utils::validate_signature(
        &order, &signature, &[1u8; 32], // domain separator
    );

    // In test environment, this will likely fail due to simplified validation
    // In a real implementation, this would validate actual cryptographic signatures
    assert!(!result); // Expected to fail in test environment
}

/// Test order expiration
#[test]
fn test_order_expiration() {
    let order = create_test_limit_order();

    // Test order expiration check
    let is_expired = limit_order_protocol::utils::is_order_expired(&order);

    // In test environment, this will depend on the current timestamp
    // Orders with default values should not be expired
    assert!(!is_expired);
}

/// Test order amount calculations
#[test]
fn test_order_amount_calculations() {
    let order = create_test_limit_order();

    // Test making amount calculation
    let making_amount =
        limit_order_protocol::utils::calculate_making_amount(&order, order.taking_amount);

    // For default values, making amount should equal taking amount
    assert_eq!(making_amount, order.taking_amount);

    // Test taking amount calculation
    let taking_amount =
        limit_order_protocol::utils::calculate_taking_amount(&order, order.making_amount);

    // For default values, taking amount should equal making amount
    assert_eq!(taking_amount, order.making_amount);
}

/// Test maker traits functionality
#[test]
fn test_maker_traits_functionality() {
    let mut traits = MakerTraits::default();

    // Test default values
    assert!(!traits.use_bit_invalidator());
    assert!(!traits.use_epoch_manager());
    assert!(!traits.has_extension());
    assert_eq!(traits.nonce_or_epoch(), 0);
    assert_eq!(traits.series(), 0);

    // Test setting values
    traits.use_bit_invalidator = true;
    traits.use_epoch_manager = true;
    traits.has_extension = true;
    traits.nonce_or_epoch = 123;
    traits.series = 456;

    assert!(traits.use_bit_invalidator());
    assert!(traits.use_epoch_manager());
    assert!(traits.has_extension());
    assert_eq!(traits.nonce_or_epoch(), 123);
    assert_eq!(traits.series(), 456);
}

/// Test taker traits functionality
#[test]
fn test_taker_traits_functionality() {
    let mut traits = TakerTraits::default();

    // Test default values
    assert!(!traits.allow_multiple_fills());
    assert!(!traits.allow_partial_fill());
    assert!(!traits.allow_invalid_fill());

    // Test setting values
    traits.allow_multiple_fills = true;
    traits.allow_partial_fill = true;
    traits.allow_invalid_fill = true;

    assert!(traits.allow_multiple_fills());
    assert!(traits.allow_partial_fill());
    assert!(traits.allow_invalid_fill());
}

/// Test order hash calculation
#[test]
fn test_order_hash_calculation() {
    let order = create_test_limit_order();
    let domain_separator = [1u8; 32];

    let hash = limit_order_protocol::utils::hash_order(&order, &domain_separator);

    // Hash should be 32 bytes
    assert_eq!(hash.len(), 32);

    // Same order should produce same hash
    let hash2 = limit_order_protocol::utils::hash_order(&order, &domain_separator);
    assert_eq!(hash, hash2);

    // Different domain separator should produce different hash
    let different_domain = [2u8; 32];
    let hash3 = limit_order_protocol::utils::hash_order(&order, &different_domain);
    assert_ne!(hash, hash3);
}

/// Test extension functionality
#[test]
fn test_extension_functionality() {
    let extension = Extension::default();

    // Test default values
    assert!(extension.maker_amount_data.is_empty());
    assert!(extension.taker_amount_data.is_empty());
    assert!(extension.permit_data.is_empty());
    assert!(extension.pre_interaction_data.is_empty());
    assert!(extension.post_interaction_data.is_empty());

    // Test extension validation
    assert!(limit_order_protocol::utils::validate_extension(&extension));
}

/// Test protocol configuration
#[test]
fn test_protocol_configuration() {
    let test_accounts = create_test_accounts();
    let domain_separator = [42u8; 32];

    run_test(test_accounts[0].clone(), || {
        let protocol = LimitOrderProtocol::new(domain_separator);

        assert_eq!(protocol.get_domain_separator(), domain_separator);
        assert_eq!(protocol.get_owner(), test_accounts[0]);
    });
}

/// Test error handling
#[test]
fn test_error_handling() {
    let test_accounts = create_test_accounts();

    run_test(test_accounts[0].clone(), || {
        let mut protocol = LimitOrderProtocol::new([1u8; 32]);

        // Test with invalid order (zero amounts)
        let mut invalid_order = create_test_limit_order();
        invalid_order.making_amount = 0;
        invalid_order.taking_amount = 0;

        let extension = create_test_extension();
        let signature = vec![1u8; 65];

        // This should fail due to invalid amounts
        let result = protocol.fill_order(
            invalid_order,
            extension,
            signature,
            test_accounts[1].clone(),
            0,
        );

        assert!(result.is_err());
    });
}

/// Test protocol state management
#[test]
fn test_protocol_state_management() {
    let test_accounts = create_test_accounts();

    run_test(test_accounts[0].clone(), || {
        let mut protocol = LimitOrderProtocol::new([1u8; 32]);

        // Test initial state
        assert!(!protocol.is_paused());

        // Test pausing (only owner can pause)
        protocol.pause();
        assert!(protocol.is_paused());

        // Test unpausing (only owner can unpause)
        protocol.unpause();
        assert!(!protocol.is_paused());
    });
}
