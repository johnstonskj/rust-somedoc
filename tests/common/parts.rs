use somedoc::model::block::label::AutoLabel;
use somedoc::model::block::{
    Alignment, DefinitionList, FrontMatter, HasAlignment, HasBlockContent, HasCaption, HasLabel,
    Heading, ImageBlock, Item, Label, List, MathBlock, Paragraph, Quote,
};
use somedoc::model::inline::Character;
use somedoc::model::inline::{HasInlineContent, HyperLink, Image, Math, Span, SpanStyle};
use somedoc::model::Document;
use std::str::FromStr;

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

pub fn document_with_labeled_heading() -> Document {
    Document::default()
        .add_heading(Heading::section("Test Document").auto_label().clone())
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

pub fn document_with_front_matter() -> Document {
    Document::default()
        .add_front_matter(FrontMatter::TableOfContents)
        .add_front_matter(FrontMatter::TableOfFigures)
        .add_front_matter(FrontMatter::TableOfTables)
        .add_front_matter(FrontMatter::TableOfEquations)
        .add_front_matter(FrontMatter::TableOfListings)
        .add_heading(Heading::section("Section One"))
        .add_heading(Heading::section("Section Two"))
        .clone()
}

pub fn paragraph_alignment() -> Document {
    Document::default()
        .add_paragraph(
            Paragraph::from("left-aligned")
                .set_alignment(Alignment::Left)
                .clone(),
        )
        .add_paragraph(
            Paragraph::from("right-aligned")
                .set_alignment(Alignment::Right)
                .clone(),
        )
        .add_paragraph(
            Paragraph::from("center-aligned")
                .set_alignment(Alignment::Centered)
                .clone(),
        )
        .add_paragraph(
            Paragraph::from("both-aligned")
                .set_alignment(Alignment::Justified)
                .clone(),
        )
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

pub fn labeled_ordered_list() -> Document {
    Document::default()
        .add_list(
            List::ordered()
                .add_item(
                    Item::plain_str("one")
                        .set_label(Label::from_str("lst1-itm1").unwrap())
                        .clone(),
                )
                .add_item(
                    Item::plain_str("two")
                        .set_label(Label::from_str("lst1-itm2").unwrap())
                        .clone(),
                )
                .add_item(
                    Item::plain_str("three")
                        .set_label(Label::from_str("lst1-itm3").unwrap())
                        .clone(),
                )
                .set_label(Label::from_str("lst1").unwrap())
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

pub fn image_block() -> Document {
    Document::default()
        .add_image(Image::new("https://example.org/example.png").into())
        .clone()
}

pub fn image_block_with_label_and_caption() -> Document {
    Document::default()
        .add_image(
            ImageBlock::from(Image::new("https://example.org/example.png"))
                .set_caption("An Example Image".into())
                .set_label(Label::from_str("img:example").unwrap())
                .clone(),
        )
        .clone()
}

pub fn math_block() -> Document {
    Document::default()
        .add_math(Math::from_str("x=2+2^2").unwrap().into())
        .clone()
}

pub fn math_block_with_label_and_caption() -> Document {
    Document::default()
        .add_math(
            MathBlock::from(Math::from_str("x=2+2^2").unwrap())
                .set_caption("Example Math".into())
                .set_label(Label::from_str("math:example").unwrap())
                .clone(),
        )
        .clone()
}

pub fn block_quote() -> Document {
    Document::default()
        .add_block_quote(Quote::paragraph(Paragraph::plain_str("a block quote")))
        .clone()
}

pub fn nested_block_quotes() -> Document {
    Document::default()
        .add_block_quote(
            Quote::default()
                .add_paragraph(Paragraph::plain_str("a block quote"))
                .add_block_quote(Quote::paragraph(Paragraph::plain_str(
                    "another block quote",
                )))
                .clone(),
        )
        .clone()
}

pub fn text_styles() -> Document {
    Document::default()
        .add_paragraph(
            Paragraph::default()
                .add_text_str("Here is some")
                .add_space()
                .add_span(Span::plain_str("plain"))
                .add_space()
                .add_span(Span::bold_str("bold"))
                .add_space()
                .add_span(Span::italic_str("italic"))
                .add_space()
                .add_span(Span::mono_str("mono"))
                .add_space()
                .add_span(Span::code_str("code"))
                .add_space()
                .add_span(Span::plain_str("plain"))
                .add_space()
                .add_span(Span::strikethrough_str("strikethrough"))
                .add_space()
                .add_span(Span::underline_str("underline"))
                .add_space()
                .add_span(Span::small_caps_str("small caps"))
                .add_space()
                .add_span(Span::superscript_str("superscript"))
                .add_space()
                .add_span(Span::subscript_str("subscript"))
                .add_span(Span::plain_str(" text."))
                .clone(),
        )
        .clone()
}

pub fn nested_text_styles() -> Document {
    Document::default()
        .add_paragraph(
            Paragraph::default()
                .add_text_str("Here is some ")
                .add_span(Span::with_styles(
                    "bold italic",
                    vec![SpanStyle::Bold, SpanStyle::Italic],
                ))
                .add_span(Span::plain_str(" text."))
                .clone(),
        )
        .add_paragraph(
            Paragraph::default()
                .add_text_str("Here is some ")
                .add_span(Span::with_styles(
                    "bold italic plain",
                    vec![SpanStyle::Bold, SpanStyle::Italic, SpanStyle::Plain],
                ))
                .add_span(Span::plain_str(" text."))
                .clone(),
        )
        .add_paragraph(
            Paragraph::default()
                .add_text_str("Here is some ")
                .add_span(Span::with_styles(
                    "bold plain italic",
                    vec![SpanStyle::Bold, SpanStyle::Plain, SpanStyle::Italic],
                ))
                .add_span(Span::plain_str(" text."))
                .clone(),
        )
        .clone()
}

pub fn hyper_links() -> Document {
    Document::default()
        .add_paragraph(Paragraph::link(HyperLink::external("https://example.org/")))
        .add_paragraph(Paragraph::link(HyperLink::external_with_caption_str(
            "https://example.org/",
            "example",
        )))
        .add_paragraph(Paragraph::link(HyperLink::internal(
            Label::from_str("section-2").unwrap(),
        )))
        .add_paragraph(Paragraph::link(HyperLink::internal_with_caption_str(
            Label::from_str("section-2").unwrap(),
            "example",
        )))
        .clone()
}

pub fn complex_paragraph() -> Document {
    Document::default()
        .add_paragraph(
            Paragraph::default()
                .add_text_str("This paragraph has ")
                .add_link(HyperLink::external_with_caption_str(
                    "https://example.org/",
                    "a link",
                ))
                .add_text_str(", some math:")
                .add_non_breaking_space()
                .add_math(Math::from_str("x=2+2^2").unwrap())
                .add_text_str(", a line break,")
                .add_line_break()
                .add_text_str("an image:")
                .add_non_breaking_space()
                .add_image(Image::with_alt_text(
                    "https://example.org/favicon.png",
                    "logo",
                ))
                .add_non_breaking_space()
                .add_character(Character::EmDash)
                .add_non_breaking_space()
                .add_text_str(" all together!")
                .clone(),
        )
        .clone()
}
