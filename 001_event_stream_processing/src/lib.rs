mod config;
mod request;
mod response;

use log::{info, warn};
use std::collections::HashMap;
use std::time::Duration;

use reqwest::Error;

pub mod consumer;
pub mod producer;

use config::{AzureConfig, KafkaConfig, ApiDetails};
use response::Exchange;

use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;

// This function calls the API and returns the result formatted as a struct (Exchange, JSON)
// - The API accepts a start_date and end_date parameter in the format YYYY-MM-DD
// TODO: Make the symbols dynamic by wrapping in a CLI argument
pub async fn request_data(api_name: &str, start_date: &str, end_date: &str) -> Result<Exchange, Error> {
    // TODO: URL builder for dynamic base currency and retrieved symbols
    let request_url = format!("https://api.apilayer.com/exchangerates_data/timeseries?start_date={}&end_date={}&base=EUR&symbols=CHF,GBP,USD", start_date, end_date);
    println!("Requesting from: {}", request_url);

    // Request data from API
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        // REQUEST YOUR OWN (FREE) API KEY FROM https://apilayer.com/marketplace/exchangerates_data-api
        // The API accepts a simple apikey parameter as authentication method
        .header("apikey", get_api_key(&api_name).unwrap())
        .send()
        .await?
        // If the request is successful, the response is parsed into a struct
        .json::<Exchange>()
        .await?;

    Ok(response)
}

// This function pushes the result of the API request to Azure Blob Storage
// - The function uses the azure_sdk_for_rust crate
// - The function uses the azure_key.json file for necessary details
pub async fn push_data() -> azure_core::Result<()> {
    // Temporary: Read a saved response from a file
    // This is to avoid hitting the API limit early on
    let blob_body = std::fs::read("example_response.json").unwrap();

    // Retrieve mandatory details from json file
    let az_details = get_azure_details().unwrap();

    // Assign details to variables
    let account = az_details.storage_account_name;
    let key = az_details.storage_account_key;
    let container = az_details.storage_container;
    let blob_name = az_details.storage_blob_name;

    // Create a blob client
    let storage_credentials = StorageCredentials::Key(account.clone(), key);
    let blob_client =
        ClientBuilder::new(account, storage_credentials).blob_client(&container, blob_name);

    // Create a Blob to store file
    // TODO: As of now there is no way to receive the response status; Contribute?
    let blob = blob_client
        .put_block_blob(blob_body)
        .content_type("application/json")
        .await?;
    println!("Successfully created blob: {:?}", blob.request_id);

    Ok(())
}

// Read the API key from a file
// TODO: Make this more secure by using a key vault
fn get_api_key(api_name: &str) -> Option<String> {
    // read the api_key.json and retrieve the api key by name
    let api_details = serde_json::from_str::<HashMap<String, ApiDetails>>(
        &std::fs::read_to_string("secrets/api_key.json").unwrap(),
    )
    .unwrap();

    // Retrieve ApiDetails matching the api_name
    let api = api_details.get(api_name);

    if let Some(_) = api {
            info!("Found API key for {}", api_name);
            Some(api.unwrap().apikey.to_string())
        } else {
            warn!("No API key found for {}", api_name);
            None
        }

}

// Read the Azure details from a file
fn get_azure_details() -> Result<AzureConfig, Error> {
    // Read the azure details from a file and store them in a vector
    let azure_details = serde_json::from_str::<AzureConfig>(
        &std::fs::read_to_string("secrets/azure_key.json").unwrap(),
    )
    .unwrap();
    Ok(azure_details)
}

// Read the Kafka details from a file
// - returns a KafkaConfig struct
fn get_kafka_details() -> Result<KafkaConfig, Error> {
    // Read the kafka details from a file and store them in a vector
    let kafka_details = serde_json::from_str::<KafkaConfig>(
        &std::fs::read_to_string("secrets/kafka_key.json").unwrap(),
    )
    .unwrap();
    Ok(kafka_details)
}

// --------------------
// Begin of test section
// --------------------

// Test the pull/push functions (request_data and push_data)
// - The functions are tested by calling them and checking if the result is Ok
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_request_data() {
        let result = request_data("exchangerates_api", "2023-01-01", "2023-01-28").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_push_data() {
        let result = push_data().await;
        assert!(result.is_ok());
    }

    /* #[tokio::test]
    async fn test_push_to_kafka() {
        let result = push_to_kafka("test").await;
        assert!(result.is_ok());
    } */
}

// --------------------
// Integration tests
// --------------------
// Unit tests
// --------------------

// Test the get_api_key function
// - The API key is stored in a file and read by the function
// - The returned value should be a string
#[test]
fn test_get_api_key() {
    let result = get_api_key("exchangerates_api");
    assert!(result.is_some());
}

#[test]
fn test_valid_api_key() {
    let result = get_api_key("exchangerates_api");
    assert!(&result.unwrap().is_ascii());
}

#[test]
fn test_invalid_api_key() {
    let result = get_api_key("random_api");
    assert!(result.is_none());
}

// Test the get_azure_details function
// - The Azure details are stored in a file and read by the function
// - The file should be read successfully
#[test]
fn test_get_azure_details() {
    let result = get_azure_details();
    assert!(result.is_ok());
}

// Test the get_kafka_details function
// - The Kafka details are stored in a file and read by the function
// - The file should be read successfully
#[test]
fn test_get_kafka_details() {
    let result = get_kafka_details();
    assert!(result.is_ok());
}
