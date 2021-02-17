use somedoc::model::block::Caption;

fn caption_eq(caption: Caption, expected: String) {
    assert_eq!(caption.to_string(), expected.clone());
    assert_eq!(caption.inner(), &expected);
    assert_eq!(caption.into_inner(), expected);
}

#[test]
fn test_caption_empty() {
    let caption = Caption::default();
    assert!(caption.is_empty());
    caption_eq(caption, String::new());
}

#[test]
fn test_caption_from_string() {
    let caption = Caption::from(String::from("hello"));
    assert!(!caption.is_empty());
    caption_eq(caption, String::from("hello"));
}

#[test]
fn test_caption_from_str() {
    let caption = Caption::from("hello");
    assert!(!caption.is_empty());
    caption_eq(caption, String::from("hello"));
}
