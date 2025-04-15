use crate::{execution::{Input, Program, StdStruct, Value}, ir::{Literal, VariableData}};
use std::sync::Arc;
use crate::builtin::{Bool, StringType};

#[derive(Debug, Clone, PartialEq)]
pub struct Char {
    pub value: char,
}

impl StdStruct for Char {
    fn get_fields(&self) -> Vec<Input> {
        vec![Input { name: "val".to_string(), value: Value::StdStruct(Arc::new(Char { value: self.value })) }]
    }

    fn get_name(&self) -> String {
        "char".to_string()
    }

    fn from_data(&mut self, _program: &mut Program, args: Vec<VariableData>) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Missing char value: value".to_string());
        }
        else if args.len() > 1 {
            return Err("Too many arguments: value".to_string());
        }
        else {
            let arg = args[0].clone();
            match arg {
                VariableData::Literal(Literal::Character(value)) => {
                    self.value = value;
                    Ok(())
                }
                _ => Err("Invalid argument: value".to_string()),
            }
        }
    }
    
    fn from_value(&mut self, _program: &mut Program, args: Vec<Value>) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Missing char value: value".to_string());
        }
        else if args.len() > 1 {
            return Err("Too many arguments: value".to_string());
        }
        else {
            let arg = args[0].clone();
            if arg.get_name() == "char" {
                let value = arg.get_value();
                if let Some(string) = value.as_str() {
                    if string.chars().count() == 1 {
                        self.value = string.chars().next().unwrap();
                        Ok(())
                    } else {
                        Err("Invalid char value: expected a single character".to_string())
                    }
                } else {
                    Err("Invalid char value".to_string())
                }
            } else {
                Err("Invalid argument type: expected char".to_string())
            }
        }
    }

    fn get_value(&self) -> serde_json::Value {
        return serde_json::Value::String(self.value.to_string());
    }
    fn add(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "char" {
            let value = other.get_value();
            let other_str = value.as_str().unwrap();
            let other_char = other_str.chars().next().unwrap();
            Ok(Value::StdStruct(Arc::new(StringType { value: format!("{}{}", self.value, other_char) })))
        }
        else{
            Err("Invalid argument: char".to_string())
        }
    }
    fn sub(&self, _program: &mut Program, _other: Value) -> Result<Value, String> {
        Err("Subtraction not supported for chars".to_string())
    }

    fn mul(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let count = other.get_value().as_i64().unwrap();
            if count < 0 {
                return Err("Cannot multiply char by negative number".to_string());
            }
            Ok(Value::StdStruct(Arc::new(StringType { value: self.value.to_string().repeat(count as usize) })))
        }
        else{
            Err("Invalid argument: expected integer for char multiplication".to_string())
        }
    }

    fn div(&self, _program: &mut Program, _other: Value) -> Result<Value, String> {
        Err("Division not supported for chars".to_string())
    }

    fn modulo(&self, _program: &mut Program, _other: Value) -> Result<Value, String> {
        Err("Modulo not supported for chars".to_string())
    }

    fn eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "char" {
            let other_char = other.get_value().as_str().unwrap().chars().next().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value == other_char })))
        }
        else{
            Err("Invalid argument: char".to_string())
        }
    }

    fn neq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "char" {
            let other_char = other.get_value().as_str().unwrap().chars().next().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value != other_char })))
        }
        else{
            Err("Invalid argument: char".to_string())
        }
    }

    fn less(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "char" {
            let other_char = other.get_value().as_str().unwrap().chars().next().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value < other_char })))
        }
        else{
            Err("Invalid argument: char".to_string())
        }
    }

    fn less_eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "char" {
            let other_char = other.get_value().as_str().unwrap().chars().next().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value <= other_char })))
        }
        else{
            Err("Invalid argument: char".to_string())
        }
    }

    fn greater(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "char" {
            let other_char = other.get_value().as_str().unwrap().chars().next().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value > other_char })))
        }
        else{
            Err("Invalid argument: char".to_string())
        }
    }

    fn greater_eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "char" {
            let other_char = other.get_value().as_str().unwrap().chars().next().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value >= other_char })))
        }
        else{
            Err("Invalid argument: char".to_string())
        }
    }

    fn new_default() -> Self where Self: Sized {
        Char {
            value: '\0',
        }
    }

    fn clone_with_value(&self, program: &mut Program, value: VariableData) -> Result<Arc<dyn StdStruct>, String> {
        let mut new_char = Char::new_default();
        new_char.from_data(program, vec![value])?;
        Ok(Arc::new(new_char))
    }
} 