//! File I/O operations for saving and loading effort data.

use std::fs::File;
use std::io::Write;

use crate::models::EffortsDto;

/// Saves efforts data to a JSON file.
///
/// # Arguments
/// * `efforts` - Reference to the EffortsDto to save
/// * `path` - File path where data should be saved
///
/// # Returns
/// * `Ok(())` - If the file was successfully saved
/// * `Err(std::io::Error)` - If there was an error writing the file
///
/// # Examples
/// ```no_run
/// # use project_app::file_io::save_efforts_to_file;
/// # use project_app::models::EffortsDto;
/// let efforts = EffortsDto::default();
/// save_efforts_to_file(&efforts, "test.json").expect("Failed to save");
/// ```
pub fn save_efforts_to_file(efforts: &EffortsDto, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(efforts).unwrap();
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// Loads efforts data from a JSON file.
///
/// If the file doesn't exist or contains invalid JSON, returns a default EffortsDto.
///
/// # Arguments
/// * `path` - File path to load data from
///
/// # Returns
/// * `EffortsDto` - The loaded data, or default if loading fails
///
/// # Examples
/// ```no_run
/// # use project_app::file_io::load_efforts_from_file;
/// let efforts = load_efforts_from_file("efforts.json");
/// ```
pub fn load_efforts_from_file(path: &str) -> EffortsDto {
    let json_str = std::fs::read_to_string(path);
    if let Ok(json_str) = json_str {
        let efforts_dto: Result<EffortsDto, serde_json::Error> = serde_json::from_str(&json_str);
        if let Ok(efforts_dto) = efforts_dto {
            return efforts_dto;
        } else {
            println!(
                "{}",
                efforts_dto
                    .expect_err(format!("Error during parse the file \"{}\"", path).as_str())
            );
        }
    } else {
        println!(
            "{}",
            json_str.expect_err(format!("Error during load the file \"{}\"", path).as_str())
        );
    }
    println!("Create a default EffortsDto");
    EffortsDto::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_and_load_roundtrip() {
        let test_file = "test_efforts_temp.json";

        // Create default efforts
        let original = EffortsDto::default();

        // Save to file
        save_efforts_to_file(&original, test_file).expect("Failed to save");

        // Load from file
        let loaded = load_efforts_from_file(test_file);

        // Verify
        assert_eq!(loaded.projects.len(), original.projects.len());
        assert_eq!(loaded.worker_names.len(), original.worker_names.len());

        // Cleanup
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let loaded = load_efforts_from_file("nonexistent_file_12345.json");
        // Should return default
        assert_eq!(loaded.projects.len(), 1);
    }

    #[test]
    fn test_save_creates_file() {
        let test_file = "test_save_creates.json";
        let efforts = EffortsDto::default();

        save_efforts_to_file(&efforts, test_file).expect("Failed to save");

        // Verify file exists
        assert!(std::path::Path::new(test_file).exists());

        // Cleanup
        let _ = fs::remove_file(test_file);
    }
}
