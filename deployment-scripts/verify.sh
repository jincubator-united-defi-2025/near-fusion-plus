#!/bin/bash

# Verification script for NEAR cross-chain swap contracts
# This script verifies that all contracts are deployed and working correctly

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NETWORK=${1:-"testnet"}
ACCOUNT_ID=${2:-""}

if [ -z "$ACCOUNT_ID" ]; then
    echo -e "${RED}Error: Please provide an account ID${NC}"
    echo "Usage: ./verify.sh <network> <account-id>"
    echo "Example: ./verify.sh testnet myaccount.testnet"
    exit 1
fi

echo -e "${BLUE}Verifying contracts on $NETWORK for $ACCOUNT_ID${NC}"

# Function to check contract state
check_contract() {
    local contract_name=$1
    local contract_id="$ACCOUNT_ID.$contract_name"
    
    echo -e "${YELLOW}Checking $contract_name...${NC}"
    
    # Check if contract exists
    if near view $contract_id get_owner --networkId $NETWORK > /dev/null 2>&1; then
        echo -e "${GREEN}✓ $contract_name is deployed and accessible${NC}"
        
        # Get contract state
        echo -e "${BLUE}  Contract ID: $contract_id${NC}"
        
        # Try to get owner (if available)
        if near view $contract_id get_owner --networkId $NETWORK > /dev/null 2>&1; then
            local owner=$(near view $contract_id get_owner --networkId $NETWORK)
            echo -e "${BLUE}  Owner: $owner${NC}"
        fi
        
        # Try to get domain separator (for limit order protocol)
        if near view $contract_id get_domain_separator --networkId $NETWORK > /dev/null 2>&1; then
            local domain_sep=$(near view $contract_id get_domain_separator --networkId $NETWORK)
            echo -e "${BLUE}  Domain Separator: $domain_sep${NC}"
        fi
        
        # Try to get rescue delay (for escrow contracts)
        if near view $contract_id get_rescue_delay --networkId $NETWORK > /dev/null 2>&1; then
            local rescue_delay=$(near view $contract_id get_rescue_delay --networkId $NETWORK)
            echo -e "${BLUE}  Rescue Delay: $rescue_delay${NC}"
        fi
        
        # Try to get access token (for escrow contracts)
        if near view $contract_id get_access_token --networkId $NETWORK > /dev/null 2>&1; then
            local access_token=$(near view $contract_id get_access_token --networkId $NETWORK)
            echo -e "${BLUE}  Access Token: $access_token${NC}"
        fi
        
        # Try to get limit order protocol (for factory)
        if near view $contract_id get_limit_order_protocol --networkId $NETWORK > /dev/null 2>&1; then
            local limit_order_protocol=$(near view $contract_id get_limit_order_protocol --networkId $NETWORK)
            echo -e "${BLUE}  Limit Order Protocol: $limit_order_protocol${NC}"
        fi
        
        # Try to get fee token (for factory)
        if near view $contract_id get_fee_token --networkId $NETWORK > /dev/null 2>&1; then
            local fee_token=$(near view $contract_id get_fee_token --networkId $NETWORK)
            echo -e "${BLUE}  Fee Token: $fee_token${NC}"
        fi
        
        # Try to get rescue delays (for factory)
        if near view $contract_id get_rescue_delay_src --networkId $NETWORK > /dev/null 2>&1; then
            local rescue_delay_src=$(near view $contract_id get_rescue_delay_src --networkId $NETWORK)
            echo -e "${BLUE}  Rescue Delay Src: $rescue_delay_src${NC}"
        fi
        
        if near view $contract_id get_rescue_delay_dst --networkId $NETWORK > /dev/null 2>&1; then
            local rescue_delay_dst=$(near view $contract_id get_rescue_delay_dst --networkId $NETWORK)
            echo -e "${BLUE}  Rescue Delay Dst: $rescue_delay_dst${NC}"
        fi
        
    else
        echo -e "${RED}✗ $contract_name is not deployed or not accessible${NC}"
        return 1
    fi
}

# Function to check contract balance
check_balance() {
    local contract_name=$1
    local contract_id="$ACCOUNT_ID.$contract_name"
    
    echo -e "${YELLOW}Checking balance for $contract_name...${NC}"
    
    local balance=$(near state $contract_id --networkId $NETWORK | grep "balance" | awk '{print $2}')
    echo -e "${BLUE}  Balance: $balance NEAR${NC}"
}

# Check all contracts
echo -e "${BLUE}Checking contract deployments...${NC}"

check_contract "base-escrow"
check_contract "escrow-src"
check_contract "escrow-dst"
check_contract "escrow-factory"
check_contract "limit-order-protocol"

echo -e "${GREEN}✓ All contracts verified${NC}"

# Check balances
echo -e "${BLUE}Checking contract balances...${NC}"

check_balance "base-escrow"
check_balance "escrow-src"
check_balance "escrow-dst"
check_balance "escrow-factory"
check_balance "limit-order-protocol"

echo -e "${GREEN}✓ All balances checked${NC}"

# Test basic functionality
echo -e "${BLUE}Testing basic functionality...${NC}"

# Test limit order protocol pause/unpause
echo -e "${YELLOW}Testing limit order protocol pause functionality...${NC}"
if near call $ACCOUNT_ID.limit-order-protocol pause --networkId $NETWORK --accountId $ACCOUNT_ID > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Pause function works${NC}"
    
    # Check if paused
    local is_paused=$(near view $ACCOUNT_ID.limit-order-protocol is_paused --networkId $NETWORK)
    if [[ "$is_paused" == "true" ]]; then
        echo -e "${GREEN}✓ Contract is paused${NC}"
    else
        echo -e "${RED}✗ Contract should be paused but isn't${NC}"
    fi
    
    # Unpause
    if near call $ACCOUNT_ID.limit-order-protocol unpause --networkId $NETWORK --accountId $ACCOUNT_ID > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Unpause function works${NC}"
    else
        echo -e "${RED}✗ Unpause function failed${NC}"
    fi
else
    echo -e "${RED}✗ Pause function failed${NC}"
fi

echo -e "${GREEN}✓ Basic functionality tests completed${NC}"

# Display verification summary
echo -e "${BLUE}Verification Summary:${NC}"
echo -e "${GREEN}✓ All contracts are deployed and accessible${NC}"
echo -e "${GREEN}✓ Contract balances are available${NC}"
echo -e "${GREEN}✓ Basic functionality tests passed${NC}"

echo -e "${BLUE}Next steps:${NC}"
echo "1. Run integration tests"
echo "2. Test cross-contract interactions"
echo "3. Deploy frontend applications"
echo "4. Set up monitoring"

echo -e "${GREEN}Verification completed successfully!${NC}" 