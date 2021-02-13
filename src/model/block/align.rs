#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Specifies the alignment of content within some container.
///
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub enum Alignment {
    /// Content is left aligned, right ragged.
    Left,
    /// Content is right aligned, left ragged.
    Right,
    /// Content is centered, left and right ragged.
    Centered,
    /// Content is justified, neither left or right ragged.
    Justified,
}

///
/// trait provided by content containers that support alignment.
///
pub trait HasAlignment {
    /// Return the alignment for this paragraph.
    fn alignment(&self) -> &Alignment;

    fn set_alignment(&mut self, alignment: Alignment) -> &mut Self;

    /// Set the alignment to left aligned, right ragged.
    fn set_left_aligned(&mut self) -> &mut Self {
        self.set_alignment(Alignment::Left)
    }

    /// Set the alignment to right aligned, left ragged.
    fn set_right_aligned(&mut self) -> &mut Self {
        self.set_alignment(Alignment::Right)
    }

    /// Set the alignment to right aligned, left ragged.
    fn set_ragged_left(&mut self) -> &mut Self {
        self.set_alignment(Alignment::Right)
    }

    /// Set the alignment to left aligned, right ragged.
    fn set_ragged_right(&mut self) -> &mut Self {
        self.set_alignment(Alignment::Left)
    }

    /// Set the alignment to centered, left and right ragged.
    fn set_centered(&mut self) -> &mut Self {
        self.set_alignment(Alignment::Centered)
    }

    /// Set the alignment to justified, neither left or right ragged.
    fn set_justified(&mut self) -> &mut Self {
        self.set_alignment(Alignment::Justified)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}
