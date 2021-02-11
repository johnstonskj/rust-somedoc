use somedoc::model::block::{DefinitionList, HasBlockContent, Heading, Item, List};
use somedoc::model::inline::HasInlineContent;
use somedoc::model::Document;

pub fn empty_document() -> Document {
    Document::default()
}

pub fn document_with_title() -> Document {
    Document::default().set_title("Test Document").clone()
}

pub fn document_with_heading() -> Document {
    Document::default()
        .add_heading(Heading::section("Test Document"))
        .clone()
}

pub fn document_with_headings() -> Document {
    Document::default()
        .add_heading(Heading::section("Section"))
        .add_heading(Heading::sub_section("Sub-section"))
        .add_heading(Heading::sub_sub_section("Sub-sub-section"))
        .add_heading(Heading::sub_sub_sub_section("Sub-sub-sub-section"))
        .add_heading(Heading::sub_sub_sub_sub_section("Sub-sub-sub-sub-section"))
        .add_heading(Heading::sub_sub_sub_sub_sub_section(
            "Sub-sub-sub-sub-sub-section",
        ))
        .add_heading(Heading::sub_sub_sub_sub_sub_sub_section(
            "Sub-sub-sub-sub-sub-sub-section",
        ))
        .clone()
}

pub fn ordered_list() -> Document {
    Document::default()
        .add_list(
            List::ordered()
                .add_item(Item::plain_str("one"))
                .add_item(Item::plain_str("two"))
                .add_item(Item::plain_str("three"))
                .clone(),
        )
        .clone()
}

pub fn unordered_list() -> Document {
    Document::default()
        .add_list(
            List::unordered()
                .add_item(Item::plain_str("one"))
                .add_item(Item::plain_str("two"))
                .add_item(Item::plain_str("three"))
                .clone(),
        )
        .clone()
}

pub fn nested_ordered_list() -> Document {
    Document::default()
        .add_list(
            List::ordered()
                .add_item(Item::plain_str("one"))
                .add_item(Item::plain_str("two"))
                .add_sub_list(
                    List::ordered()
                        .add_item(Item::plain_str("inner one"))
                        .add_item(Item::plain_str("inner two"))
                        .clone(),
                )
                .add_item(Item::plain_str("three"))
                .clone(),
        )
        .clone()
}

pub fn nested_unordered_list() -> Document {
    Document::default()
        .add_list(
            List::unordered()
                .add_item(Item::plain_str("one"))
                .add_item(Item::plain_str("two"))
                .add_sub_list(
                    List::unordered()
                        .add_item(Item::plain_str("inner one"))
                        .add_item(Item::plain_str("inner two"))
                        .clone(),
                )
                .add_item(Item::plain_str("three"))
                .clone(),
        )
        .clone()
}

pub fn nested_mixed_lists() -> Document {
    Document::default()
        .add_list(
            List::unordered()
                .add_item(Item::plain_str("one"))
                .add_item(Item::plain_str("two"))
                .add_sub_list(
                    List::ordered()
                        .add_item(Item::plain_str("inner one"))
                        .add_sub_list(
                            List::unordered()
                                .add_item(Item::plain_str("inner inner one"))
                                .clone(),
                        )
                        .add_item(Item::plain_str("inner two"))
                        .clone(),
                )
                .add_item(Item::plain_str("three"))
                .clone(),
        )
        .clone()
}

pub fn definition_list() -> Document {
    Document::default()
        .add_definition_list(
            DefinitionList::default()
                .add_definition_from("Universe".into(), "Big, really big".into())
                .clone(),
        )
        .clone()
}
