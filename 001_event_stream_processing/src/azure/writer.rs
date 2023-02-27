use azure_storage_blobs::prelude::PublicAccess;
use log::{info, warn, error};

use crate::azure::helper::get_az_client;




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
        .put_block_blob(content.to_string())
        .content_type("application/json")
        .await?;
    info!("Successfully created blob: {:?}", blob.request_id);

    Ok(())
}
