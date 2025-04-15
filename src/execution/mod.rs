mod types;
mod traits;
mod program;
mod function;
#[allow(unused_imports)]
pub use types::{Value, Input, Function, Struct, Variable};
pub use traits::{StdFunction, StdStruct};
pub use program::Program;
