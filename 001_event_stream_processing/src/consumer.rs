/*
    This file contains the function that reads the API response from Kafka
    It mimics the consumer in the project setup
*/

use crate::client::new_kafka_consumer;

use futures::TryStreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, CommitMode};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use futures::stream::Stream;

use rdkafka::producer::FutureRecord;
use reqwest::Error;
use std::time::Duration;



pub async fn read_from_kafka(topic: &str) -> Result<(u8), Error>{
    
    // Initialize the Kafka consumer
    let consumer = new_kafka_consumer().await;

    // Subscribe to the topic
    consumer.subscribe(&[topic]).expect("Error: Failed to subscribe to topic");

    // Create a stream of messages
    consumer
        .subscribe(&[&topic])
        .expect("Can't subscribe to specified topic");

    // process the messages
    let mut message_count = 0;

    // listen to the stream
    let stream_processor = consumer.stream().try_for_each(|message| {
        // move the message to a new thread and process it
        async move {
            println!("Message received: {:?}", message);
            message_count += 1;
            Ok(())
    }});
    Ok((message_count))

}
