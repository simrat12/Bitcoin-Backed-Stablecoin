use candid::{CandidType, Nat, Deserialize, Principal};
use ic_cdk::{api};
use ic_cdk::export::candid;
use ic_cdk_macros::*;
use std::vec::Vec;
use std::collections::HashMap;
use std::future::Future;
use data_encoding::BASE32_NOPAD;

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
pub struct Transfer {
    amount: Nat,
    to: Account,
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
        ic_cdk::print(format!("caller: {:?}, account: {:?}, amount_to_send: {:?}", caller, account, amount_to_send));
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
            ic_cdk::print(format!("balance: {:?}", balance.0));
            if balance.0 < amount_to_send {
                return Err("Not enough funds available in the Account!".to_string());
            }
            
            let transfer = Transfer {
                amount: amount_to_send,
                to: account.clone(),
            };

            let mint_result: Result<(), _> = api::call::call(Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap(), "icrc1_transfer", (transfer,)).await;

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
        ic_cdk::print(format!("created invoice: {:?}", invoice));
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn test_create_invoice() {
        let caller_bytes = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let caller = Principal::from_slice(&caller_bytes);

        let canister_bytes = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let canister = Principal::from_slice(&canister_bytes);

        let account = Account {
            owner: canister.clone(),
            subaccount: Some(to_subaccount(caller.clone())),
        };
        
        let invoice = create_invoice(account.clone(), Nat::from(100));
        assert_eq!(invoice.to.owner, canister);
        assert_eq!(invoice.to.subaccount, Some(to_subaccount(caller.clone())));
        assert_eq!(invoice.amount, Nat::from(100));
    }

    #[test]
    fn test_get_stable_unpaid_invoice() {
        let caller_bytes = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let caller = Principal::from_slice(&caller_bytes);
    
        let invoice = Invoice {
            to: Account {
                owner: caller.clone(),
                subaccount: Some(vec![0; 32]),
            },
            amount: Nat::from(100),
        };
    
        MINTER.with(|fc| {
            let mut minter = fc.borrow_mut();
            minter.invoices.insert(caller.clone(), invoice.clone());
        });
        
        let result = tokio_test::block_on(get_stable());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Not enough funds available in the Account!");
    }

    // #[tokio::test]
    // async fn test_get_stable_success() {
    //     let caller: ic_cdk::export::Principal = "caller_principal".into();
    //     let to_account: Account = Account {
    //         owner: "to_owner_principal".into(),
    //         subaccount: None,
    //     };
    //     let amount_to_send: u64 = 100;

    //     MINTER.with(|fc| {
    //         let mut minter = fc.borrow_mut();
    //         let invoice = Invoice {
    //             to: to_account.clone(),
    //             amount: amount_to_send,
    //         };
    //         minter.invoices.insert(caller.clone(), invoice);
    //     });

    //     let expected_balance_result = Ok((50,));

    //     ic_cdk::api::call::mock()
    //         .with_args::<(Account,)>(vec![to_account.clone()])
    //         .returns(expected_balance_result);

    //     let mint_result = Ok(());

    //     ic_cdk::api::call::mock()
    //         .with_args::<(Mint,)>(vec![Mint {
    //             amount: amount_to_send,
    //             to: to_account.clone(),
    //             created_at_time: None,
    //             memo: None,
    //         }])
    //         .returns(mint_result);

    //     let result = get_stable().await;

    //     assert_eq!(result, Ok("success!".to_string()));
    // }

    // #[tokio::test]
    // async fn test_get_stable_insufficient_funds() {
    //     let caller: ic_cdk::export::Principal = "caller_principal".into();
    //     let to_account: Account = Account {
    //         owner: "to_owner_principal".into(),
    //         subaccount: None,
    //     };
    //     let amount_to_send: u64 = 100;

    //     MINTER.with(|fc| {
    //         let mut minter = fc.borrow_mut();
    //         let invoice = Invoice {
    //             to: to_account.clone(),
    //             amount: amount_to_send,
    //         };
    //         minter.invoices.insert(caller.clone(), invoice);
    //     });

    //     let expected_balance_result = Ok((50,));

    //     ic_cdk::api::call::mock()
    //         .with_args::<(Account,)>(vec![to_account.clone()])
    //         .returns(expected_balance_result);

    //     let result = get_stable().await;

    //     assert_eq!(
    //         result,
    //         Err("Not enough funds available in the Account!".to_string())
    //     );
    // }
}





