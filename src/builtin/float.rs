use crate::{execution::{Input, Program, StdStruct, Value}, ir::{Literal, VariableData}};
use std::sync::Arc;
use crate::builtin::Bool;

#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    pub number: f64,
}

impl StdStruct for Float {
    fn get_fields(&self) -> Vec<Input> {
        vec![Input { name: "val".to_string(), value: Value::StdStruct(Arc::new(Float { number: self.number })) }]
    }

    fn get_name(&self) -> String {
        "float".to_string()
    }

    fn from_data(&mut self, _program: &mut Program, args: Vec<VariableData>) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Missing float value: number".to_string());
        }
        else if args.len() > 1 {
            return Err("Too many arguments: number".to_string());
        }
        else {
            let arg = args[0].clone();
            match arg {
                VariableData::Literal(Literal::Float(number)) => {
                    self.number = number;
                    Ok(())
                }
                _ => Err("Invalid argument: number".to_string()),
            }
        }
    }
    
    fn from_value(&mut self, _program: &mut Program, args: Vec<Value>) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Missing float value: number".to_string());
        }
        else if args.len() > 1 {
            return Err("Too many arguments: number".to_string());
        }
        else {
            let arg = args[0].clone();
            if arg.get_name() == "float" {
                let value = arg.get_value();
                if let Some(number) = value.as_f64() {
                    self.number = number;
                    Ok(())
                } else {
                    Err("Invalid float value".to_string())
                }
            } else {
                Err("Invalid argument type: expected float".to_string())
            }
        }
    }

    fn get_value(&self) -> serde_json::Value {
        return serde_json::Value::Number(serde_json::Number::from_f64(self.number).unwrap());
    }
    fn add(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            Ok(Value::StdStruct(Arc::new(Float { number: self.number + other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }
    fn sub(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            Ok(Value::StdStruct(Arc::new(Float { number: self.number - other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn mul(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            Ok(Value::StdStruct(Arc::new(Float { number: self.number * other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn div(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            if other_float == 0.0 {
                return Err("Division by zero".to_string());
            }
            Ok(Value::StdStruct(Arc::new(Float { number: self.number / other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn modulo(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            if other_float == 0.0 {
                return Err("Modulo by zero".to_string());
            }
            Ok(Value::StdStruct(Arc::new(Float { number: self.number % other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.number == other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn neq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.number != other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn less(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.number < other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn less_eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.number <= other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn greater(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.number > other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn greater_eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "float" {
            let other_float = other.get_value().as_f64().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.number >= other_float })))
        }
        else{
            Err("Invalid argument: number".to_string())
        }
    }

    fn new_default() -> Self where Self: Sized {
        Float {
            number: 0.0,
        }
    }

    fn clone_with_value(&self, program: &mut Program, value: VariableData) -> Result<Arc<dyn StdStruct>, String> {
        let mut new_float = Float::new_default();
        new_float.from_data(program, vec![value])?;
        Ok(Arc::new(new_float))
    }
} 