use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::{ClientBuilder, PublicAccess};

use log::{info, warn, error};

// import get_azure_details function from lib.rs in src/lib.rs
use crate::get_azure_details;





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


