use uuid::Uuid;
use lazy_static::lazy_static;
use regex::Regex;

use crate::validators::error_messages::{INVALID_UUID};
use crate::validators::file_helper::{read_from_path};

// https://fr.wikipedia.org/wiki/Universally_unique_identifier
static REGEX_UUID: &str = r"[[:xdigit:]]{8}\-([[:xdigit:]]{4}\-){3}[[:xdigit:]]{12}";

/// Validate if the given string is a valid UUID
/// # Arguments
/// * `uuid` - The uuid string in hyphenated format to validate
/// # Returns
/// * `bool` - True if the uuid is valid, false otherwise
pub fn validate_uuid(uuid: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!("^{}$", REGEX_UUID)).unwrap();
    }
    RE.is_match(uuid)
}

/// Validate UUID from file
/// # Arguments
/// * `file_path` - path to file
/// * `provided_uuid` - uuid in hyphenated format that the file must contain
/// # Returns
/// * `Result<bool, &str>` - True if the uuid is valid, false otherwise
/// # Errors
/// * `&str` - Error message
pub fn validate_file_uuid<'a>(file_path: &'a str, provided_uuid: &'a str) -> Result<bool, &'a str> {
    if !validate_uuid(provided_uuid) {
        return Err(INVALID_UUID);
    }

    let file_buffer = read_from_path(file_path)?;
    let uuid = Uuid::new_v5(&Uuid::default(), &file_buffer).to_hyphenated().to_string();

    Ok(uuid == provided_uuid)
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::validators::validate_uuid::{validate_uuid, validate_file_uuid};
    use crate::validators::test_helper::{result_helper};
    use crate::validators::file_helper::{read_from_path};
    use crate::validators::error_messages::{INVALID_UUID, ERROR_READING_FILE};

    // Tests has been written with file example found here:
    // https://file-examples.com/
    // Files are on my github but not delivered for the scholar rendering
    static BASE_FILE_PATH : &str = "A:/HEIG/Semestre 6/SEC/Labos/SEC_Labo1/files/";
    static IMAGES_FOLDER : &str = "images/";
    static NAMING_CONVENTION : &str = "file_example_";

    // We assume that the file is existing and is readable
    fn generate_uuid(file_path: &str) -> String {
        let buffer= read_from_path(file_path).unwrap();
        uuid::Uuid::new_v5(&Uuid::default(), &buffer)
            .to_hyphenated().to_string()
    }

    #[test]
    fn validate_uuid_format() {
        // Pass
        assert!(validate_uuid("00000008-0004-0004-0004-000000000012"));

        // Fail
        assert!(!validate_uuid("000000000000000000000000000"));
        assert!(!validate_uuid("000000-000000-0000000000"));
        assert!(!validate_uuid("000000-000000-000000-0000"));
        assert!(!validate_uuid("--------------------------"));

        // Corner cases
        // the number at the end of the part is the number of characters from this one
        // we test higher and lower length from each part
        assert!(!validate_uuid("0000007-0004-0004-0004-000000000012"));
        assert!(!validate_uuid("000000009-0004-0004-0004-000000000012"));

        assert!(!validate_uuid("00000008-003-0004-0004-000000000012"));
        assert!(!validate_uuid("00000008-0004-003-0004-000000000012"));
        assert!(!validate_uuid("00000008-0004-0004-003-000000000012"));
        assert!(!validate_uuid("00000008-00005-0004-0004-000000000012"));
        assert!(!validate_uuid("00000008-0004-00005-0004-000000000012"));
        assert!(!validate_uuid("00000008-0004-0004-00005-000000000012"));

        assert!(!validate_uuid("00000008-0004-0004-003-00000000011"));
        assert!(!validate_uuid("00000008-0004-0004-003-0000000000013"));

        // we test the possibility of having 2 hyphens following each other
        assert!(!validate_uuid("00000008--0004-0004-0004-000000000012"));
        assert!(!validate_uuid("00000008-0004--0004-0004-000000000012"));
        assert!(!validate_uuid("00000008-0004-0004--0004-000000000012"));
        assert!(!validate_uuid("00000008-0004-0004-0004--000000000012"));

        // we test other characters than hyphens
        assert!(!validate_uuid("00000008 0004 0004 0004 000000000012"));
        assert!(!validate_uuid("00000008_0004_0004_0004_000000000012"));
        assert!(!validate_uuid("00000008000400040004000000000012"));
    }

    #[test]
    fn validate_uuid_characters() {
        // Pass
        assert!(validate_uuid("00000000-0000-0000-0000-000000000000"));
        assert!(validate_uuid("12345678-1234-4567-8912-123456789012"));
        assert!(validate_uuid("abcdefab-abcd-abcd-abcd-abcdefabcdef"));
        assert!(validate_uuid("ABCDEFAB-ABCD-ABCD-ABCD-ABCDEFABCDEF"));

        // Fail & Corner cases

        // we test special characters
        assert!(!validate_uuid("*$[_0000-0000-0000-0000-000000000000"));
        assert!(!validate_uuid("00000000-*$[_-0000-0000-000000000000"));
        assert!(!validate_uuid("00000000-0000-*$[_-0000-000000000000"));
        assert!(!validate_uuid("00000000-0000-0000-*$[_-000000000000"));
        assert!(!validate_uuid("00000000-0000-0000-0000-*$[_00000000"));
        assert!(!validate_uuid("+@*%&/()-=0?'-^`~]-[}{!-¨_<>°§:;.,00"));

        // we test non hex characters but alpha-numeric
        assert!(!validate_uuid("z0000000-0000-0000-0000-000000000000"));
        assert!(!validate_uuid("00000000-u000-0000-0000-000000000000"));
        assert!(!validate_uuid("00000000-0000-v000-0000-000000000000"));
        assert!(!validate_uuid("00000000-0000-0000-w000-000000000000"));
        assert!(!validate_uuid("00000000-0000-0000-0000-x00000000000"));
        assert!(!validate_uuid("z0000000-0000-0000-0000-000000000000"));

        assert!(!validate_uuid("Y0000000-0000-0000-0000-000000000000"));
        assert!(!validate_uuid("00000000-Z000-0000-0000-000000000000"));
        assert!(!validate_uuid("00000000-0000-U000-0000-000000000000"));
        assert!(!validate_uuid("00000000-0000-0000-V000-000000000000"));
        assert!(!validate_uuid("00000000-0000-0000-0000-W00000000000"));

        assert!(!validate_uuid("abcdefgh-ijkl-mnop-qrst-uvwxyz012345"));
        assert!(!validate_uuid("ABCDEFGH-IJKL-MNOP-QRST-UVWXYZ012345"));
    }

    // We won't test all uuid formats because in these tests series cause we did it
    // before in the function used in this one
    #[test]
    fn validate_file_uuid_classical() {
        // Pass
        let uuid = generate_uuid(&format!("{}{}{}{}", BASE_FILE_PATH,
                                          IMAGES_FOLDER, NAMING_CONVENTION, "png.png"));
        result_helper(validate_file_uuid(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "png.png"),
            &uuid), true, None);

        // Fail
        result_helper(validate_file_uuid(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "png.png"),
            "00000008-0004-0004-0004-000000000012"),
                      false, None);
    }

    #[test]
    fn validate_file_uuid_bad_uuid() {
        // Corner Case & Fail
        result_helper(validate_file_uuid(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "png.png"),
            "00000008_0004-0004-0004-000000000012"),
                      false, Some(INVALID_UUID));
    }

    #[test]
    fn validate_file_uuid_no_file_found() {
        // Corner Cases & Fail
        result_helper(validate_file_uuid(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "test.test"),
            "00000008-0004-0004-0004-000000000012"),
                      false, Some(ERROR_READING_FILE));
    }
}
