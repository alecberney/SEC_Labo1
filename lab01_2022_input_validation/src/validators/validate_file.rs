use std::fmt::format;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

static REGEX_FILE_PATH: &str = r"[a-zA-Z0-9\/\.\\_-]+";

static ERROR_MESSAGE_FILE_PATH: &str = "File path given is invalid";

/// Check if the given file path is valid
/// # Arguments
/// * `file_path` - The file_path to check
/// # Returns
/// * `bool` - True if the file_path is valid, false otherwise
pub fn is_valid_file_path(file_path: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!("^{}$", REGEX_TOP_LEVEL_DOMAIN)).unwrap();
    }
    RE.is_match(file_path)
}

///
pub fn validate_file(file_path: &str) -> Result<bool, String> {
    // TODO: validation d'input

    if !is_valid_file_path(file_path) {
        return Err(format!("{}", ERROR_MESSAGE_FILE_PATH));
    }

    // https://docs.rs/infer/0.7.0/infer/index.html

    //fs::read()
    /*let kind = infer::get(buffer).expect("file type is known");

    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(kind.extension(), "jpg");
    assert_eq!(kind.matcher_type(), infer::MatcherType::Image);*/

    // Read file
    let buffer = match &fs::read(file_path) {
        Ok(buffer) => buffer,
        Err(e) => {
            return Err(format!("{}", e));
        }
    };

    // Verify file type
    Ok(infer::is_image(&buffer) || infer::is_video(&buffer))
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
