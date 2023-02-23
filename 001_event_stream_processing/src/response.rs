use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Exchange {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CatNinja {
    fact: String,
    length: i32,
}
