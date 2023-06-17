#!/bin/bash

# Change the variable to the network URL or name where you want to deploy the canisters.
NETWORK="local"

# Change the variables to the appropriate values for your tokens.
TOKEN1_NAME="Token1"
TOKEN1_SYMBOL="TKN1"
TOKEN1_MINTER_PRINCIPAL=$(dfx identity get-principal)
TOKEN1_ARCHIVE_CONTROLLER=$(dfx identity get-principal)
TOKEN1_INITIAL_BALANCES='vec { record { holder = principal "'$(dfx identity get-principal)'"; balance = 10000; } }'   # 10000 tokens for the deployer

TOKEN2_NAME="Stable"
TOKEN2_SYMBOL="STB"
TOKEN2_MINTER_PRINCIPAL="bw4dl-smaaa-aaaaa-qaacq-cai"  # The principal ID of the minter canister
TOKEN2_ARCHIVE_CONTROLLER=$(dfx identity get-principal)
TOKEN2_INITIAL_BALANCES='vec { record { holder = principal "'$(dfx identity get-principal)'"; balance = 10000; } }'

# Deploy the first token canister.
dfx deploy --network ${NETWORK} ckBTCLedger --argument "(variant { Init = record { token_name = \"${TOKEN1_NAME}\"; token_symbol = \"${TOKEN1_SYMBOL}\"; minting_account = record { owner = principal \"${TOKEN1_MINTER_PRINCIPAL}\"; }; initial_balances = ${TOKEN1_INITIAL_BALANCES}; metadata = vec {}; transfer_fee = 10; archive_options = record { trigger_threshold = 2000; num_blocks_to_archive = 1000; controller_id = principal \"${TOKEN1_ARCHIVE_CONTROLLER}\"; } } })"

# Deploy the Stable token canister.
dfx deploy --network ${NETWORK} StableLedger --argument "(variant { Init = record { token_name = \"${TOKEN2_NAME}\"; token_symbol = \"${TOKEN2_SYMBOL}\"; minting_account = record { owner = principal \"${TOKEN2_MINTER_PRINCIPAL}\"; }; initial_balances = ${TOKEN2_INITIAL_BALANCES}; metadata = vec {}; transfer_fee = 10; archive_options = record { trigger_threshold = 2000; num_blocks_to_archive = 1000; controller_id = principal \"${TOKEN2_ARCHIVE_CONTROLLER}\"; } } })"
