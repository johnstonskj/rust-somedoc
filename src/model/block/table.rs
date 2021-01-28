use crate::error;
use crate::model::block::{BlockContent, Caption, Captioned};
use crate::model::inline::HasInlineContent;
use crate::model::inline::{Character, InlineContent};
use crate::model::HasInnerContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The alignment to be used for values in a column.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Alignment {
    Default,
    Left,
    Right,
    Centered,
}

///
/// A typical, simple, table of rows and columns.
///
#[derive(Clone, Debug)]
pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Row>,
    caption: Option<Caption>,
}

///
/// Defines the shape of a table, each column has a label and alignment.
///
#[derive(Clone, Debug)]
pub struct Column {
    label: String,
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
    inner: Vec<InlineContent>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Default
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Table {
    fn default() -> Self {
        Self {
            columns: Default::default(),
            rows: Default::default(),
            caption: None,
        }
    }
}

block_impls!(Table);

has_captioned_impls!(Table);

impl Table {
    pub fn new(columns: &[Column]) -> Self {
        Self {
            columns: columns.to_vec(),
            rows: Default::default(),
            caption: None,
        }
    }

    pub fn has_columns(&self) -> bool {
        !self.columns.is_empty()
    }

    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column)
    }

    pub fn add_columns(&mut self, columns: &[Column]) {
        self.columns.extend_from_slice(columns)
    }

    pub fn has_rows(&self) -> bool {
        !self.rows.is_empty()
    }

    pub fn rows(&self) -> &Vec<Row> {
        &self.rows
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row)
    }

    pub fn add_rows(&mut self, rows: &[Row]) {
        self.rows.extend_from_slice(rows)
    }
}

// ------------------------------------------------------------------------------------------------

impl From<String> for Column {
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}

impl From<&str> for Column {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<(&str, Alignment)> for Column {
    fn from(v: (&str, Alignment)) -> Self {
        Self::from(&v)
    }
}

impl From<&(&str, Alignment)> for Column {
    fn from(v: &(&str, Alignment)) -> Self {
        Self::with_alignment(v.0, v.1.clone())
    }
}

impl Column {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            alignment: Default::default(),
        }
    }

    pub fn with_alignment(label: &str, alignment: Alignment) -> Self {
        Self {
            label: label.to_string(),
            alignment,
        }
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn alignment(&self) -> &Alignment {
        &self.alignment
    }
}

// ------------------------------------------------------------------------------------------------

impl Row {
    pub fn new(cells: &[Cell]) -> Self {
        Self {
            cells: cells.to_vec(),
        }
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell)
    }

    pub fn add_cells(&mut self, cells: &[Cell]) {
        self.cells.extend_from_slice(cells)
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Cell {
    fn default() -> Self {
        Cell::skip()
    }
}

has_inline_impls!(Cell);

impl Cell {
    pub fn skip() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub fn empty() -> Self {
        Self {
            inner: vec![Character::Space.into()],
        }
    }
}
