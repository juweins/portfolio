/*
    This file contains the integration tests for the kafka parts
    - The tests are run by calling cargo test
    - The kafka related functions are:
    - get_kafka_details
    - push_to_kafka
*/

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use exchange::kafka::producer::{new_kafka_producer, push_to_kafka};
    use exchange::kafka::consumer::{new_kafka_consumer, read_from_kafka};
    use exchange::request_data;

    use rdkafka::producer::FutureRecord;
    use rdkafka::util::Timeout;

    const TEST_API: &str = "test_api";
    const TEST_TOPIC: &str = "test";
    const TEST_MESSAGE: &str = "test_message";
    const TEST_MESSAGE_BYTES: u8 = 12;
    const TEST_KEY : &str = "00000000-0000-0000-0000-000000000000";
    const TEST_FILE: &str = "data/example_response.json";


    #[tokio::test]
    async fn test_push_to_kafka() {

        let test_message = std::fs::read_to_string(TEST_FILE).unwrap();
        let test_message_bytes = bytecount::num_chars(test_message.as_bytes()) as u8;

        let result = push_to_kafka(TEST_TOPIC, &test_message).await.expect("Error: Failed to push to Kafka");


        let message_count = result.0;
        let message_size = result.1;
        let key = result.2;

        assert_eq!(message_count, 1);
        assert_eq!(message_size, test_message_bytes);
        assert_eq!(key.len(), 36);
    }

    #[tokio::test]
    async fn test_read_from_kafka() {

        // Initialize the topic
        // This topic is used solely for this test
        // If the topic is also test, the read test will fail. (2+ messages are in the topic)
        let test_read_topic = "test_read";

        // Create a new Kafka producer (Consumer is not working here, yet)
        let push = new_kafka_producer().await;

        // Send consumer in a background thread to avoid blocking
        // Capture the result in a variable for testing
        // TTL is set to 2 second to run quickly
        let consumer = tokio::spawn(async move {
            let result = read_from_kafka(test_read_topic, 2).await;
            result
        });
        // Push a message to the topic
        // Capture result to test for successful producer push
        let producer = push.send(
            FutureRecord::to(test_read_topic)
                .payload(TEST_MESSAGE)
                .key(TEST_KEY),
            Timeout::After(Duration::from_secs(1)),
        ).await;

        // Wait for the consumer to finish by timeout (idle mode)
        let result = consumer.await.unwrap().expect("Error: Consumer failed to read from Kafka");

        let message_count = result.0;
        let message_content = result.1.get(&1).unwrap(); // since message_count is 1, the message index starts with 1
        let message_size = result.2;
        let total_size = result.3;

        // Check if tasks were successful
        assert!(producer.is_ok());
        assert_eq!(message_count, 1);
        assert_eq!(message_content, TEST_MESSAGE);
        assert_eq!(message_size[0], TEST_MESSAGE_BYTES); // Only one message is pushed atm.
        assert_eq!(total_size, TEST_MESSAGE_BYTES as u32); // Should therefore be the same as the message size
    }

    #[tokio::test]
    async fn test_ingestion() {

        // This test is for the ingestion of data from the API & push to Kafka
        let response = Arc::new(request_data(TEST_API).await.expect("Error: Failed to get data from API"));
        let message_size = bytecount::num_chars(response.to_string().as_bytes()) as u8;
        let produce = push_to_kafka(TEST_TOPIC, &response.to_string()).await.expect("Error: Failed to push to Kafka");
    
        assert!(response.is_object());
        assert_eq!(produce.0, 1);
        assert_eq!(produce.1, message_size);
        assert_eq!(produce.2.len(), 36);
    }
}