/*
    This file contains the integration tests for the api parts
    - The tests are run by calling cargo test <test file name>
    - The azure related functions are:
        - request_data
*/

// Test the pull/push functions (request_data)
// - The functions are tested by calling them and checking if the result is Ok
#[cfg(test)]
mod tests {

    use exchange::{request_data};

    #[tokio::test]
    async fn valid_api() {
        let result = request_data("test_api").await;
        println!("{}", &result.unwrap());

    }

    #[tokio::test]
    async fn test_exchange_rates_api() {
        let result = request_data("exchangerates_api").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn invalid_api() {
        let result = request_data("not_configured_api").await;
        assert!(result.is_err());
    }
}