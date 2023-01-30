// Maps to the azure_key.json file

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AzRequest {
    pub storage_account_name: String,
    pub storage_account_key: String,
    pub storage_container: String,
    pub storage_blob_name: String,
}
