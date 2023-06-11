use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::{api, storage};
use std::vec::Vec;
use candid::{Nat};
use serde::{Deserialize, Serialize};
use crate::types::{Account, Invoice, Subaccount};
use candid::Principal;
use ic_cdk::update;

#[import(canister = "ckbtc_ledger")]
struct CkBtcLedger;

#[derive(CandidType, Deserialize)]
pub struct Transfer {
    amount: Nat,
    from_subaccount: Option<Subaccount>,
    created_at_time: Option<Nat>,
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    to: Account,
}

#[init]
fn init() {
    let fortune_cookie = storage::get_mut::<FortuneCookie>();
    fortune_cookie.cookies = vec![
        "A journey of a thousand miles begins with a single step.",
        // ... rest of the fortune cookies here
    ];
}

struct FortuneCookie {
    cookies: Vec<&'static str>,
}

#[update]
async fn get_cookie() -> Result<String, String> {
    let fortune_cookie = storage::get::<FortuneCookie>();
    fortune_cookie.get_cookie().await
}

#[update]
async fn get_invoice() -> Invoice {
    let fortune_cookie = storage::get::<FortuneCookie>();
    fortune_cookie.get_invoice(api::caller()).await
}

impl FortuneCookie {
    async fn get_cookie(&self) -> Result<String, String> {
        let caller = api::caller();
        let account = to_account(caller, api::id());

        // Check balance
        let balance = CkBtcLedger::icrc1_balance_of((account.clone(),)).await?;

        if balance < 100 {
            return Err("Not enough funds available in the Account. Make sure you send at least 100 ckSats.".to_string());
        }

        // Transfer funds
        let transfer = Transfer {
            amount: balance - 10,
            from_subaccount: Some(to_subaccount(caller)),
            created_at_time: None,
            fee: Some(10),
            memo: None,
            to: account.clone(),
        };

        let transfer_result = CkBtcLedger::icrc1_transfer((transfer,)).await?;
        match transfer_result {
            Err(err) => return Err(format!("Couldn't transfer funds to default account:\n{}", err)),
            _ => (),
        }

        // Return cookie
        let cookie_index = api::time().as_nanos() as usize % self.cookies.len();
        Ok(format!("🥠: {}", self.cookies[cookie_index]))
    }

    async fn get_invoice(&self, caller: Principal) -> Invoice {
        let account = to_account(caller, api::id());
        create_invoice(account, Nat::from(100))
    }
}

fn to_account(caller: Principal, canister: Principal) -> Account {
    Account {
        owner: canister,
        subaccount: Some(to_subaccount(caller)),
    }
}

fn to_subaccount(p: Principal) -> Subaccount {
    let bytes = Principal::to_blob(p);
    let size = bytes.len();

    assert!(size <= 29);

    (0..32).map(|i| {
        if i + size < 31 {
            0
        } else if i + size == 31 {
            size as u8
        } else {
            bytes[i + size - 32]
        }
    }).collect::<Vec<u8>>()
}

fn create_invoice(to: Account, amount: Nat) -> Invoice {
    Invoice { to, amount }
}

