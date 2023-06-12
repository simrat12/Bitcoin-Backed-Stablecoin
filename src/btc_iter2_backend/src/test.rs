use candid::{CandidType, Nat, Deserialize};
use ic_cdk::{api};
use ic_cdk::export::candid;
use ic_cdk_macros::*;
use std::vec::Vec;
use std::HashMap;
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
fn init() {
}


thread_local! {
    static MINTER: std::cell::RefCell<Minter> = std::cell::RefCell::new(Minter {});
}

#[update]
async fn get_stable() -> Result<String, String> {
    let cookie_future = MINTER.with(|fc| fc.borrow().get_stable_future());
    cookie_future.await
}

#[update]
fn get_invoice() -> Invoice {
    let caller = api::caller();
    let mut fc = MINTER.with(|fc| fc.borrow_mut().get_invoice(caller));
    fc
}

impl FortuneCookie {
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
            Ok()
        }
    }

    fn get_invoice(&mut self, caller: candid::Principal) -> Invoice {    //Need to call oracle contract to fetch price of BTC here
        let account = to_account(caller, api::id());
        create_invoice(account, Nat::from(100))
        self.invoices.insert(caller, invoice);
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
