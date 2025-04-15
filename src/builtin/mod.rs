mod int;
mod float;
mod string;
mod char;
mod bool;
mod to_string;

use std::sync::Arc;

#[allow(unused_imports)]
pub use int::Int;
#[allow(unused_imports)]
pub use float::Float;
#[allow(unused_imports)]
pub use string::StringType;
#[allow(unused_imports)]
pub use char::Char;
#[allow(unused_imports)]
pub use bool::Bool;
use to_string::ToString;

use crate::execution::{StdFunction, StdStruct};

pub fn get_std_lib() -> Vec<Arc<dyn StdStruct>> {
    vec![
        Arc::new(Int { number: 0 }),
        Arc::new(Float { number: 0.0 }),
        Arc::new(StringType { value: "".to_string() }),
        Arc::new(Char { value: ' ' }),
        Arc::new(Bool { value: false }),
    ]
}

pub fn get_std_functions() -> Vec<Arc<dyn StdFunction>> {
    vec![
        Arc::new(ToString::new()),
    ]
}

