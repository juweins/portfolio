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
    use exchange::{push_to_azure, pull_from_azure, request_data};

    // Initalize the container name and filename as constants
    const test_container_name: &str = "test";
    const test_filename: &str = "data/test_file.json";

    // TODO: Test for alternative container/filename combinations
    // - Should test for correct error handling
    
    // TODO: Requires helper function to delete the container/file after the test
    #[tokio::test]
    async fn test_push_to_azure() {
        let result = push_to_azure(test_container_name, test_filename).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pull_from_azure() {
        let result = pull_from_azure(test_container_name, test_filename).await;
        assert!(result.is_ok());
    }
}
