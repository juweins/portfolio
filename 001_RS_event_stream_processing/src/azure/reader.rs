/*
    This file contains the function that pulls data from Azure Blob Storage
*/

use log::{info, warn, error};
use std::io::Write;

use crate::azure::helper::get_az_client;

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
