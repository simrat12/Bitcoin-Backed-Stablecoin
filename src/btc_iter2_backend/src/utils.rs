use candid::{Nat, Principal};
use ic_base_types::PrincipalId;
use serde::{Deserialize, Serialize};

pub type Subaccount = Vec<u8>;

pub fn to_subaccount(p: Principal) -> Subaccount {
    let bytes = ic_base_types::Principal::to_blob(p);
    let size = bytes.len();

    assert!(size <= 29);

    let a = (0..32).map(|i| {
        if i + size < 31 {
            0
        } else if i + size == 31 {
            size as u8
        } else {
            bytes[i + size - 32]
        }
    }).collect();

    a
}

pub fn to_account(caller: Principal, canister: Principal) -> Account {
    Account {
        owner: canister,
        subaccount: Some(to_subaccount(caller)),
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

pub fn create_invoice(to: Account, amount: Nat) -> Invoice {
    Invoice { to, amount }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invoice {
    pub to: Account,
    pub amount: Nat,
}
