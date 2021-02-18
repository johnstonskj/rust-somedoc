use somedoc::write::markdown::MarkdownFlavor;
use somedoc::write::OutputFormat;
use std::str::FromStr;

#[test]
fn test_to_string() {
    assert_eq!(OutputFormat::Html.to_string(), "html".to_string());
    assert_eq!(OutputFormat::Json.to_string(), "json".to_string());
    assert_eq!(OutputFormat::Latex.to_string(), "latex".to_string());
    assert_eq!(
        OutputFormat::Markdown(Default::default()).to_string(),
        "markdown+commonmark".to_string()
    );
    assert_eq!(
        OutputFormat::Markdown(MarkdownFlavor::Strict).to_string(),
        "markdown+strict".to_string()
    );
    assert_eq!(
        OutputFormat::Markdown(MarkdownFlavor::CommonMark).to_string(),
        "markdown+commonmark".to_string()
    );
    assert_eq!(
        OutputFormat::Markdown(MarkdownFlavor::GitHub).to_string(),
        "markdown+gfm".to_string()
    );
    assert_eq!(
        OutputFormat::Markdown(MarkdownFlavor::Multi).to_string(),
        "markdown+multi".to_string()
    );
    assert_eq!(
        OutputFormat::Markdown(MarkdownFlavor::PhpExtra).to_string(),
        "markdown+extra".to_string()
    );
    assert_eq!(
        OutputFormat::Markdown(MarkdownFlavor::XWiki).to_string(),
        "markdown+xwiki".to_string()
    );
}

#[test]
fn test_from_str() {
    assert_eq!(OutputFormat::from_str("html").unwrap(), OutputFormat::Html);
    assert_eq!(OutputFormat::from_str("json").unwrap(), OutputFormat::Json);
    assert_eq!(
        OutputFormat::from_str("latex").unwrap(),
        OutputFormat::Latex
    );
    assert_eq!(
        OutputFormat::from_str("markdown").unwrap(),
        OutputFormat::Markdown(Default::default())
    );
    assert_eq!(
        OutputFormat::from_str("markdown+strict").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::Strict)
    );
    assert_eq!(
        OutputFormat::from_str("markdown+commonmark").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::CommonMark)
    );
    assert_eq!(
        OutputFormat::from_str("markdown+gfm").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::GitHub)
    );
    assert_eq!(
        OutputFormat::from_str("markdown+multi").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::Multi)
    );
    assert_eq!(
        OutputFormat::from_str("markdown+extra").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::PhpExtra)
    );
    assert_eq!(
        OutputFormat::from_str("markdown+xwiki").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::XWiki)
    );
}

#[test]
fn test_from_str_extras() {
    assert_eq!(
        OutputFormat::from_str("markdown+og").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::Strict)
    );

    assert_eq!(
        OutputFormat::from_str("markdown+cm").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::CommonMark)
    );
    assert_eq!(
        OutputFormat::from_str("markdown+common").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::CommonMark)
    );

    assert_eq!(
        OutputFormat::from_str("markdown+github").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::GitHub)
    );

    assert_eq!(
        OutputFormat::from_str("markdown+mmd").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::Multi)
    );

    assert_eq!(
        OutputFormat::from_str("markdown+php_extra").unwrap(),
        OutputFormat::Markdown(MarkdownFlavor::PhpExtra)
    );
}
