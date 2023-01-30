mod response;

use response::Exchange;
use reqwest::{Error};

#[tokio::main]
pub async fn request_data(start_date: &str, end_date: &str) -> Result<Exchange, Error> {

// TODO: URL builder for dynamic base currency and retrieved symbols
let request_url = format!("https://api.apilayer.com/exchangerates_data/timeseries?start_date={}&end_date={}&base=EUR&symbols=CHF,GBP,USD", start_date, end_date);
println!("Requesting from: {}", request_url);

// Request data from API
let client = reqwest::Client::new();
let response = client.get(&request_url)
    // REQUEST YOUR OWN (FREE) API KEY FROM https://apilayer.com/marketplace/exchangerates_data-api
    // The API accepts a simple apikey parameter as authentication method
    .header("apikey", read_api_key())
    .send()
    .await?
    // If the request is successful, the response is parsed into a struct
    .json::<Exchange>()
    .await?;

Ok(response)

}

fn read_api_key() -> String {
    String::from_utf8(std::fs::read("api_key.json").unwrap()).unwrap()
}