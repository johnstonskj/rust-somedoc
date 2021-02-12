use proptest::prelude::*;
use somedoc::model::inline::{Character, Emoji};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_emoji_reject_empty() {
    let result = Emoji::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_emoji_reject_unbalanced_colons() {
    let result = Emoji::from_str(":hello");
    println!("{:?}", result);
    assert!(result.is_err());

    let result = Emoji::from_str("hello:");
    assert!(result.is_err());
}

#[test]
fn test_emoji_with_colons() {
    let result = Emoji::from_str(":hello:");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().into_inner(), String::from(":hello:"))
}

#[test]
fn test_emoji_without_colons() {
    let result = Emoji::from_str("hello");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().into_inner(), String::from(":hello:"))
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn ptest_emoji_doesnt_crash(s in "\\PC*") {
        let _ = Emoji::from_str(&s);
    }

    #[test]
    fn ptest_emoji_valid_values(s in r"(:[a-zA-Z0-9_\-]+:)|([a-zA-Z0-9_\-]+)") {
        println!("valid_values {:?}", s);
        assert!(Emoji::from_str(&s).is_ok());
    }

    #[test]
    fn ptest_char_any(c in proptest::char::any()) {
        println!("valid_values {:?}", c);
        let _ = Character::from(c);
    }
}
