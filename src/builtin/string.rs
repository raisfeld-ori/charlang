use crate::{execution::{Input, Program, StdStruct, Value}, ir::{Literal, VariableData}};
use std::sync::Arc;
use crate::builtin::Bool;

#[derive(Debug, Clone, PartialEq)]
pub struct StringType {
    pub value: String,
}

impl StdStruct for StringType {
    fn get_fields(&self) -> Vec<Input> {
        vec![Input { name: "val".to_string(), value: Value::StdStruct(Arc::new(StringType { value: self.value.clone() })) }]
    }

    fn get_name(&self) -> String {
        "string".to_string()
    }

    fn from_data(&mut self, _program: &mut Program, args: Vec<VariableData>) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Missing string value: value".to_string());
        }
        else if args.len() > 1 {
            return Err("Too many arguments: value".to_string());
        }
        else {
            let arg = args[0].clone();
            match arg {
                VariableData::Literal(Literal::String(value)) => {
                    self.value = value;
                    Ok(())
                }
                _ => Err("Invalid argument: value".to_string()),
            }
        }
    }
    
    fn from_value(&mut self, _program: &mut Program, args: Vec<Value>) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Missing string value: value".to_string());
        }
        else if args.len() > 1 {
            return Err("Too many arguments: value".to_string());
        }
        else {
            let arg = args[0].clone();
            if arg.get_name() == "string" {
                let value = arg.get_value();
                if let Some(string) = value.as_str() {
                    self.value = string.to_string();
                    Ok(())
                } else {
                    Err("Invalid string value".to_string())
                }
            } else {
                Err("Invalid argument type: expected string".to_string())
            }
        }
    }

    fn get_value(&self) -> serde_json::Value {
        return serde_json::Value::String(self.value.clone());
    }
    fn add(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "string" {
            let other_string = other.get_value().as_str().unwrap().to_string();
            Ok(Value::StdStruct(Arc::new(StringType { value: format!("{}{}", self.value, other_string) })))
        }
        else{
            Err("Invalid argument: string".to_string())
        }
    }
    fn sub(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_value().as_i64().unwrap_or(-1) == 0 {
            return Ok(Value::StdStruct(Arc::new(StringType { value: self.value.clone() })));
        }
        Err("Subtraction not supported for strings".to_string())
    }

    fn mul(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let count = other.get_value().as_i64().unwrap();
            if count < 0 {
                return Err("Cannot multiply string by negative number".to_string());
            }
            Ok(Value::StdStruct(Arc::new(StringType { value: self.value.repeat(count as usize) })))
        }
        else{
            Err("Invalid argument: expected integer for string multiplication".to_string())
        }
    }

    fn div(&self, _program: &mut Program, _other: Value) -> Result<Value, String> {
        Err("Division not supported for strings".to_string())
    }

    fn modulo(&self, _program: &mut Program, _other: Value) -> Result<Value, String> {
        Err("Modulo not supported for strings".to_string())
    }

    fn eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "string" {
            let value = other.get_value();
            let other_string = value.as_str().unwrap().to_string();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value == other_string })))
        }
        else{
            Err("Invalid argument: string".to_string())
        }
    }

    fn neq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "string" {
            let value = other.get_value();
            let other_string = value.as_str().unwrap().to_string();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value != other_string })))
        }
        else{
            Err("Invalid argument: string".to_string())
        }
    }

    fn less(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "string" {
            let other_string = other.get_value().as_str().unwrap().to_string();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value < other_string })))
        }
        else{
            Err("Invalid argument: string".to_string())
        }
    }

    fn less_eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "string" {
            let other_string = other.get_value().as_str().unwrap().to_string();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value <= other_string })))
        }
        else{
            Err("Invalid argument: string".to_string())
        }
    }

    fn greater(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "string" {
            let other_string = other.get_value().as_str().unwrap().to_string();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value > other_string })))
        }
        else{
            Err("Invalid argument: string".to_string())
        }
    }

    fn greater_eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "string" {
            let other_string = other.get_value().as_str().unwrap().to_string();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value >= other_string })))
        }
        else{
            Err("Invalid argument: string".to_string())
        }
    }

    fn new_default() -> Self where Self: Sized {
        StringType {
            value: String::new(),
        }
    }

    fn clone_with_value(&self, program: &mut Program, value: VariableData) -> Result<Arc<dyn StdStruct>, String> {
        let mut new_string = StringType::new_default();
        new_string.from_data(program, vec![value])?;
        Ok(Arc::new(new_string))
    }
} 