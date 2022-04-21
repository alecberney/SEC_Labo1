use lazy_static::lazy_static;
use regex::Regex;

use crate::validators::error_messages::{INVALID_WHITELIST_TOP_LEVEL_DOMAIN};

static REGEX_PROTOCOL_NAME: &str = r"[[:alnum:]]+://";
static REGEX_SUB_DOMAIN: &str = r"[a-zA-Z\d\.-]+";
static REGEX_TOP_LEVEL_DOMAIN: &str = r"\.[a-zA-Z\.]{1,}[[:alpha:]]";
static REGEX_FOLLOWING_URL: &str = r"([/#].*)?";

/// Check if the given top level domain is valid
/// # Arguments
/// * `top_level_domain` - The top level domain to check
/// # Returns
/// * `bool` - True if the top level domain is valid, false otherwise
pub fn is_valid_top_level_domain(top_level_domain: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!("^{}$", REGEX_TOP_LEVEL_DOMAIN)).unwrap();
    }
    RE.is_match(top_level_domain)
}

/// Create regex for top level domains from a given whitelist
/// # Arguments
/// * `top_level_domains_whitelist` - a list of top level domains
/// # Return
/// * `String` - a regex for top level domains or a str containing an error message
/// if the white list contains an incompatible top level domain
/// # Errors
/// * `&str` - an error message
fn create_whitelist_regex<'a>(top_level_domains_whitelist: Option<&Vec<&str>>) -> Result<String, &'a str> {
    let mut top_level_domains: String;

    match top_level_domains_whitelist {
        Some(domains) => {
            top_level_domains = String::new();
            for &domain in domains {
                if !is_valid_top_level_domain(domain) {
                    return Err(INVALID_WHITELIST_TOP_LEVEL_DOMAIN);
                }
                top_level_domains.push_str(domain);
                top_level_domains.push_str("|");
            }
            top_level_domains.pop();
        }
        None => {
            top_level_domains = String::from(REGEX_TOP_LEVEL_DOMAIN);
        }
    }
    Ok(top_level_domains)
}

/// Create a regex string from a given url
/// # Arguments
/// * `url` - The url to create the regex from
/// * `top_level_domains_whitelist` - a list of top level domains
/// # Return
/// * `String` - The regex string
/// # Errors
/// * `&str` - An error message if the url is not valid
fn create_url_regex_string<'a>(top_level_domains_whitelist: Option<&Vec<&str>>) -> Result<String, &'a str> {
    Ok(format!(r"^({})?({})({})({})$",
            REGEX_PROTOCOL_NAME,
            REGEX_SUB_DOMAIN,
            create_whitelist_regex(top_level_domains_whitelist)?,
            REGEX_FOLLOWING_URL
    ))
}

/// Validate a given url
/// # Arguments
/// * `url` - The url to validate
/// * `top_level_domains_whitelist` - a list of top level domains
/// # Return
/// * `bool` - True if the url is valid, false otherwise
/// # Errors
/// * `&str` - An error message if the url is not valid
pub fn validate_url<'a>(url_input: &'a str, top_level_domains_whitelist: Option<&Vec<&str>>) -> Result<bool, &'a str> {
    Ok(Regex::new(
            &create_url_regex_string(top_level_domains_whitelist)?
        ).unwrap().is_match(&url_input))
}

#[cfg(test)]
mod tests {
    use crate::validators::validate_url::{is_valid_top_level_domain, validate_url};
    use crate::validators::error_messages::{INVALID_WHITELIST_TOP_LEVEL_DOMAIN};
    use crate::validators::test_helper::{result_helper};

    #[test]
    fn is_valid_top_level_domain_classical() {
        // Pass
        assert!(is_valid_top_level_domain(".com"));
        assert!(is_valid_top_level_domain(".ch"));
        assert!(is_valid_top_level_domain(".de"));
        assert!(is_valid_top_level_domain(".org"));
        assert!(is_valid_top_level_domain(".net"));

        assert!(is_valid_top_level_domain(".COM"));
        assert!(is_valid_top_level_domain(".CH"));
        assert!(is_valid_top_level_domain(".DE"));
        assert!(is_valid_top_level_domain(".ORG"));
        assert!(is_valid_top_level_domain(".NET"));

        // Corner cases
        assert!(is_valid_top_level_domain(".co.uk"));
        assert!(is_valid_top_level_domain(".CO.UK"));
    }

    #[test]
    fn is_valid_top_level_domain_length() {
        // Pass
        assert!(is_valid_top_level_domain(".aaaa"));
        assert!(is_valid_top_level_domain(".aaaaaaaaaaaaaa"));

        // Fail
        assert!(!is_valid_top_level_domain("."));
        assert!(!is_valid_top_level_domain("a"));

        // Corner cases
        assert!(!is_valid_top_level_domain(".a"));
        assert!(is_valid_top_level_domain(".aa"));
        assert!(is_valid_top_level_domain(".aaa"));
    }

    #[test]
    fn is_valid_top_level_domain_starting_full_stop() {
        // Corner cases
        // We test different cases where top level domain starting with an full stop
        // or something else
        assert!(!is_valid_top_level_domain("aaa"));
        assert!(!is_valid_top_level_domain("a.a"));
        assert!(is_valid_top_level_domain("..a"));
        assert!(!is_valid_top_level_domain("&.a"));
        assert!(!is_valid_top_level_domain("$.a"));
        assert!(!is_valid_top_level_domain("/.a"));
    }

    #[test]
    fn is_valid_top_level_domain_finishing_ascii_letter() {
        // Corner cases
        // We test different cases where top level domain finish with an ascii letter
        // or something else
        assert!(!is_valid_top_level_domain(".a."));
        assert!(is_valid_top_level_domain("...a"));
        assert!(is_valid_top_level_domain("...A"));
        assert!(!is_valid_top_level_domain("..."));
        assert!(!is_valid_top_level_domain("..$"));
        assert!(!is_valid_top_level_domain("../"));
        assert!(!is_valid_top_level_domain("..&"));
    }

    #[test]
    fn validate_url_protocol_name_classical() {
        // Pass
        result_helper(validate_url("http://example.com/", None), true, None);
        result_helper(validate_url("https://example.com/", None), true, None);
        result_helper(validate_url("ftp://example.com/", None), true, None);
    }

    #[test]
    fn validate_url_protocol_name_alpha_num() {
        // Pass
        result_helper(validate_url("aaaa://example.com/", None), true, None);
        result_helper(validate_url("AAAA://example.com/", None), true, None);
        result_helper(validate_url("1111://example.com/", None), true, None);
        result_helper(validate_url("a1a1://example.com/", None), true, None);
        result_helper(validate_url("A1A1://example.com/", None), true, None);

        // Fail & Corner cases
        result_helper(validate_url("aa$a://example.com/", None), false, None);
        result_helper(validate_url("aa{a://example.com/", None), false, None);
        result_helper(validate_url("aa%a://example.com/", None), false, None);
        result_helper(validate_url("aa@a://example.com/", None), false, None);
        result_helper(validate_url("aa+a://example.com/", None), false, None);
        result_helper(validate_url("aa.a://example.com/", None), false, None);
        result_helper(validate_url("aa-a://example.com/", None), false, None);
    }

    #[test]
    fn validate_url_protocol_name_end() {
        // Pass
        result_helper(validate_url("aaaa://example.com/", None), true, None);

        // Fail
        result_helper(validate_url("aa$a:////example.com/", None), false, None);
        result_helper(validate_url("aa{a::://example.com/", None), false, None);
        result_helper(validate_url("aa%a$$$example.com/", None), false, None);
        result_helper(validate_url("aa@a&&&example.com/", None), false, None);
        // Corner cases
        result_helper(validate_url("aaaa:/example.com/", None), false, None);
        result_helper(validate_url("aaaa:///example.com/", None), false, None);

        result_helper(validate_url("aaaa:://example.com/", None), false, None);
        result_helper(validate_url("aaaa//example.com/", None), false, None);

        result_helper(validate_url("aaaa&//example.com/", None), false, None);
        result_helper(validate_url("aaaa$//example.com/", None), false, None);

        result_helper(validate_url("aaaa:/&example.com/", None), false, None);
        result_helper(validate_url("aaaa:&/example.com/", None), false, None);
    }

    #[test]
    fn validate_url_sub_domain_all() {
        // Pass
        result_helper(validate_url("http://aaaa.com/", None), true, None);
        result_helper(validate_url("http://AAAA.com/", None), true, None);
        result_helper(validate_url("http://1111.com/", None), true, None);
        result_helper(validate_url("http://a1a1.com/", None), true, None);
        result_helper(validate_url("http://A1A1.com/", None), true, None);
        result_helper(validate_url("http://.....com/", None), true, None);
        result_helper(validate_url("http://----.com/", None), true, None);
        result_helper(validate_url("http://a-1.a.com/", None), true, None);

        // Fail & Corner cases
        result_helper(validate_url("http://aa$a.com/", None), false, None);
        result_helper(validate_url("http://aa{a.com/", None), false, None);
        result_helper(validate_url("http://aaa%.com/", None), false, None);
        result_helper(validate_url("http://@aaa.com/", None), false, None);
        result_helper(validate_url("http://a+aa.com/", None), false, None);
    }

    #[test]
    fn validate_url_top_level_domain_classical() {
        // Pass
        result_helper(validate_url("http://example.com/", None), true, None);
        result_helper(validate_url("http://example.ch/", None), true, None);
        result_helper(validate_url("http://example.de/", None), true, None);
        result_helper(validate_url("http://example.org/", None), true, None);
        result_helper(validate_url("http://example.net/", None), true, None);

        result_helper(validate_url("http://example.COM/", None), true, None);
        result_helper(validate_url("http://example.CH/", None), true, None);
        result_helper(validate_url("http://example.DE/", None), true, None);
        result_helper(validate_url("http://example.ORG/", None), true, None);
        result_helper(validate_url("http://example.NET/", None), true, None);

        // Corner cases
        result_helper(validate_url("http://example.co.uk/", None), true, None);
        result_helper(validate_url("http://example.CO.UK/", None), true, None);
    }

    #[test]
    fn validate_url_top_level_domain_length() {
        // Pass
        result_helper(validate_url("http://example.aaaa/", None), true, None);
        result_helper(validate_url("http://example.aaaaaaaaaaaaaa/", None), true, None);

        // Fail
        result_helper(validate_url("http://example./", None), false, None);
        result_helper(validate_url("http://examplea/", None), false, None);

        // Corner cases
        result_helper(validate_url("http://example.a/", None), false, None);
        result_helper(validate_url("http://example.aa/", None), true, None);
        result_helper(validate_url("http://example.aaa/", None), true, None);
    }

    #[test]
    fn validate_url_top_level_domain_starting_full_stop() {
        // Corner cases
        result_helper(validate_url("http://exampleaaa/", None), false, None);
        result_helper(validate_url("http://examplea.a/", None), false, None);
        result_helper(validate_url("http://example..a/", None), true, None);

        result_helper(validate_url("http://example&.a/", None), false, None);
        result_helper(validate_url("http://example$.a/", None), false, None);
        result_helper(validate_url("http://example/.a/", None), false, None);
    }

    #[test]
    fn validate_url_top_level_domain_starting_finishing_ascii_letter() {
        // Corner cases
        result_helper(validate_url("http://example.a./", None), false, None);
        result_helper(validate_url("http://example...a/", None), true, None);
        result_helper(validate_url("http://example...A/", None), true, None);
        result_helper(validate_url("http://example.../", None), false, None);

        result_helper(validate_url("http://example..$/", None), false, None);
        result_helper(validate_url("http://example..//", None), false, None);
        result_helper(validate_url("http://example..&/", None), false, None);
    }

    // TODO
    #[test]
    fn validate_url_top_level_domain_whitelist() {
        let top_level_domain_accepted = vec![".com", ".ch", ".de"];
        let top_level_domain_accepted_fail = vec![".com", "...", ".de"];

        // Pass
        result_helper(validate_url("https://example.com/", Some(&top_level_domain_accepted)), true, None);
        result_helper(validate_url("https://example.ch/", Some(&top_level_domain_accepted)), true, None);
        result_helper(validate_url("https://example.de/", Some(&top_level_domain_accepted)), true, None);

        // Fail
        result_helper(validate_url("https://example.net/", Some(&top_level_domain_accepted)), false, None);
        result_helper(validate_url("https://example.swiss/", Some(&top_level_domain_accepted)), false, None);

        // Corner cases
        result_helper(validate_url("https://example.ch/", Some(&top_level_domain_accepted_fail)), false, Some(INVALID_WHITELIST_TOP_LEVEL_DOMAIN));
        result_helper(validate_url("https://example.swiss/", None), true, None);
    }

    // TODO
    #[test]
    fn validate_url_following_part() {
        // Pass
        result_helper(validate_url("http://example.com/aaaa", None), true, None);
        result_helper(validate_url("http://example.com#aaaa", None), true, None);

        // Fail
        result_helper(validate_url("http://example.com%aaaa", None), false, None);
        result_helper(validate_url("http://example.com?aaaa", None), false, None);

        // Corner cases
        result_helper(validate_url("http://example.com//aaaa", None), true, None);
        result_helper(validate_url("http://example.com##aaaa", None), true, None);
        result_helper(validate_url("http://example.comaaaa", None), true, None);
    }

    #[test]
    fn validate_url_special_case() {
        // Corner cases
        result_helper(validate_url("https://example.com/", None), true, None);
        result_helper(validate_url("..google", None), true, None);
    }
}
