/// Function that assert a Result to compare if it was the good one (error or value)
/// # Arguments
/// * `result` - the result to assert
/// * `expected_value` - the expected value
/// * `expected_error` - the expected error
#[allow(dead_code)]
pub fn result_helper(result: Result<bool, &str>, expected_value: bool, expected_error: Option<&str>) {
    match result {
        Ok(result) => assert_eq!(result, expected_value),
        Err(error) =>
            match expected_error {
                Some(message) => assert_eq!(error, message),
                None => assert!(false)
            }
    }
}