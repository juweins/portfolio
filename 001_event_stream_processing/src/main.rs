// WSL2/Ubuntu users: Make sure that you have pkg-config and libssl-dev installed! 

use serde::{Serialize, Deserialize};
use reqwest::{Error};

#[derive(Serialize, Deserialize, Debug)]
struct Exchange {
    base: String,
    end_date: String,
    rates: Precision,
}
// Since the API returns a nested JSON object with the date as the key, we need to flatten the object
// to be able to deserialize it into a HashMap (and make the struct dynamic)
#[derive(Serialize, Deserialize, Debug)]
struct Precision {
    #[serde(flatten)]
    rates: std::collections::HashMap<String, ExchangeRates>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExchangeRates {
        CHF: f64,
        GBP: f64,
        USD: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error>{

    let start_date = "2023-01-01";
    let end_date = "2023-01-28";
    let api_key = read_api_key();

    let request_url = format!("https://api.apilayer.com/exchangerates_data/timeseries?start_date={}&end_date={}&base=EUR&symbols=CHF,GBP,USD", start_date, end_date);
    println!("Requesting from: {}", request_url);
 
    let client = reqwest::Client::new();
    let response = client.get(&request_url)
        // REQUEST YOUR OWN API KEY FROM https://apilayer.com/marketplace/exchangerates_data-api
        .header("apikey", api_key.as_str())
        .send()
        .await?
        .json::<Exchange>()
        .await?;

    //let response = std::fs::read_to_string("example_response.json").unwrap();
    //let parsed_json = serde_json::from_str::<Exchange>(&response).unwrap();
    println!("Response: {:#?}", response); 
    Ok(())
    

}

fn read_api_key() -> String {
    String::from_utf8(std::fs::read("api_key.json").unwrap()).unwrap()
}