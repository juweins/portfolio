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
            .arg("data/example_response.json")
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
            .arg("test_topic")
            .arg("--ttl")
            .arg("1")
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
            .arg("data/example_response.json")
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
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());
    }

    #[test]
    fn test_menu_read() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("read")
            .arg("--container-name")
            .arg("test")
            .arg("--file")
            .arg("data/example_response.json")
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());
    }

    #[test]
    fn test_menu_ingest() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("ingest")
            .arg("--api-name")
            .arg("test_api")
            .arg("--topic")
            .arg("test_topic")
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());
    }

    #[test]
    fn test_menu_forward() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("forward")
            .arg("--topic")
            .arg("test_topic")
            .arg("--container-name")
            .arg("test_container")
            .arg("--file")
            .arg("test_file.json")
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());
    }

    #[test]
    fn test_menu_version() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("version")
            .output()
            .expect("Failed to execute process");

        let version = env!("CARGO_PKG_VERSION");

        assert!(output.status.success());
        assert_eq!(String::from_utf8(output.stdout).unwrap(), format!("Version: {}\n", version));
    }
}