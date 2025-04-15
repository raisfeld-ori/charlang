use crate::{execution::{Input, Program, StdStruct, Value}, ir::{Literal, VariableData}};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Int {
    pub number: i64,
}

impl StdStruct for Int {
    fn get_fields(&self) -> Vec<Input> {
        vec![Input { name: "val".to_string(), value: Value::StdStruct(Arc::new(Int { number: self.number })) }]
    }

    fn get_name(&self) -> String {
        "int".to_string()
    }

    fn from_data(&mut self, _program: &mut Program, args: Vec<VariableData>) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Missing integer value: number".to_string());
        }
        else if args.len() > 1 {
            return Err("Too many arguments: number".to_string());
        }
        else {
            let arg = args[0].clone();
            match arg {
                VariableData::Literal(Literal::Integer(number)) => {
                    self.number = number;
                    Ok(())
                }
                _ => Err("Invalid argument: number".to_string()),
            }
        }
    }
    
    fn from_value(&mut self, _program: &mut Program, args: Vec<Value>) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Missing integer value: number".to_string());
        }
        else if args.len() > 1 {
            return Err("Too many arguments: number".to_string());
        }
        else {
            let arg = args[0].clone();
            if arg.get_name() == "int" {
                let value = arg.get_value();
                if let Some(number) = value.as_i64() {
                    self.number = number;
                    Ok(())
                } else {
                    Err("Invalid integer value".to_string())
                }
            } else {
                Err("Invalid argument type: expected int".to_string())
            }
        }
    }

    fn get_value(&self) -> serde_json::Value {
        return serde_json::Value::Number(self.number.into());
    }
    fn add(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            Ok(Value::StdStruct(Arc::new(Int { number: self.number + other_int })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }
    fn sub(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            Ok(Value::StdStruct(Arc::new(Int { number: self.number - other_int })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn mul(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            Ok(Value::StdStruct(Arc::new(Int { number: self.number * other_int })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn div(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            if other_int == 0 {
                return Err("Division by zero".to_string());
            }
            Ok(Value::StdStruct(Arc::new(Int { number: self.number / other_int })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn modulo(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            if other_int == 0 {
                return Err("Modulo by zero".to_string());
            }
            Ok(Value::StdStruct(Arc::new(Int { number: self.number % other_int })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            Ok(Value::StdStruct(Arc::new(Int { number: if self.number == other_int { 1 } else { 0 } })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn neq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            Ok(Value::StdStruct(Arc::new(Int { number: if self.number != other_int { 1 } else { 0 } })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn less(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            Ok(Value::StdStruct(Arc::new(Int { number: if self.number < other_int { 1 } else { 0 } })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn less_eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            Ok(Value::StdStruct(Arc::new(Int { number: if self.number <= other_int { 1 } else { 0 } })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn greater(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            Ok(Value::StdStruct(Arc::new(Int { number: if self.number > other_int { 1 } else { 0 } })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn greater_eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "int" {
            let other_int = other.get_value().as_i64().unwrap();
            Ok(Value::StdStruct(Arc::new(Int { number: if self.number >= other_int { 1 } else { 0 } })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn new_default() -> Self where Self: Sized {
        Int {
            number: 0,
        }
    }

    fn clone_with_value(&self, program: &mut Program, value: VariableData) -> Result<Arc<dyn StdStruct>, String> {
        let mut new_int = Int::new_default();
        new_int.from_data(program, vec![value])?;
        Ok(Arc::new(new_int))
    }
} 