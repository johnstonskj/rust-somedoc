/*!
Write a document as HTML. This includes a small number of additional CSS and JavaScript assets
to support math formatting and code syntax highlighting.

# Example

```rust
# use somedoc::model::Document;
use somedoc::write::{OutputFormat, write_document_to_string};

# fn make_some_document() -> Document { Document::default() }
let doc = make_some_document();

let doc_str = write_document_to_string(&doc, OutputFormat::Json).unwrap();
println!("{}", doc_str);
```

*/

use crate::model::Document;
use crate::write::Writer;
use std::cell::RefCell;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Implementation of the HTML writer structure, usually this is accessed via the `writer`
/// function, but may be used directly.
///
/// # Example
///
/// ```rust
/// # use somedoc::model::Document;
/// use somedoc::write::json::JsonWriter;
/// use somedoc::write::{write_document_to_string, Writer};
/// use somedoc::model::visitor::walk_document;
///
/// # fn make_some_document() -> Document { Document::default() }
/// let doc = make_some_document();
/// let mut out = std::io::stdout();
/// let writer = JsonWriter::new(&mut out);
/// assert!(writer.write_document(&doc).is_ok());
/// ```
///
#[derive(Debug)]
pub struct JsonWriter<'a, W: Write> {
    w: RefCell<&'a mut W>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Implementation of the writer function for JSON.
///
/// While this can be called directly it is most often used  by calling either
/// [`model::write_document`](../fn.write_document.html) or
/// [`model::write_document_to_string`](../fn.write_document_to_string.html).
///
#[inline]
pub fn writer<W: Write>(doc: &Document, w: &mut W) -> crate::error::Result<()> {
    let writer = JsonWriter::new(w);
    writer.write_document(doc)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> Writer<'a, W> for JsonWriter<'a, W> {
    fn new(w: &'a mut W) -> Self {
        Self {
            w: RefCell::from(w),
        }
    }

    fn write_document(&self, doc: &Document) -> crate::error::Result<()> {
        let stringified = serde_json::to_string(doc)?;
        write!(&mut self.w.borrow_mut(), "{}", stringified)?;
        Ok(())
    }
}
