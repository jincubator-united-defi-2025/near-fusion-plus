# Cross-Chain Swap Contracts for NEAR

This directory contains the NEAR Rust implementation of cross-chain atomic swap contracts, migrated from the original Solidity contracts in the `1inch` directory.

## Overview

The cross-chain swap system enables atomic swaps between different blockchain networks using time-locked escrow contracts. The system consists of:

- **BaseEscrow**: Abstract base contract providing core escrow functionality
- **EscrowSrc**: Source chain escrow contract for locking initial funds
- **EscrowDst**: Destination chain escrow contract for receiving swapped funds
- **EscrowFactory**: Factory contract for creating and managing escrow contracts

## Contract Architecture

### BaseEscrow
The base contract provides:
- Fund rescue functionality
- Secret validation
- Token transfer utilities
- Access control mechanisms

### EscrowSrc (Source Chain)
Handles the initial locking of funds on the source chain:
- `withdraw()`: Taker can withdraw with secret during withdrawal period
- `withdraw_to()`: Withdraw to specific target address
- `public_withdraw()`: Anyone with access token can withdraw during public period
- `cancel()`: Taker can cancel during cancellation period
- `public_cancel()`: Anyone with access token can cancel during public period

### EscrowDst (Destination Chain)
Handles the destination chain escrow:
- `withdraw()`: Taker can withdraw with secret during withdrawal period
- `public_withdraw()`: Anyone with access token can withdraw during public period
- `cancel()`: Taker can cancel during cancellation period

### EscrowFactory
Manages escrow contract creation:
- `create_dst_escrow()`: Creates destination escrow
- `post_interaction()`: Handles source escrow creation
- `address_of_escrow_src()`: Computes source escrow address
- `address_of_escrow_dst()`: Computes destination escrow address

## Key Features

### Time-Locked Operations
All operations are time-locked using the `Timelocks` structure:
- Withdrawal periods (private and public)
- Cancellation periods (private and public)
- Rescue periods

### Deterministic Addresses
Escrow contracts are deployed at deterministic addresses computed from:
- Order hash
- Hashlock (secret hash)
- Maker and taker addresses
- Token and amount information
- Timelocks

### Secret-Based Withdrawal
Funds can only be withdrawn by providing the correct secret that matches the hashlock.

### Cross-Chain Atomic Swaps
The system enables atomic swaps across different blockchain networks by coordinating escrow contracts on source and destination chains.

## Usage

### Building the Contracts

```bash
cd src/cross-chain-swap
cargo near build
```

### Running Tests

```bash
cargo test
```

### Deploying Contracts

```bash
# Deploy for debugging
cargo near deploy build-non-reproducible-wasm <account-id>

# Deploy for production
cargo near deploy build-reproducible-wasm <account-id>
```

## Migration Notes

### Key Differences from Solidity

1. **Account System**: Uses NEAR's account-based system instead of Ethereum addresses
2. **Token Standards**: Uses NEAR's Fungible Token (FT) standard instead of ERC20
3. **Gas Model**: Uses NEAR's gas model instead of Ethereum's
4. **Storage**: Uses NEAR's storage model with Borsh serialization
5. **Cross-Contract Calls**: Uses NEAR's cross-contract call mechanism

### Simplified Implementations

Some complex Ethereum-specific features have been simplified for NEAR:
- Deterministic address computation (placeholder implementation)
- Access token validation (simplified)
- Contract deployment (placeholder implementation)

### Future Enhancements

1. Implement proper deterministic address computation
2. Add comprehensive access token validation
3. Implement proper contract deployment mechanism
4. Add more comprehensive error handling
5. Implement proper cross-chain communication

## Security Considerations

- All time-based validations are enforced
- Secret validation prevents unauthorized withdrawals
- Access control mechanisms protect sensitive operations
- Deterministic addresses prevent address spoofing

## Testing

The contracts include comprehensive unit tests and integration tests:

- Unit tests for each contract's core functionality
- Integration tests for cross-contract interactions
- Time-based validation tests
- Secret validation tests

Run tests with:
```bash
cargo test
```

## License

This project is licensed under the MIT License. 