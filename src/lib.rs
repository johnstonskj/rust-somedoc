/*!
One-line description.

More detailed description, with

# Example

*/

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
pub mod macros;

pub mod error;

pub mod model;

pub mod read;

pub mod write;
