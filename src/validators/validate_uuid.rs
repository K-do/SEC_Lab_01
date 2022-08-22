use lazy_static::lazy_static;
use regex::Regex;
use uuid::Uuid;

/// Validate a version-5 uuid [variant-1](https://en.wikipedia.org/wiki/Universally_unique_identifier#Variants)
///
/// # Examples
/// ``` ignore
/// let mut result = validate_uuid("b267fe9e-6e37-5bed-a2c5-e44943802a91");
/// assert!(result);
/// ```
pub fn validate_uuid(uuid: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[5][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap();
    }
    REGEX.is_match(uuid)
}

/// Check that a version-5 uuid corresponds to a file.
///
/// # Examples
/// ``` ignore
/// let namespace = Uuid::NAMESPACE_OID;
/// let uuid = Uuid::new_v5(&namespace, "my_content".as_bytes());
/// let mut result = check_file_uuid(&namespace, "my_content".as_bytes(), &uuid);
/// assert!(result);
/// ```
pub fn validate_file_uuid(namespace: &Uuid, file: &[u8], uuid: &Uuid) -> bool {
    Uuid::new_v5(namespace, file) == *uuid
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::{validate_file_uuid, validate_uuid};

    const FILE_CONTENT: &[u8] = "laCryptoCRigolo".as_bytes();

    #[test]
    fn valid_uuids() {
        assert!(validate_uuid("c70dc454-1c7d-5c59-8fed-3a321e6a4a49"));

        // uuids should not be case sensitive
        assert!(validate_uuid("C70DC454-1C7D-5C59-8fED-3A321E6A4A49"));
    }

    #[test]
    fn invalid_uuids() {
        assert!(!validate_uuid(""));

        // missing a char
        assert!(!validate_uuid("70dc454-1c7d-5c59-8fed-3a321e6a4a49"));
        assert!(!validate_uuid("c70dc454-c7d-5c59-8fed-3a321e6a4a49"));
        assert!(!validate_uuid("c70dc454-1c7d-5c5-8fed-3a321e6a4a49"));
        assert!(!validate_uuid("c70dc454-1c7d-5c59-8fd-3a321e6a4a49"));
        assert!(!validate_uuid("c70dc454-1c7d-5c59-8fed-3a321e6a4a4"));

        // invalid versions
        assert!(!validate_uuid("c70dc454-1c7d-1c59-8fed-3a321e6a4a49")); // 1
        assert!(!validate_uuid("c70dc454-1c7d-2c59-8fed-3a321e6a4a49")); // 2
        assert!(!validate_uuid("c70dc454-1c7d-3c59-8fed-3a321e6a4a49")); // 3
        assert!(!validate_uuid("c70dc454-1c7d-4c59-8fed-3a321e6a4a49")); // 4

        // invalid variants
        assert!(!validate_uuid("c70dc454-1c7d-5c59-0fed-3a321e6a4a49")); // 0
        assert!(!validate_uuid("c70dc454-1c7d-5c59-cfed-3a321e6a4a49")); // 2
        assert!(!validate_uuid("c70dc454-1c7d-5c59-ffed-3a321e6a4a49")); // 3

        // invalid chars
        assert!(!validate_uuid("c70xz454-1c7d-5c59-8fed-3a321e6a4a49"));
        assert!(!validate_uuid("c70dc454.1c7d.5c59.8fed.3a321e6a4a49"));
    }

    #[test]
    fn valid_uuid_to_file() {
        assert!(validate_file_uuid(&Uuid::NAMESPACE_OID, FILE_CONTENT,
                                   &Uuid::new_v5(&Uuid::NAMESPACE_OID, FILE_CONTENT)));

        let custom_namespace = Uuid::parse_str("c7bb890c-a4a8-4d68-85b7-1e1cfe909249").unwrap();
        assert!(validate_file_uuid(&custom_namespace, FILE_CONTENT,
                                   &Uuid::new_v5(&custom_namespace, FILE_CONTENT)));
    }

    #[test]
    fn invalid_uuid_to_file() {
        // file content doesn't match uuid
        assert!(!validate_file_uuid(&Uuid::NAMESPACE_OID, "laCryptoCPasRigolo".as_bytes(),
                                    &Uuid::new_v5(&Uuid::NAMESPACE_OID, FILE_CONTENT)));

        // namespaces are not the same
        assert!(!validate_file_uuid(&Uuid::NAMESPACE_OID, FILE_CONTENT,
                                    &Uuid::new_v5(&Uuid::NAMESPACE_DNS, FILE_CONTENT)));
    }
}
