mod config;
mod errors;
mod request;
mod response;

use log::{info, warn, error};
use serde_json::json;
use std::{collections::HashMap, io::Write};
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
pub async fn push_to_azure(container_name: &str, filename: &str, blob_name: &str) -> azure_core::Result<()> {
    // Temporary: Read a saved response from a file
    // This is to avoid hitting the API limit early on
    let blob_body = std::fs::read(filename).unwrap();

    let blob_client = get_az_client().blob_client(container_name, blob_name);
    // Check if container exists
    // - If not, create it
    let container_exists = blob_client.container_client().exists().await?;
    if !container_exists {
        info!("Container {} does not exist. Creating...", container_name);

        blob_client
        .container_client()
        .create()
        .public_access(PublicAccess::None)
        .await?;

        info!("Successfully created container: {}", container_name);
    }



    // Create a Blob to store file
    // TODO: As of now there is no way to receive the response status; Contribute?
    let blob = blob_client
        .put_block_blob(blob_body)
        .content_type("application/json")
        .await?;
    println!("Successfully created blob: {:?}", blob.request_id);

    Ok(())
}

/// Pulls a file from Azure Blob Storage
/// - Establishes a connection to Azure Blob Storage via azure_key.json
/// - Pulls the file from Azure Blob Storage
/// - Returns a Result with the status of the operation
/// - Returns the file as a vector of bytes
/// - Returns an error if the file is empty
pub async fn pull_from_azure(container_name: &str, blob_name: &str) -> azure_core::Result<Vec<u8>> {

    let blob_client = get_az_client().blob_client(container_name, blob_name);

    // Get the blob
    let blob = blob_client.get_content().await;
    
    // Unwrap the result
    let blob = match blob {
        Ok(content) => {

            // If the blob is empty, return an error
            if content.is_empty(){
                let message = "Error retrieving blob data: Blob is empty!";
                warn!("{}", message);
                Err(azure_core::Error::message(azure_storage::ErrorKind::Other, message))
            } else {
                info!("Successfully retrieved blob: {:?}", blob_name);

                // Create new file to store the retrieved blob content
                let mut file = std::fs::File::create(blob_name).unwrap();
                file.write(&content).unwrap();

                Ok(content)
            } 
        }
        Err(e) => {
            error!("Error retrieving blob data: {}", e);
            Err(e)
        }
    };
    
    blob
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
// - Returns a AzureConfig struct
fn get_azure_details() -> Result<AzureConfig, Error> {
    // Read the azure details from a file and store them in a vector
    let azure_details = serde_json::from_str::<AzureConfig>(
        &std::fs::read_to_string("secrets/azure_key.json").unwrap(),
    )
    .unwrap();
    Ok(azure_details)
}

// Read the Kafka details from a file
// - Returns a KafkaConfig struct
fn get_kafka_details() -> Result<KafkaConfig, Error> {
    // Read the kafka details from a file and store them in a vector
    let kafka_details = serde_json::from_str::<KafkaConfig>(
        &std::fs::read_to_string("secrets/kafka_key.json").unwrap(),
    )
    .unwrap();
    Ok(kafka_details)
}

/// Creates a container in Azure Blob Storage
/// - Establishes a connection to Azure Blob Storage via azure_key.json
/// - Creates the container in Azure Blob Storage
pub async fn create_azure_container(container_name: &str) -> azure_core::Result<()> {

    let blob_client = get_az_client().blob_client(container_name, "");

    // Create the container
    let container = blob_client.container_client().create().public_access(PublicAccess::None).await;
    
    // Unwrap the result
    let container = match container {
        Ok(_) => {
            info!("Successfully created container: {}", container_name);
            Ok(())
        }
        Err(e) => {
            error!("Error creating container: {}", e);
            Err(e)
        }
    };
    
    container
}

/// Delete a file from Azure Blob Storage
/// - Establishes a connection to Azure Blob Storage via azure_key.json
/// - Deletes the file from Azure Blob Storage
pub async fn delete_azure_blob(container_name: &str, filename: &str) -> azure_core::Result<()> {

    let blob_client = get_az_client().blob_client(container_name, filename);

    // Delete the blob
    let blob = blob_client.delete().await;
    
    // Unwrap the result
    let blob = match blob {
        Ok(_) => {
            info!("Successfully deleted blob: {:?}", filename);
            Ok(())
        }
        Err(e) => {
            error!("Error deleting blob data: {}", e);
            Err(e)
        }
    };
    
    blob
}

/// Deletes a container from Azure Blob Storage
/// - Establishes a connection to Azure Blob Storage via azure_key.json
/// - Deletes the container from Azure Blob Storage
pub async fn delete_azure_container(container_name: &str) -> azure_core::Result<()> {

    let blob_client = get_az_client().blob_client(container_name, "");
    // Delete the container
    let container = blob_client.container_client().delete().await;
    
    // Unwrap the result
    let container = match container {
        Ok(_) => {
            info!("Successfully deleted container: {:?}", container_name);
            Ok(())
        }
        Err(e) => {
            error!("Error deleting container data: {}", e);
            Err(e)
        }
    };
    
    container
}

/// Get client for Azure Blob Storage connection
/// - Establishes a connection to Azure Blob Storage via azure_key.json
pub fn get_az_client() -> ClientBuilder {
    // Retrieve mandatory details from json file
    let az_details = get_azure_details().unwrap();

    // Assign details to variables
    let account = az_details.storage_account_name;
    let key = az_details.storage_account_key;

    // Create a blob client
    let storage_credentials = StorageCredentials::Key(account.clone(), key);
    let client =
        ClientBuilder::new(account, storage_credentials);

    client
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
