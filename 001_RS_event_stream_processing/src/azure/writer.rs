/*
    This file contains the function that pushes data to Azure Blob Storage
*/

use azure_storage_blobs::prelude::PublicAccess;
use log::{info, warn, error};

use crate::azure::helper::{get_az_client, create_azure_container};
use super::helper::create_azure_blob;


/// Pushes a file to Azure Blob Storage
/// - Establishes a connection to Azure Blob Storage via azure_key.json
/// - Reads the file to be pushed from the file system
/// - Pushes the file to Azure Blob Storage
/// - Returns a Result with the status of the operation
pub async fn push_to_azure(container_name: &str, blob_name: &str, content: &str) -> azure_core::Result<()> {

    let blob_client = get_az_client().blob_client(container_name, blob_name);
    // Check if container exists
    // - If not, create it
    let container_exists = blob_client.container_client().exists().await?;
    if !container_exists {
        warn!("Container {} does not exist. Creating...", container_name);
        let _container = create_azure_container(container_name).await?;
    }

    // Create a Blob and push file
    let _blob = create_azure_blob(container_name, blob_name, content.as_bytes().to_vec()).await?;

    Ok(())
}
