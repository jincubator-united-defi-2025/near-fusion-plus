# Cross-chain Swap Contracts

Cross-chain atomic swap contracts for NEAR, migrated from Solidity contracts.

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
# Create a new account
cargo near create-dev-account

# Deploy the contract on it
cargo near deploy <account-id>
```

## Contract Overview

This project contains NEAR Rust implementations of cross-chain atomic swap contracts:

### BaseEscrow Contract

The foundational contract that provides common functionalities for atomic swaps:

- **Rescue Funds**: Allows recovery of funds after timelock expiration
- **Token Transfers**: Handles both native NEAR and fungible token transfers
- **Validation**: Validates secrets, access tokens, and immutables
- **Timelock Management**: Manages withdrawal and cancellation timelocks

### Key Features

- **Cross-chain Compatibility**: Designed for atomic swaps between different blockchains
- **Security**: Implements hashlock-based security with timelock mechanisms
- **Flexibility**: Supports both native NEAR and fungible token transfers
- **Gas Optimization**: Efficient gas usage for cross-contract calls

### NEAR-Specific Adaptations

- Uses NEAR's native token system and AccountId types
- Implements NEAR's cross-contract call patterns
- Leverages NEAR's gas management and Promise system
- Follows NEAR's serialization standards (Borsh)

## Architecture

The project is structured to support multiple contracts:

- `BaseEscrow`: Core escrow functionality (currently implemented)
- `EscrowSrc`: Source chain escrow (planned)
- `EscrowDst`: Destination chain escrow (planned)
- `EscrowFactory`: Factory for creating escrow contracts (planned)

## Testing Status

- ✅ Unit tests for BaseEscrow contract
- ✅ Integration tests for contract deployment and basic functionality
- 🔄 Additional contracts implementation (in progress)

## Next Steps

1. Implement remaining contracts (EscrowSrc, EscrowDst, EscrowFactory)
2. Add comprehensive integration tests
3. Implement advanced validation logic
4. Add gas optimization features

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
