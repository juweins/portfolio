// Contains structs used to parse configurations
use serde::{Deserialize, Serialize};

// struct for the kafka_key.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub group_id: String,
    pub message_timeout_ms: u32,
}