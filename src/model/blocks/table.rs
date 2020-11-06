/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error;
use crate::model::blocks::BlockContent;
use crate::model::inline::HasInlineContent;
use crate::model::inline::{Character, InlineContent};
use crate::model::ComplexContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum Alignment {
    Default,
    Left,
    Right,
    Centered,
}

#[derive(Clone, Debug)]
pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Row>,
}

#[derive(Clone, Debug)]
pub struct Column {
    label: String,
    alignment: Alignment,
}

#[derive(Clone, Debug)]
pub struct Row {
    cells: Vec<Cell>,
}

#[derive(Clone, Debug)]
pub struct Cell {
    inner: Vec<InlineContent>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Default
    }
}

// ------------------------------------------------------------------------------------------------

block_impls!(Table);

impl Table {
    pub fn new(columns: &[Column]) -> Self {
        Self {
            columns: columns.to_vec(),
            rows: Default::default(),
        }
    }

    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
