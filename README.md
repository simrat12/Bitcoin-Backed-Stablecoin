# Bitcoin-Backed-Stablecoin

This repository contains the code for a Rust project based on the fortune cookie example by Encode (https://dfinityorg.notion.site/ckBTC-example-Encode-Hackathon-0aaf6292e3404dabb49df5d1b5abc797, https://www.youtube.com/watch?v=t9DmBFj-3OA). The goal of this project is to translate the Motoko code into Rust, with additional modifications to implement a HTTPS outcall to retrieve the BTC price and mint new tokens. We also use Encode's workshop on Rust canisters in order to structure the
project (https://www.youtube.com/watch?v=2IPugAxbfXo&t=2s).

## Installation

To set up the project locally, follow these steps (https://internetcomputer.org/docs/current/developer-docs/backend/rust/rust-quickstart):

1. Ensure you have the following prerequisites:
   - An internet connection and access to a shell terminal on your local macOS or Linux computer.
   - Rust programming language and Cargo installed. If not installed, you can follow the Rust installation instructions for your operating system using the following command:
     ```bash
     curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
     ```
   - IC SDK package downloaded and installed. Refer to the official installation guide for instructions.
   - CMake installed. You can use Homebrew (macOS) or package manager (Linux) to install CMake.

2. Start the local execution environment on your computer by running the following command in the root directory of the project:
   ```bash
   dfx start --background
   ```

## Running the Project

To run the project locally, follow these steps:

1. Make sure you are in the root directory of the project.

2. Check if wasm32-unknown-unknown is installed as a Rust target. If not, run the following command to install it:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
    Deploy the ckBTCLedger and StableToken (set the TOKEN2_MINTER_PRINCIPAL to your local btc_iter2_backend canister principal ID)
    ```bash
    chmod +x deploy.sh
    ```
    ```bash
    ./deploy.sh 
    ```
    (For the next step, go to lines 61 and 76 of src/btc_iter2_backend and replace the PrincipalID's with your local ones from the previous command)
  - Register, build, and deploy the canisters specified in the dfx.json file by running the following command:  
    ```bash
    dfx deploy btc_iter2_backend
    ```
  - This command will register, build, and deploy the canisters to the local execution environment.

3. Export the canister Id's to your environement:

    ```bash
    export BTC_BACKEND_CANISTER_ID=... # Your local btc_iter2_backend canister
    ```
    ```bash
    export CKBTCLedger_CANISTER_ID=... # Your local ckBTCLedger canister
    ```


4. 
  ```bash
  dfx canister call ${BTC_BACKEND_CANISTER_ID} get_invoice
  ```
  - This will return a unique subaccount -> use this value in place of the placeholder subaccounts in the commands below.

  ```bash
  dfx canister call ${CKBTCLedger_CANISTER_ID} icrc1_transfer '(record { from_subaccount = null; to = record { owner = principal "'${BTC_BACKEND_CANISTER_ID}'"; subaccount = opt blob "\c5\801\84\e1\8b\d9k\e7~\03\ff\94\c7\a2\d9\ea\f3\12n\15\b6Pp\a6\12t\13\02\00\00\00"; }; amount = 110 : nat; })'
  ```
  ```bash
  dfx canister call ${CKBTCLedger_CANISTER_ID} icrc1_balance_of '(record { owner = principal "'${BTC_BACKEND_CANISTER_ID}'"; subaccount = opt blob "\c5\801\84\e1\8b\d9k\e7~\03\ff\94\c7\a2\d9\ea\f3\12n\15\b6Pp\a6\12t\13\02\00\00\00" })'
  ```
  ```bash
  dfx canister call ${BTC_BACKEND_CANISTER_ID} get_stable
  ```

## To Do

1. Implement the HTTPS outcalls (right now we are just using a mock example)
2. Add functionality to handle redemptions
3. Add in a stable pool staking canister (similar to the model used by Liquity's ethereum backed stablecoin, $LUSD)
