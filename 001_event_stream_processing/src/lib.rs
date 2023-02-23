mod config;
mod errors;
mod request;
mod response;

use log::{info, warn, error};
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

use reqwest::Error;
use errors::KeyError;
use anyhow::anyhow;

pub mod cli;
pub mod client;
pub mod consumer;
pub mod producer;

use config::{ApiDetails, AzureConfig, KafkaConfig};
use response::{CatNinja,Exchange};

use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;

// TODO: Make the symbols dynamic by wrapping in a CLI argument
/// Calls the API via HTTP Request
/// - 
/// - Returns the result formatted as a API specific struct (JSON)
pub async fn request_data(api_name: &str, start_date: &str, end_date: &str) -> Result<serde_json::Value, anyhow::Error> {

    // TODO: URL builder for dynamic base currency and retrieved symbols
    let request_url: String = match api_name{
        "exchangerates_api" => {
            let request_url = format!("https://api.apilayer.com/exchangerates_data/timeseries?start_date={start}&end_date={end}&base=EUR&symbols=CHF,GBP,USD",
                                    start=start_date, end=end_date);
            request_url
        }
        "test_api" => {
            let request_url = format!("https://catfact.ninja/fact");
            request_url
        }
        _ => {
            error!("API {} not supported: Missing configuration.", api_name);
            return Err(anyhow!("API {} not supported: Missing configuration.", api_name))
        }
    };

    println!("Requesting from: {}", request_url);

    let result = get_api_key(api_name);

    // If Error occured and terminate function early
    match &result {
        Ok(_) => {}
        Err(e) => {
            warn!("Error: {}", e);
            return Err(anyhow!("Error parsing key for {}: {}",api_name, e))
        }
    }

    // Request data from API
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        // The API accepts a simple apikey parameter as authentication method
        .header("apikey", result.unwrap())
        .send()
        .await?
        // If the request is successful, the response is parsed into json
        .json()
        .await?;

    Ok(response)
}

/// Pushes a file to Azure Blob Storage
/// - Establishes a connection to Azure Blob Storage via azure_key.json
/// - Reads the file to be pushed from the file system
/// - Pushes the file to Azure Blob Storage
/// - Returns a Result with the status of the operation
pub async fn push_to_azure() -> azure_core::Result<()> {
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
// Since there are multiple keys present an api_name parameter is required
// - Custom error type for keys
// TODO: Make this more secure by using a key vault
fn get_api_key(api_name: &str) -> Result<String, KeyError> {
    // read the api_key.json and retrieve the api key by name
    let api_details = serde_json::from_str::<HashMap<String, ApiDetails>>(
        &std::fs::read_to_string("secrets/api_key.json").unwrap(),
    )
    .unwrap();

    // Retrieve ApiDetails matching the api_name
    let api = api_details.get(api_name);

    if let Some(_) = api {
        info!("Found API key for {}", api_name);
        Ok(api.unwrap().apikey.to_string())
    } else {
        warn!("No API key found for {}", api_name);
        Err(KeyError::NotFound)
    }
}

// Read the Azure details from a file
// - returns a AzureConfig struct
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
// Unit tests
// --------------------

// Test the get_api_key function
// - The API key is stored in a file and read by the function
// - The returned value should be a string
#[test]
fn test_get_api_key() {
    let result = get_api_key("exchangerates_api");
    assert!(result.is_ok());
}

#[test]
fn test_valid_api_key() {
    let result = get_api_key("exchangerates_api");
    assert!(&result.unwrap().is_ascii());
}

#[test]
fn test_invalid_api_key() {
    let result = get_api_key("random_api");
    assert!(result.is_err());
}

// Test the get_azure_details function
// - The Azure details are stored in a file and read by the function
// - The file should be read successfully
#[test]
fn valid_azure_details() {
    let result = get_azure_details();
    assert!(result.is_ok());
}

// Test the get_kafka_details function
// - The Kafka details are stored in a file and read by the function
// - The file should be read successfully
#[test]
fn valid_kafka_details() {
    let result = get_kafka_details();
    assert!(result.is_ok());
}
