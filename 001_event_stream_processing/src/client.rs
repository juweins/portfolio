/*
    This file contains the functions to connect to Kafka / Azure
*/

use std::borrow::Borrow;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{StreamConsumer,Consumer, CommitMode};

use crate::get_kafka_details;

// implement custom trait to clone consumer
pub trait ToOwned {
    // The resulting type after obtaining ownership.
    type Owned: Borrow<Self>;
    #[must_use = "cloning is often expensive and is not expected to have side effects"]
    fn to_owned(&self) -> Self::Owned;
    fn clone_into(&self, target: &mut Self::Owned) {
        *target = self.to_owned();
    }
}

// implement the trait for the consumer
impl ToOwned for StreamConsumer {
    // The resulting type after obtaining ownership.
    type Owned = StreamConsumer;

    // the function that clones the consumer
    fn to_owned(&self) -> StreamConsumer {
        self.to_owned()
    }
}

pub async fn new_kafka_consumer() -> StreamConsumer {
    // read the kafka details from a file and store them in a vector
    let kafka_details = get_kafka_details().unwrap();

    // Assign details to variables (for readability)
    let bootstrap_servers = kafka_details.bootstrap_servers;
    let group_id = kafka_details.group_id;
    let message_timeout_ms = kafka_details.message_timeout_ms;

    // Create a new Kafka consumer (if not already existing)
    let consumer: &StreamConsumer = &ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("group.id", &group_id)
        .set("message.timeout.ms", &message_timeout_ms.to_string())
        .create()
        .expect("Error: Failed to create Kafka consumer");
    
    // return the consumer by lifetime
    consumer.to_owned()
}

pub async fn new_kafka_producer() -> FutureProducer {
    // read the kafka details from a file and store them in a vector
    let kafka_details = get_kafka_details().unwrap();

    // Assign details to variables (for readability)
    let bootstrap_servers = kafka_details.bootstrap_servers;
    let group_id = kafka_details.group_id;
    let message_timeout_ms = kafka_details.message_timeout_ms;

    // Create a new Kafka producer (if not already existing)
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("group.id", &group_id)
        .set("message.timeout.ms", &message_timeout_ms.to_string())
        .create()
        .expect("Error: Failed to create Kafka producer");

    // return the producer
    producer.to_owned()
}
