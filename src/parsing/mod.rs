mod types;
mod parser;

// The parsing module does lexing, tokenizing and converts into basic IR
// Further actions are done in the IR module
pub use types::*;
pub use parser::parse;
