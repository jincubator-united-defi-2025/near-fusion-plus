# Deployment Guide

## Overview

This guide provides step-by-step instructions for deploying NEAR Fusion+ contracts to the NEAR blockchain. It covers both testnet and mainnet deployments, including configuration, verification, and post-deployment steps.

## Prerequisites

### Development Environment

- **Rust**: Latest stable version (see `rust-toolchain.toml`)
- **cargo-near**: NEAR smart contract development toolkit
- **NEAR CLI**: Command line interface for NEAR
- **NEAR Account**: NEAR account with sufficient balance

### Installation

```bash
# Install cargo-near
cargo install cargo-near

# Install NEAR CLI
npm install -g near-cli

# Verify installation
cargo near --version
near --version
```

### Account Setup

```bash
# Create NEAR account (if needed)
near create-account <account-id> --masterAccount <master-account>

# Login to NEAR
near login

# Check account balance
near state <account-id>
```

## Build Configuration

### 1. Build All Contracts

```bash
# Build all contracts
cargo near build

# Build specific contract
cd src/limit-order-protocol
cargo near build
```

### 2. Build Configuration

#### Cargo.toml Configuration

```toml
[package]
name = "limit-order-protocol"
version = "0.1.0"
edition = "2021"

[dependencies]
near-sdk = "5.15.1"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.release.build-override]
opt-level = 3
lto = false
codegen-units = 16
```

#### Rust Toolchain

```toml
[toolchain]
channel = "1.75.0"
components = ["rustfmt", "clippy"]
targets = ["wasm32-unknown-unknown"]
```

### 3. Build Verification

```bash
# Check build artifacts
ls -la target/wasm32-unknown-unknown/release/

# Verify WASM file size
wc -c target/wasm32-unknown-unknown/release/*.wasm

# Validate WASM file
wasm-validate target/wasm32-unknown-unknown/release/*.wasm
```

## Testnet Deployment

### 1. Testnet Configuration

```bash
# Set testnet configuration
export NEAR_ENV=testnet

# Set deployment account
export DEPLOY_ACCOUNT=your-testnet-account.testnet

# Verify configuration
near state $DEPLOY_ACCOUNT
```

### 2. Deploy Contracts

#### Limit Order Protocol

```bash
# Deploy limit order protocol
cargo near deploy build-non-reproducible-wasm $DEPLOY_ACCOUNT \
  --args '{"domain_separator": "00000000000000000000000000000000", "weth": "weth.testnet"}'

# Verify deployment
near view $DEPLOY_ACCOUNT get_owner
```

#### Escrow Factory

```bash
# Deploy escrow factory
cargo near deploy build-non-reproducible-wasm $DEPLOY_ACCOUNT \
  --args '{"limit_order_protocol": "limit-order.testnet", "fee_token": "fee-token.testnet", "access_token": "access-token.testnet", "rescue_delay_src": 3600, "rescue_delay_dst": 3600}'

# Verify deployment
near view $DEPLOY_ACCOUNT get_limit_order_protocol
```

#### Fee Taker

```bash
# Deploy fee taker
cargo near deploy build-non-reproducible-wasm $DEPLOY_ACCOUNT \
  --args '{"limit_order_protocol": "limit-order.testnet", "access_token": "access-token.testnet", "weth": "weth.testnet"}'

# Verify deployment
near view $DEPLOY_ACCOUNT get_limit_order_protocol
```

#### Merkle Storage Invalidator

```bash
# Deploy merkle storage invalidator
cargo near deploy build-non-reproducible-wasm $DEPLOY_ACCOUNT \
  --args '{"limit_order_protocol": "limit-order.testnet"}'

# Verify deployment
near view $DEPLOY_ACCOUNT get_limit_order_protocol
```

### 3. Initialize Contracts

#### Set Contract Parameters

```bash
# Set domain separator for limit order protocol
near call $DEPLOY_ACCOUNT set_domain_separator \
  '{"domain_separator": "00000000000000000000000000000000"}' \
  --accountId $DEPLOY_ACCOUNT

# Set fee configuration
near call $DEPLOY_ACCOUNT set_fee_config \
  '{"fee_recipient": "fee-recipient.testnet", "fee_percentage": 30}' \
  --accountId $DEPLOY_ACCOUNT
```

### 4. Testnet Verification

```bash
# Run integration tests
cd integration-tests
cargo test

# Test contract interactions
near call $DEPLOY_ACCOUNT test_interaction \
  '{"order": {...}, "extension": {...}}' \
  --accountId $DEPLOY_ACCOUNT
```

## Mainnet Deployment

### 1. Mainnet Preparation

#### Security Checklist

- [ ] All tests passing
- [ ] Security audit completed
- [ ] Gas optimization verified
- [ ] Documentation updated
- [ ] Emergency procedures documented

#### Mainnet Configuration

```bash
# Set mainnet configuration
export NEAR_ENV=mainnet

# Set deployment account
export DEPLOY_ACCOUNT=your-mainnet-account.near

# Verify account balance
near state $DEPLOY_ACCOUNT
```

### 2. Mainnet Deployment

#### Reproducible Builds

```bash
# Build reproducible WASM
cargo near build --release

# Verify build reproducibility
cargo near build --release
# Compare hashes between builds
```

#### Deploy Contracts

```bash
# Deploy with reproducible builds
cargo near deploy build-reproducible-wasm $DEPLOY_ACCOUNT \
  --args '{"domain_separator": "00000000000000000000000000000000", "weth": "weth.near"}'

# Verify deployment
near view $DEPLOY_ACCOUNT get_owner
```

### 3. Mainnet Verification

#### Contract Verification

```bash
# Verify contract bytecode
near view $DEPLOY_ACCOUNT get_contract_hash

# Verify contract state
near view $DEPLOY_ACCOUNT get_owner
near view $DEPLOY_ACCOUNT is_paused
```

#### Integration Testing

```bash
# Test with small amounts
near call $DEPLOY_ACCOUNT test_small_order \
  '{"order": {...}, "amount": "1000000000000000000000000"}' \
  --accountId $DEPLOY_ACCOUNT \
  --deposit 0.001
```

## Post-Deployment

### 1. Contract Verification

#### Bytecode Verification

```bash
# Get contract bytecode hash
near view $DEPLOY_ACCOUNT get_contract_hash

# Verify against source
cargo near build --release
sha256sum target/wasm32-unknown-unknown/release/*.wasm
```

#### State Verification

```bash
# Verify contract state
near view $DEPLOY_ACCOUNT get_owner
near view $DEPLOY_ACCOUNT get_limit_order_protocol
near view $DEPLOY_ACCOUNT get_fee_token
```

### 2. Configuration Verification

#### Parameter Verification

```bash
# Verify domain separator
near view $DEPLOY_ACCOUNT domain_separator

# Verify fee configuration
near view $DEPLOY_ACCOUNT get_fee_config

# Verify timelock configuration
near view $DEPLOY_ACCOUNT get_timelock_config
```

### 3. Integration Testing

#### End-to-End Testing

```bash
# Test order creation
near call $DEPLOY_ACCOUNT create_order \
  '{"order": {...}, "signature": "..."}' \
  --accountId $DEPLOY_ACCOUNT

# Test order execution
near call $DEPLOY_ACCOUNT fill_order \
  '{"order": {...}, "extension": {...}, "signature": "...", "taker": "taker.near", "taking_amount": "1000000000000000000000000"}' \
  --accountId $DEPLOY_ACCOUNT
```

## Monitoring and Maintenance

### 1. Contract Monitoring

#### Event Monitoring

```bash
# Monitor contract events
near view $DEPLOY_ACCOUNT get_events

# Monitor gas usage
near view $DEPLOY_ACCOUNT get_gas_usage
```

#### Performance Monitoring

```bash
# Monitor transaction success rate
near view $DEPLOY_ACCOUNT get_stats

# Monitor gas costs
near view $DEPLOY_ACCOUNT get_gas_stats
```

### 2. Emergency Procedures

#### Pause Contract

```bash
# Emergency pause
near call $DEPLOY_ACCOUNT pause \
  '{}' \
  --accountId $DEPLOY_ACCOUNT

# Verify pause
near view $DEPLOY_ACCOUNT is_paused
```

#### Rescue Funds

```bash
# Rescue stuck funds
near call $DEPLOY_ACCOUNT rescue_funds \
  '{"token": "token.near", "amount": "1000000000000000000000000"}' \
  --accountId $DEPLOY_ACCOUNT
```

### 3. Upgrades and Maintenance

#### Contract Upgrades

```bash
# Deploy new version
cargo near deploy build-reproducible-wasm $DEPLOY_ACCOUNT \
  --args '{"upgrade": true}'

# Verify upgrade
near view $DEPLOY_ACCOUNT get_version
```

#### Configuration Updates

```bash
# Update fee configuration
near call $DEPLOY_ACCOUNT update_fee_config \
  '{"fee_percentage": 25}' \
  --accountId $DEPLOY_ACCOUNT

# Update timelock configuration
near call $DEPLOY_ACCOUNT update_timelock_config \
  '{"rescue_delay": 7200}' \
  --accountId $DEPLOY_ACCOUNT
```

## Security Considerations

### 1. Access Control

#### Owner Management

```bash
# Transfer ownership (if needed)
near call $DEPLOY_ACCOUNT transfer_ownership \
  '{"new_owner": "new-owner.near"}' \
  --accountId $DEPLOY_ACCOUNT

# Verify ownership transfer
near view $DEPLOY_ACCOUNT get_owner
```

#### Multi-signature Setup

```bash
# Set up multi-signature
near call $DEPLOY_ACCOUNT set_multisig \
  '{"signers": ["signer1.near", "signer2.near", "signer3.near"], "threshold": 2}' \
  --accountId $DEPLOY_ACCOUNT
```

### 2. Security Monitoring

#### Suspicious Activity Detection

```bash
# Monitor for suspicious transactions
near view $DEPLOY_ACCOUNT get_suspicious_activity

# Monitor for failed transactions
near view $DEPLOY_ACCOUNT get_failed_transactions
```

#### Security Alerts

```bash
# Set up security alerts
near call $DEPLOY_ACCOUNT set_security_alerts \
  '{"enabled": true, "threshold": 1000000000000000000000000}' \
  --accountId $DEPLOY_ACCOUNT
```

## Troubleshooting

### 1. Common Issues

#### Build Issues

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo near build
```

#### Deployment Issues

```bash
# Check account balance
near state $DEPLOY_ACCOUNT

# Check gas costs
near view $DEPLOY_ACCOUNT get_gas_estimate

# Retry deployment
cargo near deploy build-non-reproducible-wasm $DEPLOY_ACCOUNT
```

#### Runtime Issues

```bash
# Check contract state
near view $DEPLOY_ACCOUNT get_state

# Check recent transactions
near view $DEPLOY_ACCOUNT get_recent_transactions

# Check error logs
near view $DEPLOY_ACCOUNT get_error_logs
```

### 2. Debugging

#### Contract Debugging

```bash
# Enable debug mode
export NEAR_DEBUG=1

# Run with debug output
cargo near deploy build-non-reproducible-wasm $DEPLOY_ACCOUNT --debug
```

#### Log Analysis

```bash
# Get contract logs
near view $DEPLOY_ACCOUNT get_logs

# Analyze transaction logs
near view $DEPLOY_ACCOUNT get_transaction_logs
```

## Best Practices

### 1. Deployment Best Practices

- **Testnet First**: Always deploy to testnet first
- **Gradual Rollout**: Deploy with small amounts initially
- **Monitoring**: Set up comprehensive monitoring
- **Documentation**: Document all deployment steps

### 2. Security Best Practices

- **Access Control**: Implement proper access controls
- **Emergency Procedures**: Have emergency procedures ready
- **Monitoring**: Monitor for suspicious activity
- **Updates**: Keep contracts updated

### 3. Maintenance Best Practices

- **Regular Audits**: Conduct regular security audits
- **Performance Monitoring**: Monitor performance metrics
- **User Support**: Provide user support
- **Documentation**: Keep documentation updated
