# Deployment Scripts

Automated deployment and verification scripts for NEAR cross-chain swap contracts.

## Overview

This directory contains scripts to automate the deployment and verification of all migrated NEAR contracts:

- **Base Escrow**: Core escrow functionality
- **Escrow Source**: Source chain escrow contract
- **Escrow Destination**: Destination chain escrow contract
- **Escrow Factory**: Factory for creating escrows
- **Limit Order Protocol**: Limit order protocol contract

## Scripts

### `deploy.sh`

Automated deployment script that builds and deploys all contracts to NEAR.

#### Usage

```bash
./deploy.sh <network> <account-id>
```

#### Parameters

- `network`: Target network (`testnet` or `mainnet`)
- `account-id`: Your NEAR account ID

#### Example

```bash
# Deploy to testnet
./deploy.sh testnet myaccount.testnet

# Deploy to mainnet
./deploy.sh mainnet myaccount.near
```

#### What it does

1. **Builds all contracts** with optimized WASM output
2. **Deploys contracts** to NEAR with proper account sub-names
3. **Initializes contracts** with appropriate parameters
4. **Verifies deployment** and displays summary

### `verify.sh`

Verification script that checks deployed contracts and tests basic functionality.

#### Usage

```bash
./verify.sh <network> <account-id>
```

#### Example

```bash
# Verify testnet deployment
./verify.sh testnet myaccount.testnet
```

#### What it checks

1. **Contract Deployment**: Verifies all contracts are deployed
2. **Contract State**: Checks contract initialization and parameters
3. **Contract Balances**: Displays NEAR balances for each contract
4. **Basic Functionality**: Tests pause/unpause functionality
5. **Access Control**: Verifies owner permissions

## Prerequisites

### NEAR CLI

Install NEAR CLI:

```bash
npm install -g near-cli
```

### Authentication

Login to your NEAR account:

```bash
near login
```

### Account Setup

Ensure your account has sufficient NEAR for deployment:

```bash
# Check balance
near state <account-id>

# Ensure you have at least 10 NEAR for deployment
```

## Deployment Process

### 1. Build Contracts

The script automatically builds all contracts:

```bash
# Manual build (if needed)
cd src/cross-chain-swap && cargo build --target wasm32-unknown-unknown --release
cd ../escrow-src && cargo build --target wasm32-unknown-unknown --release
cd ../escrow-dst && cargo build --target wasm32-unknown-unknown --release
cd ../escrow-factory && cargo build --target wasm32-unknown-unknown --release
cd ../limit-order-protocol && cargo build --target wasm32-unknown-unknown --release
```

### 2. Deploy Contracts

Contracts are deployed with sub-account names:

- `base-escrow`: Core escrow functionality
- `escrow-src`: Source chain escrow
- `escrow-dst`: Destination chain escrow
- `escrow-factory`: Factory contract
- `limit-order-protocol`: Limit order protocol

### 3. Initialize Contracts

Each contract is initialized with appropriate parameters:

#### Base Escrow

```json
{
  "rescue_delay": 3600,
  "access_token": "<account-id>.token"
}
```

#### Escrow Source/Destination

```json
{
  "rescue_delay": 3600,
  "access_token": "<account-id>.token"
}
```

#### Escrow Factory

```json
{
  "limit_order_protocol": "<account-id>.limit-order-protocol",
  "fee_token": "<account-id>.token",
  "access_token": "<account-id>.token",
  "rescue_delay_src": 3600,
  "rescue_delay_dst": 3600
}
```

#### Limit Order Protocol

```json
{
  "domain_separator": "0000000000000000000000000000000000000000000000000000000000000000"
}
```

## Verification Process

### 1. Contract Deployment Check

Verifies all contracts are deployed and accessible.

### 2. State Verification

Checks contract initialization and parameters:

- Owner addresses
- Domain separators
- Rescue delays
- Access tokens
- Protocol addresses

### 3. Balance Check

Displays NEAR balances for each contract.

### 4. Functionality Test

Tests basic contract functionality:

- Pause/unpause operations
- Access control
- State management

## Contract Addresses

After deployment, your contracts will be available at:

- **Base Escrow**: `<account-id>.base-escrow`
- **Escrow Source**: `<account-id>.escrow-src`
- **Escrow Destination**: `<account-id>.escrow-dst`
- **Escrow Factory**: `<account-id>.escrow-factory`
- **Limit Order Protocol**: `<account-id>.limit-order-protocol`

## Troubleshooting

### Common Issues

1. **Insufficient Balance**

   ```
   Error: Not enough balance
   ```

   Solution: Ensure your account has at least 10 NEAR

2. **Authentication Error**

   ```
   Error: Not signed in
   ```

   Solution: Run `near login` and authenticate

3. **Network Error**

   ```
   Error: Network not found
   ```

   Solution: Check network parameter (`testnet` or `mainnet`)

4. **Contract Already Deployed**
   ```
   Error: Contract already exists
   ```
   Solution: Use different sub-account names or delete existing contracts

### Manual Verification

If automated verification fails, check manually:

```bash
# Check contract state
near state <account-id>.base-escrow

# View contract methods
near view <account-id>.base-escrow get_owner

# Test contract calls
near call <account-id>.limit-order-protocol pause --accountId <account-id>
```

## Security Considerations

### Access Control

- All contracts have owner-based access control
- Only contract owners can pause/unpause
- Factory contracts have additional access token requirements

### Timelocks

- Rescue delays prevent immediate withdrawals
- Timelocks are configurable per contract
- Default rescue delay is 1 hour (3600 seconds)

### Token Integration

- Contracts support both native NEAR and fungible tokens
- Token transfers require proper authorization
- Access tokens provide additional security layers

## Next Steps

After successful deployment:

1. **Run Integration Tests**

   ```bash
   cd integration-tests
   cargo test
   ```

2. **Verify on NEAR Explorer**

   - Check contract state
   - Verify transactions
   - Monitor contract activity

3. **Deploy Frontend**

   - Configure contract addresses
   - Test user interactions
   - Deploy to production

4. **Set Up Monitoring**
   - Monitor contract events
   - Set up alerts
   - Track usage metrics

## Support

For issues with deployment:

1. Check NEAR CLI version: `near --version`
2. Verify network connectivity: `near status`
3. Check account balance: `near state <account-id>`
4. Review contract logs: `near view <contract-id>`

## License

MIT License
