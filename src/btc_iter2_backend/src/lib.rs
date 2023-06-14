use candid::{CandidType, Nat, Deserialize, Principal};
use ic_cdk::{api};
use ic_cdk::export::candid;
use ic_cdk_macros::*;
use std::vec::Vec;
use std::collections::HashMap;
use std::future::Future;

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
fn init() {}

thread_local! {
    static MINTER: std::cell::RefCell<Minter> = std::cell::RefCell::new(Minter {invoices: HashMap::new()});
}

#[update]
async fn get_stable() -> Result<String, String> {
    let caller = api::caller();
    let (account, amount_to_send) = MINTER.with(|fc| {
        let minter = fc.borrow();
        let account = to_account(caller, api::id());
        let amount_to_send = minter.invoices.get(&caller).unwrap().amount.clone();
        (account, amount_to_send)
    });

    let token_future = get_stable_future(account, amount_to_send, caller.clone());
    let result = token_future.await;
    if result.is_ok() {
        MINTER.with(|fc| fc.borrow_mut().invoices.remove(&caller));
    }
    result
}

async fn get_stable_future(account: Account, amount_to_send: Nat, caller: candid::Principal) -> Result<String, String> {
    // Check balance()
    let balance: Result<(Nat,), _> = api::call::call(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(), "icrc1_balance_of", (account.clone(),)).await;

    match balance {
        Ok(balance) => {
            if balance.0 < amount_to_send {
                return Err("Not enough funds available in the Account!".to_string());
            }
            
            let mint = Mint {
                amount: amount_to_send,
                created_at_time: None,
                memo: None,
                to: account.clone(),
            };

            let mint_result: Result<(), _> = api::call::call(Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap(), "icrc1_mint", (mint,)).await;

            match mint_result {
                Ok(_) => Ok("success!".to_string()),
                Err(e) => Err(format!("Couldn't mint stablecoin:\n{:#?}", e)),
            }
        },
        Err(e) => Err(format!("Couldn't get balance:\n{:#?}", e)),
    }
}

#[update]
fn get_invoice() -> Invoice {
    let caller = api::caller();
    MINTER.with(|fc| {
        let mut minter = fc.borrow_mut();
        let account = to_account(caller, api::id());
        let invoice = create_invoice(account, Nat::from(100));
        minter.invoices.insert(caller, invoice.clone());
        invoice
    })
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







