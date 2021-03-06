use proptest::prelude::*;
use somedoc::model::block::{HasCaption, Label};
use somedoc::model::inline::{HyperLink, HyperLinkTarget};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_reject_empty() {
    let result = Label::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_new() {
    let result = Label::from_str("test_new");
    assert!(result.is_ok());
    println!("{:?}", result);
    assert_eq!(result.unwrap().inner(), &String::from("test_new"));
}

#[test]
fn test_into_inner() {
    let result = Label::from_str("test-into-inner");
    assert!(result.is_ok());
    println!("{:?}", result);
    assert_eq!(
        result.unwrap().into_inner(),
        String::from("test-into-inner")
    );
}

#[test]
fn test_to_ref() {
    let result = Label::from_str("test_to_ref");
    assert!(result.is_ok());
    println!("{:?}", result);
    let anchor = result.unwrap();

    let link: HyperLink = anchor.clone().into();
    assert!(link.is_internal());
    assert!(!link.has_caption());
    assert_eq!(link.target(), &HyperLinkTarget::Internal(anchor));
}

#[test]
fn test_generated_labels() {
    for _ in 0..100 {
        let label = Label::generate(None);
        assert!(label.starts_with("gen:"));
        assert!(label.len() > 5);
        assert!(Label::is_valid(&label.to_string()))
    }
}

#[test]
fn test_generated_prefixed_labels() {
    for _ in 0..100 {
        let label = Label::generate(Some("test"));
        assert!(label.starts_with("test:"));
        assert!(label.len() > 6);
        assert!(Label::is_valid(&label.to_string()))
    }
}

#[test]
fn test_is_valid() {
    assert!(Label::is_valid("a"));
    assert!(Label::is_valid("a1"));
    assert!(Label::is_valid("a_1"));
    assert!(Label::is_valid("a-1"));
    assert!(Label::is_valid("a.1"));
    assert!(Label::is_valid("a:1"));
}

#[test]
fn test_is_not_valid() {
    assert!(!Label::is_valid(""));
    assert!(!Label::is_valid("1"));
    assert!(!Label::is_valid(" "));
    assert!(!Label::is_valid("_"));
    assert!(!Label::is_valid("-"));
    assert!(!Label::is_valid("."));
    assert!(!Label::is_valid(":"));
    assert!(Label::is_valid("a_"));
    assert!(Label::is_valid("a-"));
    assert!(Label::is_valid("a."));
    assert!(Label::is_valid("a:"));
    assert!(!Label::is_valid("a a"));
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn char_doesnt_crash(s in "\\PC") {
        let _ = Label::from_str(&s);
    }

    #[test]
    fn valid_values(s in r"\p{L}+[\p{L}\p{N}_\-\.]*") {
        println!("valid_values {:?}", s);
        assert!(Label::from_str(&s).is_ok());
    }
}
