use crate::error;
use crate::model::block::{Alignment, BlockContent, Caption, HasAlignment, HasCaption, Label};
use crate::model::inline::HasInlineContent;
use crate::model::inline::{Character, InlineContent};
use crate::model::{block::HasLabel, HasInnerContent};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A typical, simple, table of rows and columns.
///
#[derive(Clone, Debug)]
pub struct Table {
    label: Option<Label>,
    columns: Vec<Column>,
    rows: Vec<Row>,
    caption: Option<Caption>,
}

///
/// Defines the shape of a table, each column has a label and alignment.
///
#[derive(Clone, Debug)]
pub struct Column {
    text: String,
    alignment: Alignment,
}

///
/// Rows represent data in the table and consist of a vector of `Cell`s.
///
#[derive(Clone, Debug)]
pub struct Row {
    cells: Vec<Cell>,
}

///
/// A Cell is an element at a specific row and column in the table. It is an inline content container.
///
#[derive(Clone, Debug)]
pub struct Cell {
    label: Option<Label>,
    inner: Vec<InlineContent>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Table {
    fn default() -> Self {
        Self {
            label: None,
            columns: Default::default(),
            rows: Default::default(),
            caption: None,
        }
    }
}

label_impl!(Table);

block_impls!(Table);

has_captioned_impls!(Table);

impl Table {
    /// Construct a new `Table` from an array of `Column` values. These column values describe the
    /// shape of the table to be constructed.
    pub fn new(columns: &[Column]) -> Self {
        Self {
            label: None,
            columns: columns.to_vec(),
            rows: Default::default(),
            caption: None,
        }
    }

    /// Return true if this table has columns (although it's not really a table without them), else
    /// `false`.
    pub fn has_columns(&self) -> bool {
        !self.columns.is_empty()
    }

    /// Return the current array of columns in the table.
    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }

    /// Add a new column to the table, this is appended to the current array.
    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column)
    }

    /// Add an array of columns to the table, these are appended to the current array.
    pub fn add_columns(&mut self, columns: &[Column]) {
        self.columns.extend_from_slice(columns)
    }

    /// Returns `true` if this table has data rows, else `false`.
    pub fn has_rows(&self) -> bool {
        !self.rows.is_empty()
    }

    /// Returns an array of data rows.
    pub fn rows(&self) -> &Vec<Row> {
        &self.rows
    }

    /// Add a new data row to the column, this is appended to the current array.
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row)
    }

    /// Add an array of new data rows to the column, these are appended to the current array.
    pub fn add_rows(&mut self, rows: &[Row]) {
        self.rows.extend_from_slice(rows)
    }
}

// ------------------------------------------------------------------------------------------------

impl From<String> for Column {
    fn from(text: String) -> Self {
        Self::new(&text)
    }
}

impl From<&str> for Column {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
            alignment: Default::default(),
        }
    }
}

impl From<(&str, Alignment)> for Column {
    fn from(column: (&str, Alignment)) -> Self {
        Self::from(&column)
    }
}

impl From<&(&str, Alignment)> for Column {
    fn from(column: &(&str, Alignment)) -> Self {
        Self::with_alignment(column.0, column.1.clone())
    }
}

alignment_impl!(Column);

impl Column {
    /// Construct a new column with the given text. `text` is the value of the header cell when
    /// written.
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            alignment: Default::default(),
        }
    }

    /// Construct a new column with the given text and specified alignment. `text` is the value of
    /// the header cell when written, `alignment` is the alignment of all header and data values
    /// in the table when written.
    pub fn with_alignment(text: &str, alignment: Alignment) -> Self {
        Self {
            text: text.to_string(),
            alignment,
        }
    }

    /// The text of the column header cell.
    pub fn text(&self) -> &String {
        &self.text
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Vec<Cell>> for Row {
    fn from(cells: Vec<Cell>) -> Self {
        Self { cells }
    }
}

impl Row {
    /// Create a new data row from an array of cells.
    pub fn new(cells: &[Cell]) -> Self {
        Self {
            cells: cells.to_vec(),
        }
    }

    /// Return the cells in this row.
    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    /// Append a new cell to this row.
    pub fn add_cell(&mut self, cell: Cell) -> &mut Self {
        self.cells.push(cell);
        self
    }

    /// Append all cells to this row.
    pub fn add_cells(&mut self, cells: &[Cell]) -> &mut Self {
        self.cells.extend_from_slice(cells);
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Cell {
    fn default() -> Self {
        Cell::skip()
    }
}

label_impl!(Cell);

has_inline_impls!(Cell);

impl Cell {
    /// Create a new cell that may be skipped in the output.
    pub fn skip() -> Self {
        Self {
            label: None,
            inner: Default::default(),
        }
    }

    /// Create a cell with empty content.
    pub fn empty() -> Self {
        Self {
            label: None,
            inner: vec![Character::NonBreakSpace.into()],
        }
    }
}
