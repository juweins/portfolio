/*
    This file contains the integration tests for the azure parts
    - The tests are run by calling cargo test
    - The azure related functions are:
        - push_data
        - pull_data
*/

// Test the pull/push functions (request_data and push_data)
// - The functions are tested by calling them and checking if the result is Ok
#[cfg(test)]
mod tests {

    use exchange::{push_to_azure, request_data};

    #[tokio::test]
    async fn test_request_data() {
        let result = request_data("exchangerates_api", "2023-01-01", "2023-01-28").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_push_data() {
        let result = push_to_azure().await;
        assert!(result.is_ok());
    }
}
