use uuid::Uuid;
use lazy_static::lazy_static;
use regex::Regex;

use crate::validators::error_messages::{INVALID_UUID};

// DO NOT READ FILE CONTENTS INSIDE THIS FUNCTION
// TODO : specify parameter type(s) and return type(s)

// https://fr.wikipedia.org/wiki/Universally_unique_identifier
static REGEX_UUID: &str = r"[[:alnum:]]{8,}\-([[:alnum:]]{4,}\-){3,}[[:alnum:]]{12,}";

pub fn validate_uuid(uuid: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!("^{}$", REGEX_UUID)).unwrap();
    }
    RE.is_match(uuid)
}

pub fn validate_file_uuid<'a>(file_path: &'a str, provided_uuid: &'a str) -> Result<bool, &'a str> {
    if !validate_uuid(provided_uuid) {
        return Err(INVALID_UUID);
    }

    //https://docs.rs/uuid/0.8.1/uuid/struct.Uuid.html
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
