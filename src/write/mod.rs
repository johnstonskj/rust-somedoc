/*!
Provides the functions, and format types, to serialize a [`Document`](../model/document/struct.Document.html)
in supported markup formats.

The enum [`OutputFormat`](enum.OutputFormat.html) provides a set of implemented formatters that may
then be used in [`write_document`](fn.write_document.html) and
[`write_document_to_string`](fn.write_document_to_string.html).

# Example

```rust
# use somedoc::model::Document;
use somedoc::write::{write_document_to_string, OutputFormat};

# fn make_some_document() -> Document { Document::default() }
let doc = make_some_document();

let doc_str = write_document_to_string(&doc, OutputFormat::XWiki).unwrap();
println!("{}", doc_str);
```

# Writer Implementation

Each writer module provides a single function that is called by `write_document` and is passed the
document and the `Write` implementation. While the following types are **not** used in this module
they are useful in understanding the two writer types.

```rust
use std::io::Write;
use somedoc::model::document::Document;

type WriterFn<W: Write> = dyn Fn(&Document, &mut W) -> std::io::Result<()>;

type FlavoredWriterFn<E: Default, W: Write> =
        dyn Fn(&Document, E, &mut W) -> std::io::Result<()>;
```

The function type `WriterFn` is the primary case where the document and writer are passed in.
However, some formats such as Markdown have multiple *flavors* that the caller may wish to select
from. The secont type above, `FlavoredWriterFn` takes an additional parameter that denotes the
flavor. In the type [`OutputFormat`](enum.OutputFormat.html) you can see that the `Markdown`
variant includes the flavor, and this will be the concrete value for `E` when calling the
markdown implementation of `FlavoredWriterFn`.

*/

use crate::error;
use crate::model::Document;
use crate::write::markdown::MarkdownFlavor;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This indicates the output format to use when writing a document.
///
#[derive(Clone, Debug, PartialEq)]
pub enum OutputFormat {
    /// One of the supported flavors of Markdown, see [`markdown::MarkdownFlavor`](markdown/enum.MarkdownFlavor.html).
    Markdown(MarkdownFlavor),

    // The XWiki native syntax.
    XWiki,
}

// type WriterFn<W: Write> = dyn Fn(&Document, &mut W) -> std::io::Result<()>;
//
// type FlavoredWriterFn<E: Default, W: Write> = dyn Fn(&Document, E, &mut W) -> std::io::Result<()>;
//
// pub enum Writer<E: Default, W: Write> {
//     Writer(Box<WriterFn<W>>),
//     FlavoredWriter(Box<FlavoredWriterFn<E, W>>),
// }

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Write the provided document `doc`, in the format described by `format`, into the write
/// implementation `w`.
///
pub fn write_document<W: Write>(
    doc: &Document,
    format: OutputFormat,
    w: &mut W,
) -> std::io::Result<()> {
    match format {
        OutputFormat::Markdown(flavor) => markdown::writer::<W>(doc, flavor, w),
        OutputFormat::XWiki => xwiki::writer(doc, w),
    }
}

///
/// A convenience function that will return a String containing the output of the `write_document`
/// function for the given `Document` instance.
///
pub fn write_document_to_string(doc: &Document, format: OutputFormat) -> std::io::Result<String> {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    write_document(doc, format, &mut buffer)?;
    Ok(String::from_utf8(buffer.into_inner()).unwrap())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Markdown(Default::default())
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Markdown(f) => format!("markdown+{}", f),
                OutputFormat::XWiki => "xwiki".to_string(),
            }
        )
    }
}

impl FromStr for OutputFormat {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "md" | "markdown" => Ok(Self::Markdown(Default::default())),
            "xwiki" => Ok(Self::XWiki),
            _ => Err(error::ErrorKind::UnknownFormat.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod markdown;

pub mod xwiki;
