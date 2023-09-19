/*
    This file contains the integration tests for the azure parts
    - The tests are run by calling cargo test
    - The azure related functions are:
        - create_and_delete_container
        - push_data
        - pull_data
*/

// Test the pull/push functions (request_data and push_data)
// - The functions are tested by calling them and checking if the result is Ok
#[cfg(test)]
mod tests {
    use exchange::azure::writer::{push_to_azure};
    use exchange::azure::helper::{create_azure_container, delete_azure_blob, delete_azure_container};
    use exchange::azure::reader::{pull_from_azure};

    // Initalize the container name and filename as constants
    const TEST_CONTAINER_NAME: &str = "test";
    const TEST_FILENAME: &str = "data/test_file.json"; // local file
    const TEST_BLOB_NAME: &str = "test_file.json";

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

        let result_create = push_to_azure(TEST_CONTAINER_NAME, TEST_FILENAME, TEST_BLOB_NAME).await;
        let result_delete = delete_azure_blob(TEST_CONTAINER_NAME, TEST_FILENAME).await;

        assert!(result_create.is_ok());
        assert!(result_delete.is_ok());
    }

    #[tokio::test]
    async fn test_push_to_azure() {
        let result = push_to_azure(TEST_CONTAINER_NAME, TEST_FILENAME, TEST_BLOB_NAME).await;
        let _ = delete_azure_blob(TEST_CONTAINER_NAME, TEST_BLOB_NAME).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pull_from_azure() {
        let _ = push_to_azure(TEST_CONTAINER_NAME, TEST_FILENAME, TEST_BLOB_NAME).await;
        let result = pull_from_azure(TEST_CONTAINER_NAME, TEST_BLOB_NAME).await;
        let _ = delete_azure_blob(TEST_CONTAINER_NAME, TEST_BLOB_NAME).await;
        assert!(result.is_ok());
    }
}
