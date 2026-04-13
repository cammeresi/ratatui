#![warn(missing_docs)]
//! A module for the [`Buffer`] and [`Cell`] types.

mod assert;
mod buffer;
mod cell;
mod cell_width;
mod diff;

pub use buffer::Buffer;
pub(crate) use cell::make_hyperlink;
pub use cell::{Cell, CellDiffOption};
pub use cell_width::CellWidth;
pub use diff::BufferDiff;
