# Cross-Chain Swap NEAR Contracts

This directory contains the NEAR Rust implementation of cross-chain atomic swap contracts, migrated from the original Solidity contracts.

## Overview

The cross-chain swap system enables atomic swaps between different blockchain networks using hashlock-based escrow contracts. This implementation provides:

- **BaseEscrow**: Core escrow functionality with rescue mechanisms
- **EscrowSrc**: Source chain escrow for initiating swaps
- **EscrowDst**: Destination chain escrow for completing swaps
- **EscrowFactory**: Factory contract for deploying new escrow instances

## Architecture

### Core Components

1. **BaseEscrow** (`src/base_escrow.rs`)

   - Provides fundamental escrow functionality
   - Handles token transfers and rescue operations
   - Validates secrets and access tokens

2. **EscrowSrc** (`src/escrow_src.rs`)

   - Source chain escrow implementation
   - Handles withdrawal, cancellation, and public operations
   - Inherits from BaseEscrow

3. **EscrowDst** (`src/escrow_dst.rs`)

   - Destination chain escrow implementation
   - Handles withdrawal, cancellation, and public operations
   - Inherits from BaseEscrow

4. **EscrowFactory** (`src/escrow_factory.rs`)
   - Factory contract for deploying escrow instances
   - Manages escrow creation and validation
   - Handles partial fill validation

### Data Structures

- **Immutables**: Core swap parameters (order hash, hashlock, participants, amounts)
- **Timelocks**: Time-based constraints for different operations
- **TimelockStage**: Enum defining different timelock stages
- **ValidationData**: Validation results and error information

## Key Features

### Atomic Swap Flow

1. **Initiation**: User creates escrow on source chain with hashlock
2. **Escrow Creation**: Factory deploys destination escrow
3. **Secret Revelation**: Taker reveals secret to unlock funds
4. **Completion**: Funds are transferred atomically

### Security Features

- **Hashlock-based**: Uses cryptographic hashlocks for atomicity
- **Timelock Protection**: Time-based constraints prevent stuck funds
- **Rescue Mechanisms**: Emergency withdrawal options
- **Access Control**: Token-based access control for public operations

### NEAR-Specific Adaptations

- **AccountId**: Uses NEAR account IDs instead of Ethereum addresses
- **NearToken**: Native NEAR token handling
- **Gas Management**: NEAR gas optimization
- **Cross-Contract Calls**: NEAR-specific contract interaction patterns

## Usage

### Building

```bash
cargo build --target wasm32-unknown-unknown
```

### Testing

```bash
cargo test
```

### Deployment

The contracts can be deployed to NEAR testnet or mainnet using standard NEAR deployment tools.

## Migration Notes

### From Solidity to Rust

1. **Type System**: Converted Solidity types to Rust equivalents

   - `address` → `AccountId`
   - `uint256` → `u128`
   - `bytes32` → `[u8; 32]`

2. **Error Handling**: Replaced Solidity reverts with Rust `Result<T, E>`

   - Added `#[handle_result]` attributes for proper error handling
   - Used `Abort` for critical failures

3. **State Management**: Adapted to NEAR's state management patterns

   - Used `#[near(contract_state)]` for contract state
   - Implemented proper serialization with Borsh

4. **Gas Optimization**: Optimized for NEAR's gas model
   - Used `Gas::from_tgas()` for gas calculations
   - Implemented efficient cross-contract calls

### Key Differences from Solidity Version

- **Deterministic Addresses**: Simplified address computation for NEAR
- **Token Standards**: Adapted for NEAR's fungible token standard
- **Event System**: Replaced Solidity events with NEAR logs
- **Access Control**: Simplified access token validation for NEAR

## Testing Status

✅ **Unit Tests**: All core functionality tested and passing

- BaseEscrow initialization and methods
- EscrowSrc deployment and operations
- EscrowDst deployment and operations
- EscrowFactory deployment and validation

## Next Steps

1. **Integration Testing**: Add comprehensive integration tests
2. **Gas Optimization**: Further optimize gas usage
3. **Security Audit**: Conduct thorough security review
4. **Documentation**: Add detailed API documentation
5. **Deployment Scripts**: Create deployment automation

## Contributing

When contributing to this project:

1. Ensure all tests pass: `cargo test`
2. Follow NEAR development best practices
3. Add appropriate error handling
4. Update documentation for any changes

## License

This project follows the same license as the original Solidity contracts.
