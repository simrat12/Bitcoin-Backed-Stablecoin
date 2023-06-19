use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,};
use ic_cdk::{export::candid::Nat};

// Update method using the HTTPS outcalls feature
#[ic_cdk::update]
async fn get_btc_usd_exchange() -> Result<Nat, String> {
    let now = ic_cdk::api::time() / 1_000_000_000; // now holds Unix timestamp
    let seconds_of_time: u64 = 60; // the granularity
    let one_hour_in_seconds: u64 = 3600;
    let start = (now - one_hour_in_seconds).to_string(); // convert Unix timestamp 1 hour ago to string
    let end = (now - one_hour_in_seconds + seconds_of_time).to_string(); // end time is start time + granularity
    let host = "api.pro.coinbase.com";

    let url = format!(
        "https://{}/products/BTC-USD/candles?start={}&end={}&granularity={}",
        host,
        start,
        end,
        seconds_of_time.to_string()
    );

    ic_cdk::println!("URL: {}", url);

    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}:443"),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "exchange_rate_canister".to_string(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,               //optional for request
        max_response_bytes: None, //optional for request
        transform: None,          //optional for request
        headers: request_headers,
    };

    match http_request(request).await {
        Ok((response,)) => {
            let str_body = String::from_utf8(response.body)
                .map_err(|_| "Transformed response is not UTF-8 encoded.")?;

            ic_cdk::println!("Response body: {}", str_body);

            let data: Vec<Vec<f64>> = serde_json::from_str(&str_body)
                .map_err(|_| "Failed to parse the response into the expected format.")?;
            
            if let Some(record) = data.get(0) {
                if let Some(closing_price) = record.get(4) {
                    return Ok(Nat::from(*closing_price as u64));
                }
            }

            Err("Failed to retrieve the closing price from the response.".to_string())
        }
        Err((r, m)) => {
            let message = format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            Err(message)
        }
    }
}
