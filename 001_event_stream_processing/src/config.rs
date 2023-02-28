// Contains structs used to parse configurations
use serde::{Deserialize, Serialize};

// struct for the kafka_key.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub group_id: String,
    pub message_timeout_ms: u32,
    pub connection_max_idle_ms: u32,
}

// struct for the azure_key.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct AzureConfig {
    pub storage_account_name: String,
    pub storage_account_key: String,
    pub storage_container: String,
    pub storage_blob_name: String,
}

// structs for the api_key.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiDetails {
    pub url: String,
    pub apikey: String,
    pub description: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APiConfig {
    pub api_name: String,
    pub api_details: ApiDetails,
}
