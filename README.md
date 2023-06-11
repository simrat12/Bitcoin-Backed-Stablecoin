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
  - Register, build, and deploy the canisters specified in the dfx.json file by running the following command:  
    ```bash
    dfx deploy
    ```
  - This command will register, build, and deploy the canisters to the local execution environment.

## To Do

1. Ensure that the (mock) ckBTCcannister is imported correctly into the btc_iter2_backend lib.rs
2. Test the functionality and ensure there are no issues
3. Implement the HTTPS outcalls (right now we are just using a mock example)
3. Copy and paste current icrc1-ledger.did/wasm files into a new directory and use this as the stablecoin 
4. Change 'fortune cookie' implementation to call the https cannister and retrieve BTC price and mint stablecoin to the user
5. Add functionality to handle redemptions
