/*
    This file contains the integration tests for the azure parts
    - The tests are run by calling cargo test
    - The azure related functions are:
        - push_data
        - pull_data
        - delete_data
        - delete_container
*/

// Test the pull/push functions (request_data and push_data)
// - The functions are tested by calling them and checking if the result is Ok
#[cfg(test)]
mod tests {
    use exchange::{push_to_azure, pull_from_azure};
    use exchange::{create_azure_container};
    use exchange::{delete_azure_blob, delete_azure_container};

    use rand::Rng;

    // Initalize the container name and filename as constants
    const test_container_name: &str = "test";
    const test_filename: &str = "data/test_file.json";
    const test_blob_name: &str = "test_file.json";

    #[tokio::test]
    async fn test_crud_container() {
        let random_name = format!("test{}", rand::random::<u8>());

        let result_create = create_azure_container(&random_name).await;
        let result_delete = delete_azure_container(&random_name).await;

        assert!(result_create.is_ok());
        assert!(result_delete.is_ok());
    }

    #[tokio::test]
    async fn test_delete_from_azure() {

        let result_create = push_to_azure(test_container_name, test_filename, test_blob_name).await;
        let result_delete = delete_azure_blob(test_container_name, test_filename).await;

        assert!(result_create.is_ok());
        assert!(result_delete.is_ok());
    }

    // TODO: Test for alternative container/filename combinations
    // - Should test for correct error handling
    #[tokio::test]
    async fn test_push_to_azure() {
        let result = push_to_azure(test_container_name, test_filename, test_blob_name).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pull_from_azure() {
        let result = pull_from_azure(test_container_name, test_filename).await;
        assert!(result.is_ok());
    }
}
