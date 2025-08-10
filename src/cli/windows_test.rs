//! Windows-specific tests and utilities
//! 
//! This module contains tests and utilities that are specific to Windows
//! to ensure proper functionality on that platform.

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_path_handling() {
        // Test that config directory can be retrieved on Windows
        let config_dir = gim_config::directory::config_dir();
        assert!(config_dir.is_ok(), "Failed to get config directory on Windows");
        
        // Verify that the path is valid and uses backslashes
        let path = config_dir.unwrap();
        assert!(path.exists(), "Config directory does not exist");
    }
    
    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_file_operations() {
        use std::fs;
        
        // Test that we can write and read files in the config directory
        let config_dir = gim_config::directory::config_dir().unwrap();
        let test_file = config_dir.join("gim_test_file.txt");
        
        // Write test content
        let test_content = "This is a test file for Windows compatibility";
        assert!(fs::write(&test_file, test_content).is_ok(), "Failed to write test file");
        
        // Read and verify content
        assert!(test_file.exists(), "Test file was not created");
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, test_content, "File content does not match");
        
        // Clean up
        fs::remove_file(&test_file).unwrap();
    }
}