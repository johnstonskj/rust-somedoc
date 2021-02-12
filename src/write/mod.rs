/*!
Provides the functions, and format types, to serialize a [`Document`](../model/document/struct.Document.html)
in supported markup formats.

The enum [`OutputFormat`](enum.OutputFormat.html) provides a set of implemented formatters that may
then be used in [`write_document`](fn.write_document.html) and
[`write_document_to_string`](fn.write_document_to_string.html).

# Example

The following uses the `write_document_to_string` convenience function.

```rust
# use somedoc::model::Document;
use somedoc::write::{OutputFormat, write_document_to_string};

# fn make_some_document() -> Document { Document::default() }
let doc = make_some_document();

let doc_str = write_document_to_string(&doc, OutputFormat::Latex).unwrap();
println!("{}", doc_str);
```

# Writer Implementation

Each of the supported output formats implements *at least* the `Writer` and possibly the
`ConfigurableWriter` trait. These provide common functions for construction of the writer struct
and the `write_document` method. The following example constructs two separate writers and emits
the same document into both.

```rust
# use somedoc::model::Document;
use somedoc::write::{ConfigurableWriter, Writer};
use somedoc::write::markdown::{MarkdownFlavor, MarkdownWriter};
use somedoc::write::latex::LatexWriter;

# fn make_some_document() -> Document { Document::default() }
let doc = make_some_document();

let mut out = std::io::stdout();

let writer = MarkdownWriter::new_with(&mut out, MarkdownFlavor::CommonMark);
assert!(writer.write_document(&doc).is_ok());

let writer = LatexWriter::new(&mut out);
assert!(writer.write_document(&doc).is_ok());
```

*/

use std::fmt::{Display, Formatter};
use std::io::Write;
use std::str::FromStr;

use crate::error;
use crate::model::Document;
use crate::write::html::HtmlWriter;
use crate::write::latex::LatexWriter;
use crate::write::markdown::{MarkdownFlavor, MarkdownWriter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This indicates the output format to use when writing a document.
///
#[derive(Clone, Debug, PartialEq)]
pub enum OutputFormat {
    /// One of the supported flavors of Markdown, see [`markdown::MarkdownFlavor`](markdown/enum.MarkdownFlavor.html).
    #[cfg(feature = "fmt_markdown")]
    Markdown(MarkdownFlavor),

    /// Generic HTML, supports math via MathJax and code syntax via hightlight.js.
    #[cfg(feature = "fmt_html")]
    Html,

    /// Pretty generic LaTeX support, includes a number of packages for support of listings, block
    /// quotes, images, etc.
    #[cfg(feature = "fmt_latex")]
    Latex,
}

///
/// This trait can be implemented by a serializer to provide a common instantiation method.
///
pub trait Writer<'a, W: Write> {
    /// Create a new writer using the write implementation provided.
    fn new(w: &'a mut W) -> Self
    where
        Self: Sized;

    /// Format and write the provided document using the `Write` instance given during construction.
    fn write_document(&self, doc: &Document) -> crate::error::Result<()>;
}

///
/// This trait can be implemented by a serializer to provide a common instantiation method when
/// configuration may be passed to the new instance.
///
pub trait ConfigurableWriter<'a, W: Write, T: Default>: Writer<'a, W> {
    /// Create a new writer using the write implementation provided, and the configuration value(s).
    fn new_with(w: &'a mut W, config: T) -> Self;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Write the provided document `doc`, in the format described by `format`, into the write
/// implementation `w`. This is simply a convenience function
///
pub fn write_document<W: Write>(
    doc: &Document,
    format: OutputFormat,
    w: &mut W,
) -> crate::error::Result<()> {
    match format {
        #[cfg(feature = "fmt_markdown")]
        OutputFormat::Markdown(flavor) => {
            let writer = MarkdownWriter::new_with(w, flavor);
            writer.write_document(doc)
        }
        #[cfg(feature = "fmt_html")]
        OutputFormat::Html => {
            let writer = HtmlWriter::new(w);
            writer.write_document(doc)
        }
        #[cfg(feature = "fmt_latex")]
        OutputFormat::Latex => {
            let writer = LatexWriter::new(w);
            writer.write_document(doc)
        }
    }
}

///
/// A convenience function that will return a String containing the output of the `write_document`
/// function for the given `Document` instance.
///
pub fn write_document_to_string(
    doc: &Document,
    format: OutputFormat,
) -> crate::error::Result<String> {
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
                #[cfg(feature = "fmt_markdown")]
                Self::Markdown(f) => f.to_string(),
                #[cfg(feature = "fmt_html")]
                Self::Html => "html".to_string(),
                #[cfg(feature = "fmt_latex")]
                Self::Latex => "latex".to_string(),
            }
        )
    }
}

impl FromStr for OutputFormat {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "fmt_markdown")]
            "md" | "markdown" => Ok(Self::Markdown(MarkdownFlavor::GitHub)),
            #[cfg(feature = "fmt_markdown")]
            "xwiki" => Ok(Self::Markdown(MarkdownFlavor::XWiki)),
            #[cfg(feature = "fmt_html")]
            "html" => Ok(Self::Html),
            #[cfg(feature = "fmt_latex")]
            "latex" => Ok(Self::Latex),
            _ => Err(error::ErrorKind::UnknownFormat.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "fmt_html")]
pub mod html;

#[cfg(feature = "fmt_latex")]
pub mod latex;

#[cfg(feature = "fmt_markdown")]
pub mod markdown;

pub(crate) mod utils;
