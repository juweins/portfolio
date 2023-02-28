mod config;
mod errors;
mod request;
mod response;
pub mod cli;
pub mod kafka;
pub mod azure;

use anyhow::anyhow;
use config::{ApiDetails, AzureConfig, KafkaConfig};
use errors::KeyError;
use log::{info, warn, error};
use reqwest::Error;
use jsonschema::{Draft, JSONSchema};
use std::{collections::HashMap};

// TODO: Make the symbols dynamic by wrapping in a CLI argument
/// Calls the API via HTTP Request
/// - 
/// - Returns the result formatted as a API specific struct (JSON)
pub async fn request_data(api_name: &str) -> Result<serde_json::Value, anyhow::Error> {

    // With version exchange 0.6 onwards, the api calls are loaded from the config file. This allows the most flexibility.
    // TODO: Command to configure the API calls

    let request_url = match get_api_url(api_name) {
        Ok(url) => url,
        Err(e) => {
            error!("Error: {}", e);
            return Err(anyhow!("Error parsing url for {}: {}",api_name, e))
        }
    };
    let request_key = match get_api_key(api_name) {
        Ok(key) => key,
        Err(e) => {
            error!("Error: {}", e);
            return Err(anyhow!("Error parsing key for {}: {}",api_name, e))
        }
    };


    // Request data from API
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        // The API accepts a simple apikey parameter as authentication method
        .header("apikey", request_key)
        .send()
        .await?
        // If the request is successful, the response is parsed into json
        .json()
        .await?;

    Ok(response)
}

/// Read the API key from a file
// Since there are multiple keys present an api_name parameter is required
// - Custom error type for keys
// TODO: Make this more secure by using a key vault
fn get_api_key(api_name: &str) -> Result<String, KeyError> {

    // expand the path to the config file
    let path = shellexpand::tilde("~/.config/exchange/api_config.json").to_string();
    // read the api_key.json and retrieve the api key by name
    let api_details = serde_json::from_str::<HashMap<String, ApiDetails>>(
        &std::fs::read_to_string(path).unwrap(),
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

/// Read the API url from a file
// Since there are multiple urls present an api_name parameter is required
// - Custom error type for urls
fn get_api_url(api_name: &str) -> Result<String, KeyError> {

    //expand the path to the config file
    let path = shellexpand::tilde("~/.config/exchange/api_config.json").to_string();
    // read the api_key.json and retrieve the api key by name
    let api_details = serde_json::from_str::<HashMap<String, ApiDetails>>(
        &std::fs::read_to_string(path).unwrap(),
    )
    .unwrap();

    // Retrieve ApiDetails matching the api_name
    let api = api_details.get(api_name);

    if let Some(_) = api {
        info!("Found API url for {}", api_name);
        Ok(api.unwrap().url.to_string())
    } else {
        warn!("No API url found for {}", api_name);
        Err(KeyError::NotFound)
    }
}

/// Read the Azure details from a file
// - Returns a AzureConfig struct
fn get_azure_details() -> Result<AzureConfig, Error> {

    // expand the path to the config file
    let path = shellexpand::tilde("~/.config/exchange/azure_config.json").to_string();
    // Read the azure details from a file and store them in a vector
    let azure_details = serde_json::from_str::<AzureConfig>(
        &std::fs::read_to_string(path).unwrap(),
    )
    .unwrap();
    Ok(azure_details)
}

/// Read the Kafka details from a file
// - Returns a KafkaConfig struct
fn get_kafka_details() -> Result<KafkaConfig, Error> {

    // expand the path to the config file
    let path = shellexpand::tilde("~/.config/exchange/kafka_config.json").to_string();

    // Read the kafka details from a file and store them in a vector
    let kafka_details = serde_json::from_str::<KafkaConfig>(
        &std::fs::read_to_string(path).unwrap(),
    )
    .unwrap();
    Ok(kafka_details)
}

// --------------------
// Begin of test section
// --------------------
// Unit tests
// --------------------

#[cfg(test)]
mod tests {
    use super::*;

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

#[test]
fn test_get_api_url() {
    let result = get_api_url("exchangerates_api");
    assert!(result.is_ok());
}

#[test]
fn test_valid_api_url() {
    let result = get_api_url("exchangerates_api");
    assert!(&result.unwrap().is_ascii());
}

#[test]
fn valid_azure_details() {
    let result = get_azure_details();
    assert!(result.is_ok());
}

#[test]
fn valid_kafka_details() {
    let result = get_kafka_details();
    assert!(result.is_ok());
}
}