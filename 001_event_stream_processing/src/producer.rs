/*
    This file contains the function that pushes the API response to Kafka
    It mimics the producer in the project setup
*/

use crate::client::new_kafka_producer;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::error::{KafkaError, KafkaResult};

use std::time::Duration;

// Take API response and push it to (local) Kafka
// - The function uses the rdkafka crate
// - The function uses the kafka_key.json file for necessary details
// TODO: Explode the function into smaller parts
pub async fn push_to_kafka(topic_name: &str, test: bool) -> Result<(), KafkaError> {

    // Initialize the Kafka producer
    let producer = new_kafka_producer().await;
    // Temporary: Read a saved response from a file
    // This is to avoid hitting the API limit early on
    // TODO: Remove this and use the API response directly

    // TODO: Remove test flag - development only!
    let mut response_body: Vec<u8> = Vec::new();
    if test {
        response_body = "test_message".as_bytes().to_vec();
    } else {
        response_body = std::fs::read("example_response.json").unwrap();
    }

    // Create a new record
    let record = FutureRecord::to(topic_name)
        .payload(&response_body)
        .key("key");

    // Send the record
    let delivery_status = producer.send(record, Duration::from_secs(0)).await;
    println!("Delivery status: {:?}", delivery_status);

    Ok(())
}
