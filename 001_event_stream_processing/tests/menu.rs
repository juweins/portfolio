/*
    This file contains the integration tests for the menu items.
    It tests the correct behaviour of the menu items and arguments.
*/

// TODO: Mock the kafka server (mockito?)
// TODO: Mocking the azure blob possible?
// TODO: Mock the API calls to prevent rate limiting
#[cfg(test)]
mod tests {
    
    use std::process::Command;

    // Correct input -> Correct behaviour
    #[test]
    fn test_menu_produce() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("produce")
            .arg("--topic")
            .arg("test")
            .arg("--file")
            .arg("example_response.json")
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());    
    }

    // Correct input -> Correct behaviour
    #[test]
    fn test_menu_consume() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("consume")
            .arg("--topic")
            .arg("test")
            .arg("--ttl")
            .arg("10")
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());    
    }

    // Correct input -> Correct behaviour
    #[test]
    fn test_menu_write() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("write")
            .arg("--container-name")
            .arg("test")
            .arg("--file")
            .arg("example_response.json")
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());
    }

    // Correct input -> Correct behaviour
    // This calls the httpbin/json api
    #[test]
    fn test_menu_request() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("request")
            .arg("--api-name")
            .arg("exchangerates_api")
            .arg("--start-date")
            .arg("2021-01-01")
            .arg("--end-date")
            .arg("2021-01-28")
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());
    }
}