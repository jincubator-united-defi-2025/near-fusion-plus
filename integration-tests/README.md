# Integration Tests

Comprehensive integration tests for the cross-chain swap and limit order protocol contracts.

## Overview

This test suite validates the interaction between all the migrated NEAR contracts:

- **Cross-Chain Swap Contracts**: `EscrowSrc`, `EscrowDst`, `EscrowFactory`, `BaseEscrow`
- **Limit Order Protocol**: `LimitOrderProtocol`, `OrderMixin`, `OrderLib`

## Test Categories

### Cross-Chain Swap Tests (`cross_chain_swap_tests.rs`)

Tests the complete cross-chain atomic swap workflow:

1. **Complete Workflow Test**: Tests the entire flow from order creation to escrow completion
2. **Escrow Withdrawal**: Tests withdrawal with valid secrets
3. **Escrow Cancellation**: Tests cancellation functionality
4. **Factory Validation**: Tests order validation in the factory
5. **Destination Escrow**: Tests destination escrow creation
6. **Secret Validation**: Tests cryptographic secret validation
7. **Immutable Validation**: Tests immutable value validation
8. **Order Validation**: Tests order parameter validation
9. **Timelock Functionality**: Tests timelock management
10. **Factory Configuration**: Tests factory setup and configuration

### Limit Order Protocol Tests (`limit_order_tests.rs`)

Tests the limit order protocol functionality:

1. **Protocol Initialization**: Tests contract initialization
2. **Order Creation**: Tests order creation and validation
3. **Extension Validation**: Tests order extension functionality
4. **Signature Validation**: Tests cryptographic signature validation
5. **Order Expiration**: Tests order expiration logic
6. **Amount Calculations**: Tests making/taking amount calculations
7. **Maker Traits**: Tests maker trait functionality
8. **Taker Traits**: Tests taker trait functionality
9. **Order Hashing**: Tests order hash calculation
10. **Extension Functionality**: Tests extension data handling
11. **Protocol Configuration**: Tests protocol setup
12. **Error Handling**: Tests error conditions
13. **State Management**: Tests protocol state (pause/unpause)

## Running Tests

### Prerequisites

Ensure all contract dependencies are built:

```bash
# Build all contracts
cd src/cross-chain-swap && cargo build
cd ../escrow-src && cargo build
cd ../escrow-dst && cargo build
cd ../escrow-factory && cargo build
cd ../limit-order-protocol && cargo build
```

### Run All Integration Tests

```bash
cd integration-tests
cargo test
```

### Run Specific Test Categories

```bash
# Run only cross-chain swap tests
cargo test cross_chain_swap

# Run only limit order tests
cargo test limit_order

# Run with verbose output
cargo test -- --nocapture
```

### Run Individual Tests

```bash
# Run a specific test
cargo test test_complete_cross_chain_swap_workflow

# Run tests matching a pattern
cargo test test_escrow
```

## Test Utilities

The `utils.rs` module provides helper functions for:

- **Test Data Creation**: Creating test accounts, orders, immutables
- **Context Setup**: Setting up test environments
- **Secret Management**: Creating and validating cryptographic secrets
- **Validation Helpers**: Testing validation logic

## Test Scenarios

### Cross-Chain Swap Workflow

1. **Order Creation**: Maker creates a limit order
2. **Order Execution**: Taker executes the order
3. **Source Escrow**: Factory creates source escrow to lock funds
4. **Destination Escrow**: Factory creates destination escrow
5. **Secret Exchange**: Taker reveals secret to unlock funds
6. **Completion**: Both escrows complete the atomic swap

### Limit Order Protocol

1. **Order Creation**: Maker creates order with specific parameters
2. **Order Validation**: Protocol validates order parameters
3. **Order Execution**: Taker fills the order
4. **Extension Processing**: Protocol processes order extensions
5. **State Management**: Protocol manages order state

## Expected Test Results

### Passing Tests

- Contract initialization
- Order validation
- Configuration tests
- Utility function tests
- State management tests

### Expected Failures (Test Environment)

- Timelock-based operations (due to test environment constraints)
- Cryptographic signature validation (simplified in tests)
- Cross-contract calls (mocked in tests)

## Debugging Tests

### Enable Debug Output

```bash
RUST_LOG=debug cargo test -- --nocapture
```

### Run Single Test with Debug

```bash
RUST_LOG=debug cargo test test_complete_cross_chain_swap_workflow -- --nocapture
```

### Check Test Coverage

```bash
# Install cargo-tarpaulin for coverage
cargo install cargo-tarpaulin

# Run coverage analysis
cargo tarpaulin --out Html
```

## Test Environment

The integration tests use:

- **NEAR SDK Test Utils**: For contract testing
- **Mock Accounts**: Predefined test accounts
- **Simplified Validation**: For cryptographic operations
- **Test Data**: Realistic but simplified test data

## Contributing

When adding new tests:

1. Follow the existing test structure
2. Use the utility functions in `utils.rs`
3. Add comprehensive test cases
4. Document test scenarios
5. Ensure tests are deterministic

## Troubleshooting

### Common Issues

1. **Compilation Errors**: Ensure all contracts are built
2. **Test Failures**: Check test environment setup
3. **Timelock Issues**: Expected in test environment
4. **Signature Validation**: Simplified in tests

### Debug Commands

```bash
# Clean and rebuild
cargo clean && cargo build

# Run with verbose output
cargo test -- --nocapture --test-threads=1

# Check dependencies
cargo tree
```

## License

MIT License
