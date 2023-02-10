/*
    This file contains the function that pushes the API response to Kafka
    It mimics the producer in the project setup
*/

use crate::get_kafka_details;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

use reqwest::Error;
use std::time::Duration;

// Take API response and push it to (local) Kafka
// - The function uses the rdkafka crate
// - The function uses the kafka_key.json file for necessary details
// TODO: Explode the function into smaller parts
pub async fn push_to_kafka(topic_name: &str) -> Result<(), Error> {
    // read the kafka details from a file and store them in a vector
    let kafka_details = get_kafka_details().unwrap();

    // Assign details to variables
    let bootstrap_servers = kafka_details.bootstrap_servers;
    let group_id = kafka_details.group_id;
    let message_timeout_ms = kafka_details.message_timeout_ms;

    // Create a new Kafka producer
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("group.id", &group_id)
        .set("message.timeout.ms", &message_timeout_ms.to_string())
        .create()
        .expect("Error: Failed to create Kafka producer");

    // Temporary: Read a saved response from a file
    // This is to avoid hitting the API limit early on
    let response_body = std::fs::read("example_response.json").unwrap();

    // Create a new record
    let record = FutureRecord::to(topic_name)
        .payload(&response_body)
        .key("key");

    // Send the record
    let delivery_status = producer.send(record, Duration::from_secs(0)).await;
    println!("Delivery status: {:?}", delivery_status);

    Ok(())
}
