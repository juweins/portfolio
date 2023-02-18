/*
    This file contains the integration tests for the kafka parts
    - The tests are run by calling cargo test
    - The kafka related functions are:
        - get_kafka_details
        - push_to_kafka
*/

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use exchange::client::{new_kafka_producer, new_kafka_consumer};
    use exchange::producer::push_to_kafka;
    use exchange::consumer::read_from_kafka;
    use rdkafka::error::KafkaError;
    use rdkafka::producer::FutureRecord;
    use rdkafka::util::Timeout;

    #[tokio::test]
    async fn test_push_to_kafka() {
        let result = push_to_kafka("test", "example_response.json").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_read_from_kafka() {

        // Initialize the topic and test variables
        let topic = "test";
        let test = true;
        let message = "test_message";
        let message_bytes = bytecount::num_chars(message.as_bytes()) as u8;

            // Create a new Kafka producer (Consumer is not working here, yet)
            let push = new_kafka_producer().await;
            
            // Send consumer in a background thread to avoid blocking
            // Capture the result in a variable for testing
            let consumer = tokio::spawn(async move {
                let result = read_from_kafka(topic, test).await;
                result
            });

            // Push a message to the topic
            // Capture result to test for successful producer push
            let producer = push.send(
                FutureRecord::to(topic)
                    .payload(message)
                    .key("test"),
                Timeout::After(Duration::from_secs(1)),
            ).await;

            // Wait for the consumer to finish by timeout (idle mode)
            let result = consumer.await.unwrap().expect("Error: Consumer failed to read from Kafka");


            let message_count = result.0;
            let message_size = result.1;
            let total_size = result.2;

            // Check if tasks were successful
            assert!(producer.is_ok());
            assert_eq!(message_count, 1);
            assert_eq!(message_size[0], message_bytes); // Only one message is pushed atm.
            assert_eq!(total_size, message_bytes as u32); // Should therefore be the same as the message size
    }

}