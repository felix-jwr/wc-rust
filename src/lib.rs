mod cli;
mod input;

pub use cli::{InputArgs, resolve_defaults};
pub use input::{FileResult, read_input_as_bytes};
