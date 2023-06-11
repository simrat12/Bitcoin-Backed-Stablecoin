use ic_cdk::export::candid::CandidType;
use ic_cdk::query;

#[derive(CandidType)]
pub struct BtcPrice {
    price: u64,
}

#[query]
fn get_btc_price() -> BtcPrice {
    BtcPrice { price: 20000 }
}
