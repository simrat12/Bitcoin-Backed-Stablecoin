use candid::Nat;
use ic_base_types::Principal;
use serde::{Deserialize, Serialize};

pub type Subaccount = Vec<u8>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invoice {
    pub to: Account,
    pub amount: Nat,
}
