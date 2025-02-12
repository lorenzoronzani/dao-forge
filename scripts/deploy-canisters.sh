#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
echo "Script directory: $SCRIPT_DIR"

cd $SCRIPT_DIR/..

# Create canisters
dfx canister create --all

# Build canisters
dfx build

# Deploy canisters
dfx deploy dao-manager
dfx deploy frontend-platform
dfx deploy internet_identity
dfx deploy dao-discovery-registry