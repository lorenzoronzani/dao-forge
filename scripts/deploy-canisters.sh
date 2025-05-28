#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
echo "Script directory: $SCRIPT_DIR"

cd $SCRIPT_DIR/..

# Create canisters
dfx canister create dao_association

# Build canisters
dfx build dao_association

# Deploy canisters
dfx deploy internet_identity
dfx deploy dao_sogc_pubblication
dfx deploy documents_storage
dfx deploy voting
dfx deploy dao_discovery
dfx deploy dao_agency
dfx deploy dao_platform