use crate::execution::{Input, Program, StdFunction, Value};
use crate::ir::{Literal, VariableData};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct ToString {

}

impl StdFunction for ToString {
    fn run(&self, program: &mut Program, args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("Invalid number of arguments".to_string());
        }
        let string = program.get_value(String::from("string"));
        match string {
            Value::Struct(string) => {
                let string_value = args[0].get_value().to_string();
                let string_clone = string.clone();
                let mut new_struct = (*string_clone).clone();
                new_struct.value = serde_json::Value::String(string_value);
                return Ok(Value::Struct(Arc::new(new_struct)));
            }
            Value::StdStruct(string) => {
                let string_value = args[0].get_value().to_string();
                let result = string.clone_with_value(program, VariableData::Literal(Literal::String(string_value)))?;
                return Ok(Value::StdStruct(result));
            }
            _ => Err("String not found".to_string()),
        }
    }
    fn get_name(&self) -> String {
        "toString".to_string()
    }
    fn get_parameters(&self, program: &mut Program) -> Vec<Input> {
        let string = program.get_value(String::from("string"));
        vec![Input { name: "value".to_string(), value: string }]
    }
    fn new() -> Self {
        Self {}
    }
}

