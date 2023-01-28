// WSL2/Ubuntu users: Make sure that you have pkg-config and libssl-dev installed! 

use serde::Deserialize;
use reqwest::{Error};

#[derive(Deserialize, Debug)]
struct Exchange {
    base: String,
    end_date: String,
    rates: ExchangeRates,
}
#[derive(Deserialize, Debug)]
struct ExchangeRates {
        GBP: f64,
        USD: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error>{

    let start_date = "2023-01-01";
    let end_date = "2023-01-02";
    let api_key = read_api_key();

    let request_url = format!("https://api.apilayer.com/exchangerates_data/timeseries?start_date={}&end_date={}&base=USD&symbols=EUR,GBP", start_date, end_date);
    println!("Requesting from: {}", request_url);
 
    let client = reqwest::Client::new();
    let response = client.get(&request_url)
        // REQUEST YOUR OWN API KEY FROM https://apilayer.com/marketplace/exchangerates_data-api
        .header("apikey", api_key)
        .send()
        .await?
        .text()
        .await?;

    println!("Response: {:?}", response); 
    Ok(())
    

}

fn read_api_key() -> String {
    String::from_utf8(std::fs::read("api_key.json").unwrap()).unwrap()
}