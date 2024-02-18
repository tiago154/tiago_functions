use regex::Regex;

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn is_valid_email(email: &str) -> bool {
    Regex::new(r"^[\w-]+(\.[\w-]+)*@[\w-]+(\.[\w-]+)+$")
        .unwrap()
        .is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string(""), "");
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("user123@example.co.uk"));
        assert!(!is_valid_email("invalidemail"));
        assert!(!is_valid_email("user@example"));
        assert!(!is_valid_email("@example.com"));
    }
}
