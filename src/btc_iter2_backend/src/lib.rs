use candid::{CandidType, Nat, Deserialize};
use ic_cdk::{api};
use ic_cdk::export::candid;
use ic_cdk_macros::{init, update, import};
use std::vec::Vec;
use std::future::Future;

// This repo is based on the two links provided in the comments below, the fortune cookie example by Enocde which is based on the ckBTC example by Dfinity. This is
// an attempt to translate the code from Motoko into Rust. There  are a few things to resolve, namely importing ckBTCLedger correctly, then after this compiles the
//next step is to change the 'get cookie future' function so that instead of returning a fortune cookie, it uses a HTTPS outcall to return the BTC price and then mints 
//the new tokens to the caller. The new tokens will be a separate icrc-1 token. Right now, we have one icrc-1 token which is just being used to mock ckBTC.

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

#[derive(Clone, Debug, Deserialize, CandidType)]
struct Transfer {
    amount: Nat,
    from_subaccount: Option<Vec<u8>>,
    created_at_time: Option<Nat>,
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    to: Account,
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct FortuneCookie {                           // Using Fortune cookie example for now https://dfinityorg.notion.site/ckBTC-example-Encode-Hackathon-0aaf6292e3404dabb49df5d1b5abc797, https://www.youtube.com/watch?v=t9DmBFj-3OA
    cookies: Vec<&'static str>,
}

thread_local! {
    static FORTUNE_COOKIE: std::cell::RefCell<FortuneCookie> = std::cell::RefCell::new(FortuneCookie {
        cookies: vec![
            "A journey of a thousand miles begins with a single step.",
            // ... rest of the fortune cookies here
        ],
    });
}

#[init]
fn init() {
}

#[update]
async fn get_cookie() -> Result<String, String> {
    let cookie_future = FORTUNE_COOKIE.with(|fc| fc.borrow().get_cookie_future());
    cookie_future.await
}

#[update]
fn get_invoice() -> Invoice {
    let caller = api::caller();
    let mut fc = FORTUNE_COOKIE.with(|fc| fc.borrow_mut().get_invoice(caller));
    fc
}

impl FortuneCookie {
    fn get_cookie_future(&self) -> impl Future<Output = Result<String, String>> {        //This logic will be changed to return the BTC price and mint the new tokens
        let caller = api::caller();
        let account = to_account(caller, api::id());

        // The following is the async block that returns a future
        async move {
            // Check balance
            let balance = ckBTCLedger::icrc1_balance_of((account.clone(),)).await?;

            if balance < 100 {
                return Err("Not enough funds available in the Account. Make sure you send at least 100 ckSats.".to_string());
            }

            // Transfer funds
            let transfer = Transfer {
                amount: balance - 10,
                from_subaccount: Some(to_subaccount(caller)),
                created_at_time: None,
                fee: Some(Nat::from(10)),
                memo: None,
                to: account.clone(),
            };

            let transfer_result = ckBTCLedger::icrc1_transfer((transfer,)).await?;
            if transfer_result.is_err() {
                return Err(format!("Couldn't transfer funds to default account:\n{}", transfer_result.unwrap_err()));
            }

            // Return cookie
            let cookie_index = api::time() as usize % self.cookies.len();
            Ok(format!("🥠: {}", self.cookies[cookie_index]))
        }
    }

    fn get_invoice(&mut self, caller: candid::Principal) -> Invoice {
        let account = to_account(caller, api::id());
        create_invoice(account, Nat::from(100))
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


