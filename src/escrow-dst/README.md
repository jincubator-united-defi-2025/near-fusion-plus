# EscrowDst Contract

Destination Escrow contract for cross-chain atomic swap on NEAR Protocol.

## Overview

This contract implements the destination chain escrow functionality for cross-chain atomic swaps. It allows users to lock funds on the destination chain and withdraw them using a secret hash, or cancel the escrow after certain timelocks.

## Features

- **Atomic Swaps**: Lock funds on destination chain for cross-chain trading
- **Timelock Management**: Enforce time-based restrictions for withdrawals and cancellations
- **Secret Validation**: Verify withdrawal secrets against hashlocks
- **Access Control**: Restrict operations to authorized parties
- **Token Support**: Support for both native NEAR and fungible tokens

## Contract Functions

### Core Functions

- `withdraw(secret, immutables)` - Withdraw funds using secret (taker only)
- `public_withdraw(secret, immutables)` - Public withdrawal (access token holder only)
- `cancel(immutables)` - Cancel escrow (taker only)

### View Functions

- `get_rescue_delay()` - Get rescue delay setting
- `get_access_token()` - Get access token address
- `get_owner()` - Get contract owner

## Timelock Stages

The contract enforces the following timelock stages:

1. **DstWithdrawal** - Private withdrawal period
2. **DstPublicWithdrawal** - Public withdrawal period
3. **DstCancellation** - Private cancellation period

## Key Differences from EscrowSrc

- **Withdrawal Target**: Funds are withdrawn to the maker (not taker)
- **Cancellation Target**: Funds are returned to the taker (not maker)
- **Simplified Interface**: No `withdraw_to` function (always withdraws to maker)
- **No Public Cancel**: Only taker can cancel (no public cancellation)

## Usage

### Deployment

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown

# Deploy to NEAR
near deploy <account-id> target/wasm32-unknown-unknown/release/escrow_dst.wasm
```

### Initialization

```bash
# Initialize with rescue delay and access token
near call <contract-id> new '{"rescue_delay": 3600, "access_token": "token.near"}' --account-id <account-id>
```

### Testing

```bash
# Run unit tests
cargo test --lib

# Run integration tests
cargo test
```

## Security

- All operations are validated against timelocks
- Secrets are verified against hashlocks using keccak256
- Access control ensures only authorized parties can perform operations
- Immutable values are validated before processing

## Migration from Solidity

This contract is migrated from the Solidity `EscrowDst.sol` contract, adapted for NEAR Protocol's architecture and conventions.

## License

MIT License
