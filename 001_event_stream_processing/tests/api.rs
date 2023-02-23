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
        let result = request_data("test_api", "2023-01-01", "2023-01-28").await;
        println!("{}", &result.unwrap());

    }

    #[tokio::test]
    async fn invalid_api() {
        let result = request_data("not_configured_api", "2023-01-01", "2023-01-28").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn invalid_start_date() {
        let result = request_data("exchangerates_api", "2999-01-01", "2023-01-28").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn invalid_end_date() {
        let result = request_data("exchangerates_api", "2023-01-01", "1000-01-44").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn invalid_start_and_end_date() {
        let result = request_data("exchangerates_api", "9999-01-01", "1000-01-28").await;
        assert!(result.is_err());
    }


}