use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

use crate::validators::error_messages::{INVALID_FILE_PATH, ERROR_READING_FILE};

/// This regex only accept classical char for a file name / path
static REGEX_FILE_PATH: &str = r"[a-zA-Z0-9\/\.\\_-]+";

/// Check if the given file path is valid
/// # Arguments
/// * `file_path` - The file_path to check
/// # Returns
/// * `bool` - True if the file_path is valid, false otherwise
pub fn is_valid_file_path(file_path: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!("^{}$", REGEX_FILE_PATH)).unwrap();
    }
    RE.is_match(file_path)
}

/// Read the content of the given file path
/// # Arguments
/// * `file_path` - The file_path to read
/// # Returns
/// * `Vec<u8>` - The content of the file in bytes, or an error message
/// if the path is invalid or is not readable
/// # Errors
/// * `&str` - The error message
pub fn read_from_path(file_path: &str) -> Result<Vec<u8>, &str> {
    if !is_valid_file_path(file_path) {
        return Err(INVALID_FILE_PATH);
    }

    let buffer = fs::read(file_path);
    return match buffer {
        Ok(buffer) => Ok(buffer),
        Err(_) => Err(ERROR_READING_FILE),
    };
}
