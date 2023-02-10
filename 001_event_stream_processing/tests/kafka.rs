/*
    This file contains the integration tests for the kafka parts
    - The tests are run by calling cargo test
    - The kafka related functions are:
        - get_kafka_details
        - push_to_kafka
*/

#[cfg(test)]
mod tests {
    use exchange_stream::producer::push_to_kafka;

    #[tokio::test]
    async fn test_push_to_kafka() {
        let result = push_to_kafka("test").await;
        assert!(result.is_ok());
    }
}
