# Fee Taker

## Overview

The Fee Taker contract is an extension contract for the Limit Order Protocol that handles fee collection and distribution during order execution. It provides flexible fee structures and supports both native NEAR and fungible token fee collection.

## Contract Structure

### Main Contract: `FeeTaker`

```rust
pub struct FeeTaker {
    limit_order_protocol: AccountId,
    access_token: AccountId,
    weth: AccountId,
    owner: AccountId,
}
```

### Key Components

- **Limit Order Protocol**: Address of the main limit order protocol
- **Access Token**: Token required for access to certain functions
- **WETH**: Wrapped NEAR token address
- **Owner**: Contract administrator

## Core Functionality

### Fee Collection

#### Post Interaction

```rust
pub fn post_interaction(
    &mut self,
    order: Order,
    extension: Vec<u8>,
    order_hash: [u8; 32],
    taker: AccountId,
    making_amount: u128,
    taking_amount: u128,
    remaining_making_amount: u128,
    extra_data: Vec<u8>,
) -> Result<(), FeeTakerError>
```

**Process:**

1. Validate caller is limit order protocol
2. Parse fee configuration from extension
3. Validate fee configuration
4. Check if fee is applicable
5. Transfer tokens with or without fee
6. Log fee collection

#### Fee Configuration

```rust
pub struct FeeConfig {
    pub fee_recipient: AccountId,    // Fee recipient address
    pub fee_amount: u128,            // Fee amount
    pub fee_token: AccountId,        // Fee token address
    pub fee_percentage: u32,         // Fee percentage (basis points)
}
```

### Fee Validation

#### Configuration Validation

```rust
fn validate_fee_config(fee_config: &FeeConfig, receiver: &AccountId) -> Result<(), FeeTakerError>
```

- **Recipient Validation**: Ensures fee recipient is valid
- **Amount Validation**: Validates fee amounts are reasonable
- **Token Validation**: Ensures fee token is supported
- **Percentage Validation**: Validates fee percentages

#### Applicability Check

```rust
fn is_fee_applicable(fee_config: &FeeConfig) -> bool
```

- **Zero Fee Check**: Determines if fee should be collected
- **Configuration Check**: Validates fee configuration
- **Token Support**: Checks if fee token is supported

### Token Transfer

#### Fee Transfer

```rust
fn transfer_tokens_with_fee(
    token: &AccountId,
    from: &AccountId,
    to: &AccountId,
    amount: u128,
    fee_config: &FeeConfig,
) -> Result<(), FeeTakerError>
```

**Process:**

1. Calculate fee amount
2. Transfer tokens to recipient
3. Transfer remaining tokens to taker
4. Handle fee collection

#### Regular Transfer

```rust
fn transfer_tokens_without_fee(
    token: &AccountId,
    to: &AccountId,
    amount: u128,
) -> Result<(), FeeTakerError>
```

- **Direct Transfer**: Transfer tokens without fee collection
- **Gas Optimization**: Efficient token transfers
- **Error Handling**: Comprehensive error management

## Fee Structures

### Percentage-based Fees

```rust
fn calculate_percentage_fee(amount: u128, percentage: u32) -> u128
```

- **Basis Points**: Fee calculated in basis points (1/10000)
- **Precision**: High precision fee calculation
- **Rounding**: Proper rounding for fee amounts

### Fixed Amount Fees

```rust
fn calculate_fixed_fee(amount: u128, fee_amount: u128) -> u128
```

- **Fixed Amount**: Fixed fee amount regardless of trade size
- **Minimum Fee**: Ensures minimum fee collection
- **Maximum Fee**: Prevents excessive fee collection

### Hybrid Fees

```rust
fn calculate_hybrid_fee(amount: u128, fee_config: &FeeConfig) -> u128
```

- **Combination**: Percentage + fixed amount fees
- **Flexibility**: Supports complex fee structures
- **Optimization**: Gas-efficient fee calculation

## Security Features

### Access Control

#### Protocol Validation

```rust
fn validate_limit_order_protocol(caller: &AccountId, protocol: &AccountId) -> Result<(), FeeTakerError>
```

- **Caller Validation**: Only limit order protocol can call
- **Protocol Verification**: Ensures correct protocol address
- **Security**: Prevents unauthorized fee collection

#### Owner Functions

```rust
pub fn rescue_funds(&mut self, token: AccountId, amount: u128) -> Result<(), FeeTakerError>
```

- **Owner Only**: Only owner can rescue funds
- **Emergency Function**: Emergency fund recovery
- **Security**: Prevents unauthorized fund access

### Error Handling

```rust
pub enum FeeTakerError {
    InvalidCaller,
    InvalidFeeConfig,
    FeeNotApplicable,
    TransferFailed,
    InvalidAmount,
    UnauthorizedAccess,
}
```

## Integration Points

### External Dependencies

- **Limit Order Protocol**: Main protocol contract
- **Fungible Token Standard**: Token transfers
- **NEAR Protocol**: Base blockchain infrastructure

### Internal Dependencies

- **Order Execution**: Fee collection during order execution
- **Token Transfers**: Fee and token transfer handling
- **Extension System**: Fee configuration parsing

## Usage Examples

### Creating Fee Taker

```rust
let fee_taker = FeeTaker::new(
    "limit-order.near".parse().unwrap(),
    "access-token.near".parse().unwrap(),
    "weth.near".parse().unwrap(),
);
```

### Fee Configuration

```rust
let fee_config = FeeConfig {
    fee_recipient: "fee-recipient.near".parse().unwrap(),
    fee_amount: 1000_000_000, // 1000 tokens
    fee_token: "usdc.near".parse().unwrap(),
    fee_percentage: 30, // 0.3%
};
```

### Post Interaction Call

```rust
let result = fee_taker.post_interaction(
    order,
    extension,
    order_hash,
    "taker.near".parse().unwrap(),
    making_amount,
    taking_amount,
    remaining_making_amount,
    extra_data,
);
```

### Rescuing Funds

```rust
let result = fee_taker.rescue_funds(
    "usdc.near".parse().unwrap(),
    1000_000_000, // 1000 USDC
);
```

## Gas Optimization

### Efficient Operations

- **Minimal Storage**: Only essential data stored
- **Optimized Transfers**: Efficient token transfers
- **Batch Processing**: Support for batch operations

### Gas Costs

- **Fee Collection**: ~40,000 gas
- **Token Transfer**: ~20,000 gas
- **Configuration Parsing**: ~10,000 gas
- **Validation**: ~5,000 gas

## Testing

### Unit Tests

- Fee calculation accuracy
- Configuration validation
- Token transfer scenarios
- Error condition handling

### Integration Tests

- End-to-end fee collection
- Cross-contract interactions
- Gas consumption validation
- Error recovery scenarios

## Monitoring

### Key Events

- `FeeCollected`: Fee collection events
- `TokensTransferred`: Token transfer events
- `FundsRescued`: Fund rescue events
- `ConfigurationUpdated`: Fee configuration updates

### Metrics

- Fee collection rate
- Fee amounts collected
- Token transfer success rate
- Gas consumption patterns
- Most active fee recipients

## Fee Strategies

### Dynamic Fees

- **Volume-based**: Fees based on trading volume
- **Time-based**: Fees based on time of day
- **User-based**: Fees based on user tier
- **Token-based**: Different fees for different tokens

### Fee Optimization

- **Gas Efficiency**: Minimize gas costs for fee collection
- **User Experience**: Balance fee collection with user experience
- **Competitive Pricing**: Competitive fee structures
- **Revenue Optimization**: Maximize fee revenue

## Security Considerations

### Best Practices

1. **Fee Validation**: Comprehensive fee configuration validation
2. **Access Control**: Strict caller validation
3. **Error Handling**: Comprehensive error management
4. **Emergency Functions**: Emergency fund recovery mechanisms

### Risk Mitigation

- **Fee Limits**: Prevent excessive fee collection
- **Access Control**: Restrict sensitive operations
- **Validation**: Validate all inputs and configurations
- **Monitoring**: Monitor fee collection patterns
