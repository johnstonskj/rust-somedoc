/*!
One-line description.

More detailed description, with

# Example

*/

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct Delimiters {
    a: ActualDelimiters,
    multiply_prefix: bool,
    multiply_suffix: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum Feature {
    Space,
    NonBreakSpace,
    Hyphen,
    EmDash,
    EnDash,
    Emoji,
    SpanPlain,
    SpanItalic,
    SpanSlanted,
    SpanBold,
    SpanMono,
    SpanCode,
    SpanStrikethrough,
    SpanUnderline,
    SpanSmallCaps,
    SpanSuperscript,
    SpanSubscript,
    SpanSized,
    InlineAnchor,
    InlineHyperLink,
    InlineImage,
    InlineMath,
    InlineLineBreak,
    BlockComment,
    BlockHeading,
    BlockImage,
    BlockMath,
    BlockOrderedList,
    BlockUnorderedList,
    BlockDefinitionList,
    BlockFormatted,
    BlockCode,
    BlockCodeLanguage,
    BlockParagraph,
    BlockQuote,
    BlockTable,
    BlockThematicBreak,
    BlockSeparator,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum ActualDelimiters {
    None,
    Just {
        value: String,
    },
    Prefix {
        prefix: String,
    },
    Suffix {
        suffix: String,
    },
    Pair {
        prefix: String,
        suffix: String,
    },
    PaddedPrefix {
        prefix: String,
        prefix_pad: String,
    },
    PaddedSuffix {
        suffix: String,
        suffix_pad: String,
    },
    PaddedPair {
        prefix: String,
        prefix_pad: String,
        suffix: String,
        suffix_pad: String,
    },
    LinePrefix {
        prefix: String,
    },
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref FEATURES: HashMap<&'static str, Delimiters> = [
        ("markdown::Emoji", Delimiters::matching_pair(":")),
        ("markdown::SpanItalic", Delimiters::matching_pair("*")),
        ("markdown::SpanSlanted", Delimiters::matching_pair("*")),
        ("markdown::SpanBold", Delimiters::matching_pair("**")),
        ("markdown::SpanMono", Delimiters::matching_pair("`")),
        ("markdown::SpanCode", Delimiters::matching_pair("`")),
        ("markdown::InlineImage", Delimiters::prefix("!")),
        ("markdown::InlineLineBreak", Delimiters::prefix("  \n")),
        (
            "markdown::BlockHeading",
            Delimiters::multiplied_prefix("#", " ")
        ),
        ("markdown::BlockOrderedList", Delimiters::prefix("1. ")),
        ("markdown::BlockUnorderedList", Delimiters::prefix("* ")),
        ("markdown::BlockFormatted", Delimiters::line_prefix("    ")),
        ("markdown+cm::BlockCode", Delimiters::matching_pair("```")),
        ("markdown+gfm::BlockCode", Delimiters::matching_pair("```")),
        ("markdown+md::BlockCode", Delimiters::matching_pair("```")),
        (
            "markdown+mdextra::BlockCode",
            Delimiters::matching_pair("```")
        ),
        (
            "markdown::BlockQuote",
            Delimiters::multiplied_prefix("> ", "")
        ),
        ("markdown::BlockThematicBreak", Delimiters::just("-----")),
        ("markdown::BlockSeparator", Delimiters::just("\n"))
    ]
    .iter()
    .cloned()
    .collect();
}

pub(crate) fn has_feature(format: &impl Display, feature: &Feature) -> bool {
    let format = format.to_string();
    let key = format!("{}::{}", format, feature);
    if !FEATURES.contains_key(key.as_str()) {
        if let Some(format) = format.split('+').next() {
            let key = format!("{}::{}", format, feature);
            FEATURES.contains_key(key.as_str())
        } else {
            false
        }
    } else {
        true
    }
}

pub(crate) fn delimiter_for<'a>(
    format: &impl Display,
    feature: &Feature,
) -> Option<&'a Delimiters> {
    let format = format.to_string();
    let key = format!("{}::{}", format, feature);
    match FEATURES.get(key.as_str()) {
        None => {
            if let Some(format) = format.split('+').next() {
                let key = format!("{}::{}", format, feature);
                FEATURES.get(key.as_str())
            } else {
                None
            }
        }
        Some(v) => Some(v),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Feature::Space => "Space",
                Feature::NonBreakSpace => "NonBreakSpace",
                Feature::Hyphen => "Hyphen",
                Feature::EmDash => "EmDash",
                Feature::EnDash => "EnDash",
                Feature::Emoji => "Emoji",
                Feature::SpanPlain => "SpanPlain",
                Feature::SpanItalic => "SpanItalic",
                Feature::SpanSlanted => "SpanSlanted",
                Feature::SpanBold => "SpanBold",
                Feature::SpanMono => "SpanMono",
                Feature::SpanCode => "SpanCode",
                Feature::SpanStrikethrough => "SpanStrikethrough",
                Feature::SpanUnderline => "SpanUnderline",
                Feature::SpanSmallCaps => "SpanSmallCaps",
                Feature::SpanSuperscript => "SpanSuperscript",
                Feature::SpanSubscript => "SpanSubscript",
                Feature::SpanSized => "SpanSized",
                Feature::InlineAnchor => "InlineAnchor",
                Feature::InlineHyperLink => "InlineHyperLink",
                Feature::InlineImage => "InlineImage",
                Feature::InlineMath => "InlineMath",
                Feature::InlineLineBreak => "InlineLineBreak",
                Feature::BlockComment => "BlockComment",
                Feature::BlockHeading => "BlockHeading",
                Feature::BlockImage => "BlockImage",
                Feature::BlockMath => "BlockMath",
                Feature::BlockOrderedList => "BlockOrderedList",
                Feature::BlockUnorderedList => "BlockUnorderedList",
                Feature::BlockDefinitionList => "BlockDefinitionList",
                Feature::BlockFormatted => "BlockFormatted",
                Feature::BlockCode => "BlockCode",
                Feature::BlockCodeLanguage => "BlockCodeLanguage",
                Feature::BlockParagraph => "BlockParagraph",
                Feature::BlockQuote => "BlockQuote",
                Feature::BlockTable => "BlockTable",
                Feature::BlockThematicBreak => "BlockThematicBreak",
                Feature::BlockSeparator => "BlockSeparator",
            },
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl From<ActualDelimiters> for Delimiters {
    fn from(v: ActualDelimiters) -> Self {
        Self {
            a: v,
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }
}

impl Delimiters {
    pub fn none() -> Self {
        Self {
            a: ActualDelimiters::None,
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }

    pub fn just(v: &str) -> Self {
        Self {
            a: ActualDelimiters::Just {
                value: v.to_string(),
            },
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }

    pub fn line_prefix(p: &str) -> Self {
        Self {
            a: ActualDelimiters::LinePrefix {
                prefix: p.to_string(),
            },
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }

    pub fn prefix(p: &str) -> Self {
        Self {
            a: ActualDelimiters::Prefix {
                prefix: p.to_string(),
            },
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }

    pub fn suffix(s: &str) -> Self {
        Self {
            a: ActualDelimiters::Suffix {
                suffix: s.to_string(),
            },
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }

    pub fn pair(p: &str, s: &str) -> Self {
        Self {
            a: ActualDelimiters::Pair {
                prefix: p.to_string(),
                suffix: s.to_string(),
            },
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }

    pub fn matching_pair(ps: &str) -> Self {
        Self {
            a: ActualDelimiters::Pair {
                prefix: ps.to_string(),
                suffix: ps.to_string(),
            },
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }

    pub fn prefix_with_pad(p: &str, pp: &str) -> Self {
        Self {
            a: ActualDelimiters::PaddedPrefix {
                prefix: p.to_string(),
                prefix_pad: pp.to_string(),
            },
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }

    pub fn suffix_with_pad(s: &str, ss: &str) -> Self {
        Self {
            a: ActualDelimiters::PaddedSuffix {
                suffix: s.to_string(),
                suffix_pad: ss.to_string(),
            },
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }

    pub fn pair_with_pad(p: &str, pp: &str, s: &str, ss: &str) -> Self {
        Self {
            a: ActualDelimiters::PaddedPair {
                prefix: p.to_string(),
                prefix_pad: pp.to_string(),
                suffix: s.to_string(),
                suffix_pad: ss.to_string(),
            },
            multiply_prefix: false,
            multiply_suffix: false,
        }
    }

    pub fn multiplied_prefix(p: &str, pp: &str) -> Self {
        Self {
            a: ActualDelimiters::PaddedPrefix {
                prefix: p.to_string(),
                prefix_pad: pp.to_string(),
            },
            multiply_prefix: true,
            multiply_suffix: false,
        }
    }

    pub fn multiplied_suffix(s: &str, ss: &str) -> Self {
        Self {
            a: ActualDelimiters::PaddedSuffix {
                suffix: s.to_string(),
                suffix_pad: ss.to_string(),
            },
            multiply_prefix: false,
            multiply_suffix: true,
        }
    }

    pub fn multiplied_pair(p: &str, s: &str) -> Self {
        Self {
            a: ActualDelimiters::Pair {
                prefix: p.to_string(),
                suffix: s.to_string(),
            },
            multiply_prefix: true,
            multiply_suffix: true,
        }
    }

    pub fn multiplied_pair_with_pad(p: &str, pp: &str, s: &str, ss: &str) -> Self {
        Self {
            a: ActualDelimiters::PaddedPair {
                prefix: p.to_string(),
                prefix_pad: pp.to_string(),
                suffix: s.to_string(),
                suffix_pad: ss.to_string(),
            },
            multiply_prefix: true,
            multiply_suffix: true,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_none(&self) -> bool {
        match self.a {
            ActualDelimiters::None => true,
            _ => false,
        }
    }

    pub fn is_some(&self) -> bool {
        match self.a {
            ActualDelimiters::None => false,
            _ => true,
        }
    }

    pub fn is_just(&self) -> bool {
        match self.a {
            ActualDelimiters::Just { .. } => true,
            _ => false,
        }
    }

    #[inline]
    pub fn has_prefix(&self) -> bool {
        match self.a {
            ActualDelimiters::Prefix { .. }
            | ActualDelimiters::Pair { .. }
            | ActualDelimiters::PaddedPrefix { .. }
            | ActualDelimiters::PaddedPair { .. } => true,
            _ => false,
        }
    }

    pub fn get_prefix(&self) -> Option<&String> {
        match &self.a {
            ActualDelimiters::Prefix { prefix: p } => Some(p),
            ActualDelimiters::Pair { prefix: p, .. } => Some(p),
            ActualDelimiters::PaddedPrefix { prefix: p, .. } => Some(p),
            ActualDelimiters::PaddedPair { prefix: p, .. } => Some(p),
            _ => None,
        }
    }

    #[inline]
    pub fn has_prefix_pad(&self) -> bool {
        match self.a {
            ActualDelimiters::PaddedPrefix { .. } | ActualDelimiters::PaddedPair { .. } => true,
            _ => false,
        }
    }

    pub fn get_prefix_pad(&self) -> Option<&String> {
        match &self.a {
            ActualDelimiters::PaddedPrefix { prefix_pad: pp, .. } => Some(pp),
            ActualDelimiters::PaddedPair { prefix_pad: pp, .. } => Some(pp),
            _ => None,
        }
    }

    #[inline]
    pub fn has_suffix(&self) -> bool {
        match self.a {
            ActualDelimiters::Suffix { .. }
            | ActualDelimiters::Pair { .. }
            | ActualDelimiters::PaddedSuffix { .. }
            | ActualDelimiters::PaddedPair { .. } => true,
            _ => false,
        }
    }

    pub fn get_suffix(&self) -> Option<&String> {
        match &self.a {
            ActualDelimiters::Suffix { suffix: s } => Some(s),
            ActualDelimiters::Pair { suffix: s, .. } => Some(s),
            ActualDelimiters::PaddedSuffix { suffix: s, .. } => Some(s),
            ActualDelimiters::PaddedPair { suffix: s, .. } => Some(s),
            _ => None,
        }
    }

    #[inline]
    pub fn has_suffix_pad(&self) -> bool {
        match self.a {
            ActualDelimiters::PaddedSuffix { .. } | ActualDelimiters::PaddedPair { .. } => true,
            _ => false,
        }
    }

    pub fn get_suffix_pad(&self) -> Option<&String> {
        match &self.a {
            ActualDelimiters::PaddedSuffix { suffix_pad: sp, .. } => Some(sp),
            ActualDelimiters::PaddedPair { suffix_pad: sp, .. } => Some(sp),
            _ => None,
        }
    }

    #[inline]
    pub fn is_line_prefix(&self) -> bool {
        match self.a {
            ActualDelimiters::LinePrefix { .. } => true,
            _ => false,
        }
    }

    pub fn get_line_prefix(&self) -> Option<&String> {
        match &self.a {
            ActualDelimiters::LinePrefix { prefix: p } => Some(p),
            _ => None,
        }
    }

    pub fn format(&self, v: &str) -> String {
        self.format_with_multiple(v, 0)
    }

    pub fn format_with_multiple(&self, v: &str, m: usize) -> String {
        fn multiply(multiply: bool, v: &str, m: usize) -> String {
            if multiply && m > 1 {
                let mut out: String = String::new();
                for _ in 0..m {
                    out.push_str(v);
                }
                out
            } else {
                v.to_string()
            }
        }

        match &self.a {
            ActualDelimiters::None => v.to_string(),
            ActualDelimiters::Just { value: v } => v.to_string(),
            ActualDelimiters::Prefix { prefix: p } => {
                format!("{}{}", multiply(self.multiply_prefix, p, m), v)
            }
            ActualDelimiters::Suffix { suffix: s } => {
                format!("{}{}", v, multiply(self.multiply_suffix, s, m))
            }
            ActualDelimiters::Pair {
                prefix: p,
                suffix: s,
            } => format!("{}{}{}", multiply(self.multiply_prefix, p, m), v, s),
            ActualDelimiters::PaddedPrefix {
                prefix: p,
                prefix_pad: pp,
            } => format!("{}{}{}", multiply(self.multiply_prefix, p, m), pp, v),
            ActualDelimiters::PaddedSuffix {
                suffix: s,
                suffix_pad: ss,
            } => format!("{}{}{}", v, ss, multiply(self.multiply_suffix, s, m)),
            ActualDelimiters::PaddedPair {
                prefix: p,
                prefix_pad: pp,
                suffix: s,
                suffix_pad: ss,
            } => format!(
                "{}{}{}{}{}",
                multiply(self.multiply_prefix, p, m),
                pp,
                v,
                ss,
                multiply(self.multiply_suffix, s, m)
            ),
            ActualDelimiters::LinePrefix { prefix: p } => p.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const INPUTS: &[&str] = &["", " ", "hi", "9o7y4tjshdfni6713r"];

    fn format_and_test(inputs: &[&str], delim: Delimiters, outputs: Vec<String>) {
        assert_eq!(inputs.len(), outputs.len());
        for i in 0..inputs.len() {
            assert_eq!(
                &delim.format(inputs.get(i).unwrap()),
                outputs.get(i).unwrap()
            );
        }
    }

    #[test]
    fn test_none() {
        format_and_test(
            INPUTS,
            Delimiters::none(),
            INPUTS.iter().map(|v| v.to_string()).collect(),
        );
    }

    #[test]
    fn test_just() {
        format_and_test(
            INPUTS,
            Delimiters::just("%"),
            INPUTS.iter().map(|_| "%".to_string()).collect(),
        );
    }

    #[test]
    fn test_prefix() {
        format_and_test(
            INPUTS,
            Delimiters::prefix("%"),
            INPUTS.iter().map(|v| format!("%{}", v)).collect(),
        );
    }

    #[test]
    fn test_suffix() {
        format_and_test(
            INPUTS,
            Delimiters::suffix("%"),
            INPUTS.iter().map(|v| format!("{}%", v)).collect(),
        );
    }

    #[test]
    fn test_pair() {
        format_and_test(
            INPUTS,
            Delimiters::pair("%", "&"),
            INPUTS.iter().map(|v| format!("%{}&", v)).collect(),
        );
        format_and_test(
            INPUTS,
            Delimiters::matching_pair("%%"),
            INPUTS.iter().map(|v| format!("%%{}%%", v)).collect(),
        );
    }

    #[test]
    fn test_prefix_with_pad() {
        format_and_test(
            INPUTS,
            Delimiters::prefix_with_pad("%", "-"),
            INPUTS.iter().map(|v| format!("%-{}", v)).collect(),
        );
    }

    #[test]
    fn test_suffix_with_pad() {
        format_and_test(
            INPUTS,
            Delimiters::suffix_with_pad("%", "-"),
            INPUTS.iter().map(|v| format!("{}-%", v)).collect(),
        );
    }
}
