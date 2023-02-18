/*
    This file contains the function that reads the API response as an event from Kafka
    It mimics the consumer in the project setup
*/

use std::time::Duration;

use crate::client::new_kafka_consumer;
use crate::get_kafka_details;

use rdkafka::error::{KafkaError, KafkaResult};
use log::{warn, info};
use rdkafka::{Message};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer,BaseConsumer};




pub async fn read_from_kafka(topic: &str, test: bool) -> Result<(u8), KafkaError>{

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

    // Create a new Kafka consumer (if not already existing)
    let consumer: &BaseConsumer = &ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("group.id", &group_id)
        .set("message.timeout.ms", &message_timeout_ms.to_string())
        .set("enable.auto.commit", "true")
        .set("connections.max.idle.ms", "1000")
        .create()
        .expect("Error: Failed to create Kafka consumer");


    // Initialize the message counter
    // - u8 is ok in this case since we only want to process a few messages
    let mut message_counter: u8 = 0;
    let mut retry_counter: u8 = 0;

    // Initialize the message
    let event_message: String = "".to_string();

    // Subscribe to the topic
    consumer.subscribe(&[topic]).expect("Error: Failed to subscribe to topic");
    info!("Subscribed to topic: {}", topic);

    info!("Consumer starts...");
    loop {
        // poll the stream for a message
        let message = consumer.poll(Duration::from_secs(5));

        // Initialize variable to check idle state
        let msg = message;

        // check if the stream is idle
        if msg.is_none() {
            
            info!("Listening for messages... (retry={})", retry_counter);

            // if the stream is idle, increase the retry counter
            retry_counter += 1;

            // put the thread to sleep for 2 seconds
            tokio::time::sleep(Duration::from_secs(2)).await;

            // if max retries, break the loop to exit the consumer stream
            if retry_counter > 5 {
                break;
            }
        } else {

            // if the stream is not idle, reset the retry counter
            retry_counter = 0;

            // increase the message counter
            message_counter += 1;

            // convert the message to a readable string in stdout
            let message = msg.unwrap().unwrap().payload().unwrap().to_owned();
            let readable_message = String::from_utf8(message).unwrap();

            // print the message
            println!("Message: {}", readable_message);
        }

    }
        info!("Terminate listening for messages... (max retries reached)");
        println!("Message counter: {}", message_counter);
        Ok(message_counter)
}