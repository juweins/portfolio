// Maps to the azure_key.json file

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KafkaRequest {
    pub kafka_brokers: String,
    pub kafka_topic: String,
}
