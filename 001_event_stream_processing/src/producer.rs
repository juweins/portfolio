/*
    This file contains the function that pushes the API response to Kafka
    It mimics the producer in the project setup
*/

use crate::client::new_kafka_producer;

use std::io::Error;

use bytecount::num_chars;
use log::{warn, info};
use uuid::Uuid;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::error::{KafkaError, KafkaResult};



use std::time::Duration;

/// Pushes a message to the Kafka topic. (Kafka Producer)
/// - Establishes a connection to the Kafka broker as defined in the kafka_key.json file
/// - Creates a new record with the message content (e.g. the saved API response)
/// - Sends the record
/// - Returns the number of messages sent
pub async fn push_to_kafka(topic_name: &str, message_content: &str) -> Result<u8, KafkaError> {

    // Initialize the unique uuid as key (Maybe use a timestamp instead?)
    let key =  Uuid::new_v4().to_string();

    // Initialize the Kafka producer
    let producer = new_kafka_producer().await;

    // Temporary: Read a saved response from a file
    // This is to avoid hitting the API limit early on
    let mut response_body: Vec<u8> = Vec::new();

    response_body = std::fs::read(message_content.to_string()).unwrap();

    // Create a new record
    let record = FutureRecord::to(topic_name)
        .payload(&response_body)
        .key(&key);

    // Send the record
    let delivery_status = producer.send(record, Duration::from_secs(0)).await;

    // Return the result of the operation to CLI
    match delivery_status {
        Ok(_) => {
            // Calculate the number of bytes sent
            let bytes_sent: u8 = bytecount::num_chars(&response_body) as u8;

            info!("Message sent successfully");
            info!("Key: {}", &key);
            info!("Bytes sent: {}", bytes_sent);

            Ok(bytes_sent)
        },
        Err(e) => {
            warn!("Error while sending message: {:?}", e);
            Err(KafkaError::NoMessageReceived)
        }
    }
}
