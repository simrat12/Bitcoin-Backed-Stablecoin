use candid::{CandidType, Nat, Deserialize};
use ic_cdk::{api};
use ic_cdk::export::candid;
use ic_cdk_macros::*;
use std::vec::Vec;
use std::collections::HashMap;
use std::future::Future;

// The idea here, is that a user calls the invoice function (which in turn fetches the BTC price from oracle's HTTPS outcall). 
// Then, they transfer the given ckBTC to their respective address. Then they will call 'get_stable()' which checks if the
//funds have been transferred. If so, the respective amount of stablecoin is minted to the caller's address.

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct Account {
    pub owner: candid::Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct Invoice {
    pub to: Account,
    pub amount: Nat,
}

#[import(canister = "ckBTCLedger")]
#[cfg(target_arch = "wasm32")]
struct ckBTCLedger;

#[import(canister_id = "aaaaa-aa", candid_path = "icrc1-ledger.did")]             // Import syntax based on 31 min of https://www.youtube.com/watch?v=2IPugAxbfXo&t=2s 
#[cfg(not(target_arch = "wasm32"))]
struct ckBTCLedger;

#[import(canister = "StableLedger")]
#[cfg(target_arch = "wasm32")]
struct StableLedgerr;

#[import(canister_id = "aaaaa-aa", candid_path = "StableToken/icrc1-ledger.did")]             
#[cfg(not(target_arch = "wasm32"))]
struct StableLedger;

#[import(canister = "mock_https_outcalls")]
#[cfg(target_arch = "wasm32")]
struct mock_https_outcalls;

#[import(canister_id = "aaaaa-aa", candid_path = "mock_https_outcalls/mock_https_outcalls.did")]             
#[cfg(not(target_arch = "wasm32"))]
struct mock_https_outcalls;

#[derive(Clone, Debug, Deserialize, CandidType)]
struct Mint {
    amount: Nat,
    to: Account,
    created_at_time: Option<Nat>,
    memo: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct Minter {
    invoices: HashMap<candid::Principal, Invoice>,
}

#[init]
fn init() {
}


thread_local! {
    static MINTER: std::cell::RefCell<Minter> = std::cell::RefCell::new(Minter {invoices: HashMap::new()});
}

#[update]
async fn get_stable() -> Result<String, String> {
    let token_future = MINTER.with(|fc| fc.borrow().get_stable_future());
    token_future.await
}

#[update]
fn get_invoice() -> Invoice {
    let caller = api::caller();
    let mut fc = MINTER.with(|fc| fc.borrow_mut().get_invoice(caller));
    fc
}

impl Minter {
    fn get_stable_future(&self) -> impl Future<Output = Result<String, String>> {        //This checks the balance of the caller and mints the new tokens to the caller if they have fulfilled their invoice
        let caller = api::caller();
        let account = to_account(caller, api::id());

        // The following is the async block that returns a future
        async move {
            // Check balance
            let balance = ckBTCLedger::icrc1_balance_of((account.clone(),)).await?;

            let amount_to_send = self.invoices.get(&caller).unwrap().amount;

            if balance < amount_to_send {
                return Err("Not enough funds available in the Account!".to_string());
            }

            // Mint stablecoin
            let mint = Mint {
                amount: balance - amount_to_send,
                created_at_time: None,
                memo: None,
                to: account.clone(),
            };

            let mint_result = StableLedger::icrc1_mint((mint,)).await?;
            if mint_result.is_err() {
                return Err(format!("Couldn't transfer funds to default account:\n{}", mint_result.unwrap_err()));
            }

            // Remove invoive
            self.invoices.remove(&caller);
            Ok("success!".to_string())
        }
    }

    fn get_invoice(&mut self, caller: candid::Principal) -> Invoice {    //Need to call oracle contract to fetch price of BTC here
        let account = to_account(caller, api::id());
        let invoice = create_invoice(account, Nat::from(100)); // 100 is just a placeholder for now
        self.invoices.insert(caller, invoice);
        invoice
    }
}

fn to_account(caller: candid::Principal, canister: candid::Principal) -> Account {
    Account {
        owner: canister,
        subaccount: Some(to_subaccount(caller)),
    }
}

fn to_subaccount(p: candid::Principal) -> Vec<u8> {
    let bytes = p.as_slice();
    let size = bytes.len();

    assert!(size <= 29);

    (0..32).map(|i| if i < size { bytes[i] } else { 0 }).collect()
}

fn create_invoice(to: Account, amount: Nat) -> Invoice {
    Invoice {
        to,
        amount,
    }
}



