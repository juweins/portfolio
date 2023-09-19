/*
    This file contains helper functions for Azure Blob Storage
*/

use crate::get_azure_details;
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::{ClientBuilder, PublicAccess};

use log::{info, warn, error};
use uuid::Uuid;

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
    let client = ClientBuilder::new(account, storage_credentials);

    client
}

/// Create a container in Azure Blob
/// - Establishes a connection to Azure Blob Storage via azure_key.json
/// - Creates the container in Azure Blob Storage
/// - Sets the container to public access
/// - Returns the container name
pub async fn create_azure_blob(container_name: &str, filename: &str, data: Vec<u8>) -> azure_core::Result<Uuid> {

    let blob_client = get_az_client().blob_client(container_name, filename);

    // Create the blob
    let blob = blob_client.put_block_blob(data).content_type("application/json").await;
    
    // Unwrap the result
    let blob = match blob {
        Ok(blob) => {
            info!("Successfully created blob {}: {:?}", filename, blob.request_id);
            Ok(blob.request_id)
        }
        Err(e) => {
            error!("Error creating blob data: {}", e);
            Err(e)
        }
    };
    
    blob
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

// -----------
// Unit Tests
// -----------
#[cfg(test)]
mod tests{
    use super::*;
    const TEST_CONTAINER : &str = "test";
    const TEST_CONTAINER_TO_DELETE: &str = "testodelete";
    const TEST_FILE : &str = "test.json";
    const TEST_DATA : &str = "{ \"test\": \"test\" }";

    #[test]
    fn test_get_az_client() {
        // untestable as per crate (azure_storage)
        // TODO: find a way to test this
    }

    #[tokio::test]
    async fn test_create_azure_blob() {
        let _ = create_azure_container(TEST_CONTAINER).await;
        let result = create_azure_blob(TEST_CONTAINER, TEST_FILE, TEST_DATA.as_bytes().to_vec()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_azure_blob() {
        let _ = create_azure_container(TEST_CONTAINER).await;
        let result = delete_azure_blob(TEST_CONTAINER, TEST_FILE).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_azure_container() {
        let result = create_azure_container(TEST_CONTAINER_TO_DELETE).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_azure_container() {
        let result = delete_azure_container(TEST_CONTAINER_TO_DELETE).await;
        assert!(result.is_ok());
    }

}

