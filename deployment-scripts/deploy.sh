#!/bin/bash

# Deployment script for NEAR cross-chain swap contracts
# This script builds and deploys all contracts to NEAR

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NETWORK=${1:-"testnet"} # testnet or mainnet
ACCOUNT_ID=${2:-""}

if [ -z "$ACCOUNT_ID" ]; then
    echo -e "${RED}Error: Please provide an account ID${NC}"
    echo "Usage: ./deploy.sh <network> <account-id>"
    echo "Example: ./deploy.sh testnet myaccount.testnet"
    exit 1
fi

echo -e "${BLUE}Deploying contracts to $NETWORK as $ACCOUNT_ID${NC}"

# Function to build contract
build_contract() {
    local contract_name=$1
    local contract_path=$2
    
    echo -e "${YELLOW}Building $contract_name...${NC}"
    cd $contract_path
    cargo build --target wasm32-unknown-unknown --release
    cd - > /dev/null
    echo -e "${GREEN}✓ Built $contract_name${NC}"
}

# Function to deploy contract
deploy_contract() {
    local contract_name=$1
    local contract_path=$2
    local wasm_file=$3
    
    echo -e "${YELLOW}Deploying $contract_name...${NC}"
    
    # Deploy the contract
    near deploy $ACCOUNT_ID.$contract_name $contract_path/target/wasm32-unknown-unknown/release/$wasm_file \
        --networkId $NETWORK \
        --accountId $ACCOUNT_ID
    
    echo -e "${GREEN}✓ Deployed $contract_name${NC}"
}

# Function to initialize contract
init_contract() {
    local contract_name=$1
    local init_args=$2
    
    echo -e "${YELLOW}Initializing $contract_name...${NC}"
    
    # Initialize the contract
    near call $ACCOUNT_ID.$contract_name $init_args \
        --networkId $NETWORK \
        --accountId $ACCOUNT_ID \
        --gas 300000000000000
    
    echo -e "${GREEN}✓ Initialized $contract_name${NC}"
}

# Build all contracts
echo -e "${BLUE}Building all contracts...${NC}"

build_contract "cross-chain-swap" "src/cross-chain-swap"
build_contract "escrow-src" "src/escrow-src"
build_contract "escrow-dst" "src/escrow-dst"
build_contract "escrow-factory" "src/escrow-factory"
build_contract "limit-order-protocol" "src/limit-order-protocol"

echo -e "${GREEN}✓ All contracts built successfully${NC}"

# Deploy contracts
echo -e "${BLUE}Deploying contracts...${NC}"

deploy_contract "base-escrow" "src/cross-chain-swap" "cross_chain_swap.wasm"
deploy_contract "escrow-src" "src/escrow-src" "escrow_src.wasm"
deploy_contract "escrow-dst" "src/escrow-dst" "escrow_dst.wasm"
deploy_contract "escrow-factory" "src/escrow-factory" "escrow_factory.wasm"
deploy_contract "limit-order-protocol" "src/limit-order-protocol" "limit_order_protocol.wasm"

echo -e "${GREEN}✓ All contracts deployed successfully${NC}"

# Initialize contracts
echo -e "${BLUE}Initializing contracts...${NC}"

# Initialize base escrow
init_contract "base-escrow" "new '{\"rescue_delay\": 3600, \"access_token\": \"$ACCOUNT_ID.token\"}'"

# Initialize escrow source
init_contract "escrow-src" "new '{\"rescue_delay\": 3600, \"access_token\": \"$ACCOUNT_ID.token\"}'"

# Initialize escrow destination
init_contract "escrow-dst" "new '{\"rescue_delay\": 3600, \"access_token\": \"$ACCOUNT_ID.token\"}'"

# Initialize escrow factory
init_contract "escrow-factory" "new '{
  \"limit_order_protocol\": \"$ACCOUNT_ID.limit-order-protocol\",
  \"fee_token\": \"$ACCOUNT_ID.token\",
  \"access_token\": \"$ACCOUNT_ID.token\",
  \"rescue_delay_src\": 3600,
  \"rescue_delay_dst\": 3600
}'"

# Initialize limit order protocol
init_contract "limit-order-protocol" "new '{\"domain_separator\": \"0000000000000000000000000000000000000000000000000000000000000000\"}'"

echo -e "${GREEN}✓ All contracts initialized successfully${NC}"

# Display deployment summary
echo -e "${BLUE}Deployment Summary:${NC}"
echo -e "${GREEN}✓ Base Escrow: $ACCOUNT_ID.base-escrow${NC}"
echo -e "${GREEN}✓ Escrow Source: $ACCOUNT_ID.escrow-src${NC}"
echo -e "${GREEN}✓ Escrow Destination: $ACCOUNT_ID.escrow-dst${NC}"
echo -e "${GREEN}✓ Escrow Factory: $ACCOUNT_ID.escrow-factory${NC}"
echo -e "${GREEN}✓ Limit Order Protocol: $ACCOUNT_ID.limit-order-protocol${NC}"

echo -e "${BLUE}Next steps:${NC}"
echo "1. Verify contracts on NEAR Explorer"
echo "2. Run integration tests"
echo "3. Configure frontend applications"
echo "4. Set up monitoring and alerts"

echo -e "${GREEN}Deployment completed successfully!${NC}" 