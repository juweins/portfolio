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
        let result = push_to_kafka("test", true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_read_from_kafka() {

        // Initialize the topic and test variables
        let topic = "test";
        let test = true;
        let message = "test_message";

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
            let consumer = consumer.await.unwrap();

            // Check if tasks were successful
            assert!(producer.is_ok());
            assert!(consumer.is_ok());

            // Check if the consumer received the message
            assert_eq!(consumer.unwrap(), 1)
    }

}