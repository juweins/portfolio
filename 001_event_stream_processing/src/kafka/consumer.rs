/*
    This file contains the function that reads the API response as an event from Kafka
    It mimics the consumer in the project setup
*/

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use bytecount::num_chars;
use log::{warn, info};

use crate::get_kafka_details;

use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::{Message};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer,BaseConsumer, CommitMode};


/// Reads from the Kafka topic. (Kafka Consumer)
/// - Establishes a connection to the Kafka broker as defined in the kafka_key.json file
/// - Reads the messages from the topic
/// 
/// - Returns a tuple (total number of messages read, byte size of the individual messages, total byte size transmitted)
pub async fn read_from_kafka(topic: &str, time_to_live: u8) -> Result<(u8, HashMap<u8, String>,Vec<u8>, u32), KafkaError>{

    // TODO: Why does calling new_kafka_consumer() here not work?
    // - It works in the producer.rs file
    // - It works in the client.rs file
    // - It does not work here
    // results in an stack overflow error

    // let consumer = new_kafka_consumer().await;

    // TODO: remove this and use the function above instead
    // - This is a temporary solution to avoid the stack overflow error
    // - The function is not working here, yet


    // Read the kafka details from a file and store them in a vector
    let kafka_details = get_kafka_details().unwrap();

    // Assign details to variables (for readability)
    let bootstrap_servers = kafka_details.bootstrap_servers;
    let group_id = kafka_details.group_id;
    let message_timeout_ms = kafka_details.message_timeout_ms;
    let connection_max_idle_ms = kafka_details.connection_max_idle_ms;

    // Create a new Kafka consumer (if not already existing)
    let consumer: &BaseConsumer = &ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("group.id", &group_id)
        .set("message.timeout.ms", &message_timeout_ms.to_string())
        .set("connections.max.idle.ms", &connection_max_idle_ms.to_string())
        .create()
        .expect("Error: Failed to create Kafka consumer");


    // Initialize the message counter
    // - u8 is ok in this case since we only want to process a few messages
    let mut message_counter: u8 = 0;
    let mut retry_counter: u8 = 0;

    // Initialize a vector to store the bytes of consumed messages 
    let mut message_bytes: Vec<u8> = Vec::new();

    // Initialize a vector to store all messages as strings
    let mut message_content: HashMap<u8, String> = HashMap::new();



    // Subscribe to the topic
    consumer.subscribe(&[topic]).expect("Error: Failed to subscribe to topic");

    info!("Subscribed to topic: {}", topic);
    info!("Consumer starts...");

    loop {
        // Poll the stream for a message
        let message = consumer.poll(Duration::from_secs(time_to_live as u64));

        // Initialize variable to check idle state
        let msg = message;

        // Check if the stream is idle
        if msg.is_none() {
            
            info!("Listening for messages... (retry={})", retry_counter);

            // If the stream is idle, increase the retry counter
            retry_counter += 1;

            // Put the thread to sleep for 2 seconds
            tokio::time::sleep(Duration::from_secs(2)).await;

            // If max retries, break the loop to exit the consumer stream
            if retry_counter > 5 {
                break;
            }
        } else if msg.is_some() {

            // If the stream is not idle, reset the retry counter
            retry_counter = 0;
            
            // Increase the message counter
            message_counter += 1;

            // Convert the message to a readable string in stdout
            match msg {
                Some(Ok(m)) => {
                    let message = m.payload().unwrap().to_owned();
                    message_content.insert(message_counter, String::from_utf8(message.clone()).unwrap());

                    let bytes_received = bytecount::num_chars(&message) as u8;
                    message_bytes.push(bytes_received);

                    // Print the message (debugging/development)
                    // TODO: change this before 1.0.0 // to info!()
                    info!("Message: {}", String::from_utf8(message).unwrap());
                    consumer.commit_message(&m, CommitMode::Sync).unwrap();

                },
                Some(Err(e)) => {
                    warn!("Error while reading from stream: {}", e);
                },
                None => {
                    warn!("Unexpected empty message");
                    warn!("Shutting down consumer");
                    // Looking at implementations in other languages (e.g. Java)
                    // There is no complement to wakeup() / close() function in rdkafka gracefully
                    // Therefore, we break the loop with an empty message
                    break;
                }
            }
            // let message = msg.unwrap().unwrap().payload().unwrap().to_owned();
            // let readable_message = String::from_utf8(message).unwrap();
        }
    }

        // Clone the message vector to calculate the total byte size
        let vector = message_bytes.clone();

        let mut total_bytes: u32 = 0;

        for i in vector {
            total_bytes += i as u32;
        }

        info!("Terminate listening for messages... (max retries reached)");
        info!("Message received: {}", message_counter);
        Ok((message_counter, message_content, message_bytes, total_bytes))
}

pub async fn new_kafka_consumer() -> BaseConsumer {
    // read the kafka details from a file and store them in a vector
    let kafka_details = get_kafka_details().unwrap();

    // Assign details to variables (for readability)
    let bootstrap_servers = kafka_details.bootstrap_servers;
    let group_id = kafka_details.group_id;
    let message_timeout_ms = kafka_details.message_timeout_ms;

    // Create a new Kafka consumer (if not already existing)
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("group.id", &group_id)
        .set("message.timeout.ms", &message_timeout_ms.to_string())
        .set("enable.auto.commit", "true")
        .set("connections.max.idle.ms", "1000")
        .create()
        .expect("Error: Failed to create Kafka consumer");

    // return the consumer
    consumer
}