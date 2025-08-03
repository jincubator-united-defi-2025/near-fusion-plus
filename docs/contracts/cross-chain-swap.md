# Cross-Chain Swap

## Overview

The Cross-Chain Swap contract provides the foundation for atomic swaps across different blockchains. It implements a base escrow system that can be extended for specific cross-chain scenarios.

## Contract Structure

### Main Contract: `BaseEscrow`

```rust
pub struct BaseEscrow {
    rescue_delay: u64,
    access_token: AccountId,
    factory: AccountId,
}
```

### Key Components

- **Rescue Delay**: Time delay before funds can be rescued
- **Access Token**: Token required for access to certain functions
- **Factory**: Address of the factory that created this escrow

## Core Functionality

### Escrow Management

#### Immutables Structure

```rust
pub struct Immutables {
    pub taker: AccountId,           // Taker address
    pub hashlock: [u8; 32],        // Hash of the secret
    pub timelocks: Timelocks,       // Time-based constraints
    pub maker_asset: AccountId,     // Asset being sold
    pub taker_asset: AccountId,     // Asset being bought
    pub maker_amount: u128,         // Amount being sold
    pub taker_amount: u128,         // Amount being bought
}
```

#### Timelocks Structure

```rust
pub struct Timelocks {
    pub src_withdrawal: u64,        // Source withdrawal time
    pub src_cancellation: u64,      // Source cancellation time
    pub dst_withdrawal: u64,        // Destination withdrawal time
    pub dst_cancellation: u64,      // Destination cancellation time
}
```

### Secret Management

#### Hash Generation

```rust
fn hash_secret(secret: &[u8; 32]) -> [u8; 32]
```

- **Purpose**: Generate hashlock from secret
- **Algorithm**: SHA-256 hash function
- **Usage**: Used for atomic swap completion

#### Secret Validation

```rust
pub fn validate_secret(&self, secret: &[u8; 32], immutables: &Immutables) -> Result<(), Abort>
```

- **Process**: Hash secret and compare with hashlock
- **Validation**: Ensures secret matches the expected hashlock
- **Security**: Prevents unauthorized fund withdrawal

### Fund Management

#### Rescue Mechanism

```rust
pub fn rescue_funds(
    &mut self,
    token: AccountId,
    amount: u128,
    immutables: Immutables,
) -> Result<(), Abort>
```

**Process:**

1. Validate caller is taker
2. Check rescue delay has passed
3. Transfer tokens to taker
4. Log rescue operation

#### Token Transfer

```rust
pub fn uni_transfer(&self, token: &AccountId, to: &AccountId, amount: u128)
```

- **NEAR Transfer**: Native NEAR token transfers
- **Fungible Token Transfer**: FT standard token transfers
- **Gas Optimization**: Efficient cross-contract calls

### Access Control

#### Access Token Validation

```rust
pub fn validate_access_token(&self) -> Result<(), Abort>
```

- **Purpose**: Validate access token ownership
- **Usage**: Restrict access to sensitive functions
- **Security**: Prevent unauthorized operations

#### Caller Validation

```rust
fn validate_caller(caller: &AccountId) -> Result<(), Abort>
```

- **Purpose**: Validate function caller
- **Usage**: Ensure only authorized parties can call functions
- **Security**: Prevent unauthorized access

## Security Features

### Time-based Security

#### Rescue Delay

- **Purpose**: Prevent immediate fund rescue
- **Duration**: Configurable delay period
- **Security**: Gives time for proper swap completion

#### Timelock Validation

```rust
fn validate_after(timestamp: u64) -> Result<(), Abort>
```

- **Purpose**: Ensure sufficient time has passed
- **Usage**: Validate rescue delay requirements
- **Security**: Prevent premature fund access

### Access Control

#### Taker-only Functions

- **Rescue Funds**: Only taker can rescue funds
- **Secret Validation**: Only taker can validate secrets
- **Security**: Prevents unauthorized fund access

#### Access Token Requirements

- **Validation Functions**: Require access token
- **Public Functions**: May require access token
- **Security**: Restrict sensitive operations

## Integration Points

### External Dependencies

- **NEAR Protocol**: Base blockchain infrastructure
- **Fungible Token Standard**: FT token transfers
- **Factory Contract**: Escrow creation and management

### Internal Dependencies

- **Escrow Factory**: Creates and manages escrows
- **Source/Destination Escrows**: Extend base functionality
- **Limit Order Protocol**: Triggers escrow creation

## Usage Examples

### Creating an Escrow

```rust
let escrow = BaseEscrow::new(
    3600, // 1 hour rescue delay
    "access-token.near".parse().unwrap(),
);
```

### Validating a Secret

```rust
let secret = [1u8; 32];
let result = escrow.validate_secret(&secret, &immutables);
```

### Rescuing Funds

```rust
let result = escrow.rescue_funds(
    "usdc.near".parse().unwrap(),
    1000_000_000, // 1000 USDC
    immutables,
);
```

### Transferring Tokens

```rust
// NEAR transfer
escrow.uni_transfer(
    &"near".parse().unwrap(),
    &"alice.near".parse().unwrap(),
    1_000_000_000_000_000_000_000_000, // 1 NEAR
);

// Fungible token transfer
escrow.uni_transfer(
    &"usdc.near".parse().unwrap(),
    &"bob.near".parse().unwrap(),
    1000_000_000, // 1000 USDC
);
```

## Error Handling

### Abort Conditions

- **Invalid Caller**: Unauthorized function access
- **Time Validation**: Insufficient time passed
- **Secret Mismatch**: Invalid secret provided
- **Token Transfer Failure**: Failed token transfers

### Error Types

```rust
pub enum Abort {
    InvalidCaller,
    TimeNotElapsed,
    InvalidSecret,
    TransferFailed,
}
```

## Gas Optimization

### Efficient Operations

- **Minimal Storage**: Only essential data stored
- **Optimized Transfers**: Efficient cross-contract calls
- **Lazy Validation**: On-demand validation checks

### Gas Costs

- **Escrow Creation**: ~30,000 gas
- **Secret Validation**: ~10,000 gas
- **Fund Rescue**: ~50,000 gas
- **Token Transfer**: ~20,000 gas

## Testing

### Unit Tests

- Escrow creation and initialization
- Secret validation
- Fund rescue mechanisms
- Access control validation

### Integration Tests

- Cross-contract interactions
- Token transfer scenarios
- Time-based operations
- Error condition handling

## Monitoring

### Key Events

- `EscrowCreated`: Escrow creation events
- `SecretValidated`: Secret validation events
- `FundsRescued`: Fund rescue events
- `TokensTransferred`: Token transfer events

### Metrics

- Escrow creation rate
- Secret validation success rate
- Fund rescue frequency
- Gas consumption patterns

## Security Considerations

### Best Practices

1. **Secret Management**: Secure secret generation and storage
2. **Time Validation**: Proper timelock implementation
3. **Access Control**: Strict caller validation
4. **Error Handling**: Comprehensive error management

### Risk Mitigation

- **Rescue Delays**: Prevent immediate fund access
- **Access Tokens**: Restrict sensitive operations
- **Caller Validation**: Ensure authorized access
- **Time Checks**: Validate timing requirements
