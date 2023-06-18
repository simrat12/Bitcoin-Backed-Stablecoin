# Bitcoin-Backed-Stablecoin

This repository contains the code for a Rust project based on the fortune cookie example by Encode ([Fortune Cookie Example](https://dfinityorg.notion.site/ckBTC-example-Encode-Hackathon-0aaf6292e3404dabb49df5d1b5abc797), [YouTube Link](https://www.youtube.com/watch?v=t9DmBFj-3OA)). The goal of this project is to translate the Motoko code into Rust, with additional modifications to implement an HTTPS outcall to retrieve the BTC price and mint new tokens. We also use Encode's workshop on Rust canisters in order to structure the project ([Workshop Link](https://www.youtube.com/watch?v=2IPugAxbfXo&t=2s)).

## Installation

To set up the project locally, follow these steps as detailed in the [Rust Quickstart Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/rust-quickstart):

1. Ensure you have the following prerequisites:
   - An internet connection and access to a shell terminal on your local macOS or Linux computer.
   - Rust programming language and Cargo installed. If not installed, you can follow the Rust installation instructions for your operating system using the following command:
     ```
     curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
     ```
   - IC SDK package downloaded and installed. Refer to the official installation guide for instructions.
   - CMake installed. You can use Homebrew (macOS) or package manager (Linux) to install CMake.

2. Start the local execution environment on your computer by running the following command in the root directory of the project:
    ```
    dfx start --background
    ```


## Running the Project

To run the project locally, follow these steps:

1. Ensure you are in the root directory of the project.

2. Check if `wasm32-unknown-unknown` is installed as a Rust target. If not, run the following command to install it:
 ```
 rustup target add wasm32-unknown-unknown
 ```
 Deploy the `ckBTCLedger` and `StableToken` (set the `TOKEN2_MINTER_PRINCIPAL` to your local `btc_iter2_backend` canister principal ID):
 ```
 chmod +x deploy.sh
 ./deploy.sh 
 ```
 For the next step, go to lines 61 and 76 of `src/btc_iter2_backend` and replace the PrincipalID's with your local ones from the previous command.
 
 Register, build, and deploy the canisters specified in the `dfx.json` file by running the following command:
 ```
 dfx deploy btc_iter2_backend
 ```
 This command will register, build, and deploy the canisters to the local execution environment.

3. Export the canister Id's to your environment:

 ```
 export BTC_BACKEND_CANISTER_ID=... # Your local btc_iter2_backend canister
 export CKBTCLedger_CANISTER_ID=... # Your local ckBTCLedger canister
 ```

4. To run operations, execute the following commands:

 a) Generate a unique invoice:
 ```
 dfx canister call ${BTC_BACKEND_CANISTER_ID} create_invoice '(100, "Mint")'
 ```
 This will return a unique subaccount. Use this value in place of the placeholder subaccounts in the commands below.

 b) Transfer a certain amount from your `ckBTCLedger` to the invoice subaccount:
 ```
 dfx canister --network local call ${CKBTCLedger_CANISTER_ID} icrc1_transfer '(record { "to" = record { "owner" = principal "'${BTC_BACKEND_CANISTER_ID}'"; "subaccount" = opt blob "your subaccount"; }; "amount" = 100 : nat; })'
 ```

 c) Request stable tokens:
 ```
 dfx canister call ${BTC_BACKEND_CANISTER_ID} get_stable '(blob "hash value")'
 ```

 d) To Redeem:

 Generate a unique invoice for redeeming:
 ```
 dfx canister call ${BTC_BACKEND_CANISTER_ID} create_invoice '(100, "Redeem")'
 ```

 Transfer a certain amount from your `ckBTCLedger` to the invoice subaccount:
 ```
 dfx canister --network local call ${CKBTCLedger_CANISTER_ID} icrc1_transfer '(record { "to" = record { "owner" = principal "'${BTC_BACKEND_CANISTER_ID}'"; "subaccount" = opt blob "your subaccount"; }; "amount" = 100 : nat; })'
 ```

 Request to redeem tokens:
 ```
 dfx canister call ${BTC_BACKEND_CANISTER_ID} redeem '(blob "hash value")'
 ```

## To Do

1. Implement the HTTPS outcalls (right now we are just using a mock example)
2. Add in a stable pool staking canister (similar to the model used by Liquity's ethereum backed stablecoin, $LUSD)
3. Add in a governance canister (similar to the model used by Liquity's ethereum backed stablecoin, $LUSD)