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

    use exchange_stream::client::{new_kafka_producer, new_kafka_consumer};
    use exchange_stream::producer::push_to_kafka;
    use exchange_stream::consumer::read_from_kafka;
    use rdkafka::producer::FutureRecord;
    use rdkafka::util::Timeout;

    #[tokio::test]
    async fn test_push_to_kafka() {
        let result = push_to_kafka("test", true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_read_from_kafka() {
        let topic = "test";
        let test = true;

            let push = new_kafka_producer().await;
            let record = FutureRecord::to("test")
                .payload("test_message")
                .key("key");
            let delivery_status = push.send(record, Timeout::After(Duration::from_secs(1)));

            let pull = read_from_kafka("test", test).await;

            assert!(pull.is_ok());
            assert_eq!(pull.unwrap(), 0);
    }

}