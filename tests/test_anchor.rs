use proptest::prelude::*;
use somedoc::model::inline::{Anchor, HyperLinkTarget};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_reject_empty() {
    let result = Anchor::new("");
    assert!(result.is_err());

    let result = Anchor::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_new() {
    let result = Anchor::new("test_new");
    assert!(result.is_ok());
    println!("{:?}", result);
    assert_eq!(result.unwrap().inner(), &String::from("test_new"));
}

#[test]
fn test_into_inner() {
    let result = Anchor::new("test_into_inner");
    assert!(result.is_ok());
    println!("{:?}", result);
    assert_eq!(
        result.unwrap().into_inner(),
        String::from("test_into_inner")
    );
}

#[test]
fn test_to_ref() {
    let result = Anchor::new("test_to_ref");
    assert!(result.is_ok());
    println!("{:?}", result);
    let anchor = result.unwrap();

    let link = anchor.to_ref();
    assert!(link.is_internal());
    assert!(!link.has_alt_text());
    assert_eq!(link.target(), &HyperLinkTarget::Internal(anchor));
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn char_doesnt_crash(s in "\\PC") {
        let _ = Anchor::from_str(&s);
    }

    #[test]
    fn valid_values(s in ".+") {
        println!("valid_values {:?}", s);
        assert!(Anchor::from_str(&s).is_ok());
    }
}
