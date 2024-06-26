#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_cmd_shows_error_and_exits_non_zero_if_config_invalid() {
        let output = Command::new(env!("CARGO_BIN_EXE_mockerize-cli"))
            .arg("test")
            .arg("tests/data/invalid.server.json")
            .output()
            .expect("Failed to execute process");

        assert!(!output.status.success());
        assert_eq!(output.status.code().unwrap(), 1); // Verify exit code

        // stdout should be "ERROR" followed by newline
        // stderr should detail the underlying error
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert_eq!(stdout, "ERROR\n");

        // Verify that we see details about our (intentional) syntax error
        assert!(stderr.contains("tests/data/invalid.server.json"));
        assert!(stderr.contains("line 3 column 16"));
    }

    #[test]
    fn test_cmd_shows_ok_with_zero_exit_code_on_success() {
        let output = Command::new(env!("CARGO_BIN_EXE_mockerize-cli"))
            .arg("test")
            .arg("tests/data/example.server.json")
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success()); // Verify exit code is success

        // The output should be "OK" followed by newline
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert_eq!(stdout, "OK\n");
    }
}
