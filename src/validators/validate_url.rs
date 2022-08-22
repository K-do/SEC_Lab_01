use lazy_static::lazy_static;
use regex::Regex;

const PROTOTYPE_SUB_LEVEL_PATTERN: &str = r"^([[:alnum:]]+://)?([[:alnum:].-]+)";
const TOP_LEVEL_PATTERN: &str = r"(\.[[:alpha:].]{1,}[[:alpha:]])";
const END_PATTERN: &str = r"([/#].*)?$";

/// Validate an url providing an optional top level whitelist.
///
/// If a whitelist is passed as argument, the top level domains within are validated before
/// checking the url. The whitelist can't be empty and the top level domains must match
/// the rules specified in the lab. The top level domains inside the whitelist are case sensitive.
///
/// # Errors
/// If the whitelist is empty or at least one top level domain inside is invalid, an error will
/// be returned.
///
/// # Examples
/// ``` ignore
/// let mut result = validate_url("https://docs.rs/lazy_static", None);
/// assert!(result);
///
/// result = validate_url("en.wikipedia.org/wiki/Breast_cancer", Some(&vec![".ch", ".com"]));
/// assert!(!result);
/// ```
pub fn validate_url(url: &str, top_level_whitelist: Option<&Vec<&str>>) -> Result<bool, String> {
    match top_level_whitelist {
        None => {
            lazy_static! {
                static ref REGEX:Regex = Regex::new(&format!("{}{}{}",
                    PROTOTYPE_SUB_LEVEL_PATTERN, TOP_LEVEL_PATTERN, END_PATTERN)).unwrap();
            }
            Ok(REGEX.is_match(url))
        }

        Some(whitelist) => {
            if whitelist.is_empty() {
                return Err(String::from("The white list is empty."));
            }

            lazy_static! {
                static ref TOP_LEVEL_REGEX:Regex = Regex::new(&format!("^{}$", TOP_LEVEL_PATTERN)).unwrap();
            }

            // Check the top level domains in the whitelist and extract them if valid
            let mut top_level_list = String::from("(");
            for (index, &tld) in whitelist.iter().enumerate() {
                if !TOP_LEVEL_REGEX.is_match(tld) {
                    return Err(String::from("Invalid top level domain in white list."));
                }

                top_level_list.push_str(&format!(r"\{}", tld));

                if index != (whitelist.len() - 1) {
                    top_level_list.push('|');
                }
            }
            top_level_list.push(')');

            let regex = Regex::new(
                &format!("{}{}{}", PROTOTYPE_SUB_LEVEL_PATTERN, &top_level_list, END_PATTERN))
                .unwrap();

            Ok(regex.is_match(url))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::validate_url;

    #[test]
    fn valid_whitelists() {
        assert!(validate_url("", Some(&vec![".com"])).is_ok());

        // uppercase allowed
        assert!(validate_url("", Some(&vec![".COM"])).is_ok());

        // at least 3 chars
        assert!(validate_url("", Some(&vec![".ch"])).is_ok());

        // multiple top level domains allowed
        assert!(validate_url("", Some(&vec![".ch.com"])).is_ok());

        // multiple full stops in top level domain and multiple top level domains allowed
        assert!(validate_url("", Some(&vec!["..a", ".a.b"])).is_ok());
    }

    #[test]
    fn invalid_whitelists() {
        // at least 3 chars
        assert!(validate_url("", Some(&vec!["."])).is_err());
        assert!(validate_url("", Some(&vec![".a"])).is_err());

        // must end by an ascii char
        assert!(validate_url("", Some(&vec!["ch."])).is_err());

        // top level domain can't be empty
        assert!(validate_url("", Some(&vec![".com", ""])).is_err());

        // only ascii letters
        assert!(validate_url("", Some(&vec![".1p"])).is_err());
        assert!(validate_url("", Some(&vec![".漢字"])).is_err());

        // white list can't be empty
        assert!(validate_url("", Some(&vec![])).is_err());
    }

    #[test]
    fn valid_protocols() {
        assert!(validate_url("https://test.com", None).unwrap());

        // should not be case sensitive
        assert!(validate_url("hTTpS://test.com", None).unwrap());

        // only numbers and/or ascii letters before the ://
        assert!(validate_url("1234://test.com", None).unwrap());
        assert!(validate_url("1p://test.com", None).unwrap());
        assert!(validate_url("1://test.com", None).unwrap());

        // no protocol allowed
        assert!(validate_url("test.com", None).unwrap());
    }

    #[test]
    fn invalid_protocols() {
        // must have ://
        assert!(!validate_url("http:/test.com", None).unwrap());
        assert!(!validate_url("http:://test.com", None).unwrap());

        // must have at least one ascii letter or number before the ://
        assert!(!validate_url("://test.com", None).unwrap());
        assert!(!validate_url(" ://test.com", None).unwrap());

        // only ascii letters and numbers allowed
        assert!(!validate_url("p_1://test.com", None).unwrap());
        assert!(!validate_url("漢字://test.com", None).unwrap());
    }

    #[test]
    fn valid_sub_level_domains() {
        assert!(validate_url("sub.com", None).unwrap());

        // should not be case sensitive
        assert!(validate_url("SUB.com", None).unwrap());

        // only ascii letters, numbers, full stops and hyphens allowed
        assert!(validate_url("..com", None).unwrap());
        assert!(validate_url("-.com", None).unwrap());
        assert!(validate_url("3.com", None).unwrap());
        assert!(validate_url("www.3-b..com", None).unwrap());
    }

    #[test]
    fn invalid_sub_level_domains() {
        // can't be empty
        assert!(!validate_url(".com", None).unwrap());
        assert!(!validate_url("https://.com", None).unwrap());

        // only ascii letters, numbers, full stops and hyphens allowed
        assert!(!validate_url(" .com", None).unwrap());
        assert!(!validate_url("a_b.com", None).unwrap());
        assert!(!validate_url("漢字.com", None).unwrap());
    }

    #[test]
    fn valid_top_level_domains() {
        assert!(validate_url("test.com", None).unwrap());

        // should not be case sensitive
        assert!(validate_url("test.COM", None).unwrap());

        // at least 3 chars
        assert!(validate_url("test.ch", None).unwrap());

        // multiple top level domains and full stops allowed
        assert!(validate_url("test.ch.com", None).unwrap());
        assert!(validate_url("test..a", None).unwrap());
    }

    #[test]
    fn invalid_top_level_domains() {
        // can't be empty
        assert!(!validate_url("test", None).unwrap());

        // at least 3 chars
        assert!(!validate_url("test.", None).unwrap());
        assert!(!validate_url("test.a", None).unwrap());

        // must end by an ascii letter
        assert!(!validate_url("test.c.", None).unwrap());

        // only ascii letters and full stops allowed
        assert!(!validate_url("test.1p", None).unwrap());
        assert!(!validate_url("test.c-h", None).unwrap());
        assert!(!validate_url("test.漢字", None).unwrap());
    }

    #[test]
    fn valid_top_level_domains_with_whitelist() {
        assert!(validate_url("test.com", Some(&vec![".com"])).unwrap());
        assert!(validate_url("test.COM", Some(&vec![".COM"])).unwrap());
        assert!(validate_url("test.ch", Some(&vec![".ch"])).unwrap());
        assert!(validate_url("test.ch.com", Some(&vec![".com"])).unwrap());
        assert!(validate_url("test.ch.com", Some(&vec![".ch.com"])).unwrap());
        assert!(validate_url("test..a", Some(&vec!["..a"])).unwrap());
    }

    #[test]
    fn invalid_top_level_domains_with_whitelist() {
        assert!(!validate_url("test.com", Some(&vec![".ch"])).unwrap());

        // whitelist is case sensitive
        assert!(!validate_url("test.COM", Some(&vec![".com"])).unwrap());
        assert!(!validate_url("test.ch", Some(&vec![".CH"])).unwrap());

        // only the most top level domain is considered
        assert!(!validate_url("test.ch.com", Some(&vec![".ch"])).unwrap());
    }

    #[test]
    fn valid_end_url() {
        // must start with / or #
        assert!(validate_url("test.com/", None).unwrap());
        assert!(validate_url("test.com#", None).unwrap());

        // can be anything after / or #
        assert!(validate_url("test.com/A#2.漢/", None).unwrap());
        assert!(validate_url("test.com#A#2.漢/", None).unwrap());
    }

    #[test]
    fn invalid_end_url() {
        // must start with / or #
        assert!(!validate_url("test.com?", None).unwrap());
        assert!(!validate_url("test.com:", None).unwrap());
        assert!(!validate_url("test.com:/", None).unwrap());
        assert!(!validate_url("test.com:#", None).unwrap());
    }
}
