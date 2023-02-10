// Contains structs used to parse configurations
use serde::{Deserialize, Serialize};

// struct for the kafka_key.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub group_id: String,
    pub message_timeout_ms: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AzureConfig {
    pub storage_account_name: String,
    pub storage_account_key: String,
    pub storage_container: String,
    pub storage_blob_name: String,
}