use infer::{is_image, is_video, get, Type};

use crate::validators::error_messages::{INVALID_FILE_GROUP, INVALID_FILE_TYPE};
use crate::validators::file_helper::{read_from_path};

/// Validates if the file has the same extension that his file type
/// # Arguments
/// * `file_type` - The file type containing the extension
/// * `file_path` - The file path to validate
/// # Returns
/// * `bool` - True if the file has the same extension that his file type, false otherwise
fn match_extension(file_path: &str, type_file: &Type) -> bool {
    file_path.trim().to_lowercase().ends_with(type_file.extension())
}

/// Verify if the file has a valid extension in case it is a special extension
/// where a mime type can contain multiple extensions.
/// # Arguments
/// * `file_path` - The path of the file to be validated.
/// * `type_file` - The type of the file to be validated.
/// # Returns
/// `true` if the file has a valid special extension, `false` otherwise.
fn is_special_extension(file_path: &str, type_file: &Type) -> bool {
    // We accept special extension (tiff and jpeg) from ref:
    // jpg / jpeg and tif / tiff
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
    match type_file.extension() {
        "jpg" | "jpeg" => file_path.trim().to_lowercase().ends_with("jpeg"),
        "tif" | "tiff" => file_path.trim().to_lowercase().ends_with("tiff"),
        _ => false
    }
}

/// Check if the given file path owns the valid content and extension
/// # Arguments
/// * `file_path` - The file path to check
/// * `verify_extension` - True if the extension must be verified, false otherwise
/// # Returns
pub fn validate_file(file_path: &str, verify_extension: bool) -> Result<bool, &str> {
    let buffer = read_from_path(file_path)?;

    // Verify the group of the file type
    if !is_video(&buffer) && !is_image(&buffer) {
        return Err(INVALID_FILE_GROUP);
    }

    // Verify the extension of the file if asked
    let file_type_buffer = get(&buffer);
    match file_type_buffer {
        Some(file_type) => {
            Ok(!verify_extension
                || match_extension(file_path, &file_type)
                || is_special_extension(file_path, &file_type))
        },
        None => Err(INVALID_FILE_TYPE),
    }
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    //https://file-examples.com/
}
