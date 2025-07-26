#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
echo "Script directory: $SCRIPT_DIR"

cd $SCRIPT_DIR/..

set -o allexport
source ./.env
set +o allexport

# Create canisters
dfx canister create dao_association

# Build canisters
dfx build dao_association

# Deploy canisters
dfx deploy internet_identity
dfx deploy dao_sogc_publication
dfx deploy documents_storage
dfx deploy network_call --argument "(record { courier_url = \"$COURIER_URL\"; courier_auth_token = \"$COURIER_AUTH_TOKEN\"; template_id = \"$TEMPLATE_ID\" })"
dfx deploy dao_discovery
dfx deploy voting
dfx deploy dao_agency
dfx deploy dao_platform