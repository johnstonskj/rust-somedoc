#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Used to insert auto-generated content tables. In general it is **not** the job of the writer
/// to generate the tables but to insert the necessary commands for the target markup to do so.
///
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub enum FrontMatter {
    // A table of all section headings.
    TableOfContents,
    // A table of all captioned math blocks.
    TableOfEquations,
    // A table of all captioned image blocks.
    TableOfFigures,
    // A table of all captioned code blocks.
    TableOfListings,
    // A table of all captioned tables.
    TableOfTables,
    // A table of glossary terms.
    Glossary,
}
