use crate::{execution::{Input, Program, StdStruct, Value}, ir::{Literal, VariableData}};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Bool {
    pub value: bool,
}

impl StdStruct for Bool {
    fn get_fields(&self) -> Vec<Input> {
        vec![Input { name: "val".to_string(), value: Value::StdStruct(Arc::new(Bool { value: self.value })) }]
    }

    fn get_name(&self) -> String {
        "bool".to_string()
    }

    fn from_data(&mut self, _program: &mut Program, args: Vec<VariableData>) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Missing boolean value: value".to_string());
        }
        else if args.len() > 1 {
            return Err("Too many arguments: value".to_string());
        }
        else {
            let arg = args[0].clone();
            match arg {
                VariableData::Literal(Literal::Boolean(value)) => {
                    self.value = value;
                    Ok(())
                }
                _ => Err("Invalid argument: value".to_string()),
            }
        }
    }
    
    fn from_value(&mut self, _program: &mut Program, args: Vec<Value>) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Missing boolean value: value".to_string());
        }
        else if args.len() > 1 {
            return Err("Too many arguments: value".to_string());
        }
        else {
            let arg = args[0].clone();
            if arg.get_name() == "bool" {
                let value = arg.get_value();
                if let Some(bool_value) = value.as_bool() {
                    self.value = bool_value;
                    Ok(())
                } else {
                    Err("Invalid boolean value".to_string())
                }
            } else {
                Err("Invalid argument type: expected bool".to_string())
            }
        }
    }

    fn get_value(&self) -> serde_json::Value {
        return serde_json::Value::Bool(self.value);
    }
    fn add(&self, _program: &mut Program, _other: Value) -> Result<Value, String> {
        return Ok(Value::StdStruct(Arc::new(Bool { value: self.value && _other.get_value().as_bool().unwrap() })));
    }
    fn sub(&self, _program: &mut Program, _other: Value) -> Result<Value, String> {
        return Ok(Value::StdStruct(Arc::new(Bool { value: self.value || _other.get_value().as_bool().unwrap() })));
    }

    fn mul(&self, _program: &mut Program, _other: Value) -> Result<Value, String> {
        Err("Multiplication not supported for booleans".to_string())
    }

    fn div(&self, _program: &mut Program, _other: Value) -> Result<Value, String> {
        Err("Division not supported for booleans".to_string())
    }

    fn modulo(&self, _program: &mut Program, _other: Value) -> Result<Value, String> {
        Err("Modulo not supported for booleans".to_string())
    }

    fn eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "bool" {
            let other_bool = other.get_value().as_bool().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value == other_bool })))
        }
        else{
            Err("Invalid argument: bool".to_string())
        }
    }

    fn neq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "bool" {
            let other_bool = other.get_value().as_bool().unwrap();
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value != other_bool })))
        }
        else{
            Err("Invalid argument: bool".to_string())
        }
    }

    fn less(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "bool" {
            let other_bool = other.get_value().as_bool().unwrap();
            // false < true
            Ok(Value::StdStruct(Arc::new(Bool { value: !self.value && other_bool })))
        }
        else{
            Err("Invalid argument: bool".to_string())
        }
    }

    fn less_eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "bool" {
            let other_bool = other.get_value().as_bool().unwrap();
            // false <= true, false <= false, true <= true
            Ok(Value::StdStruct(Arc::new(Bool { value: !self.value || other_bool })))
        }
        else{
            Err("Invalid argument: bool".to_string())
        }
    }

    fn greater(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "bool" {
            let other_bool = other.get_value().as_bool().unwrap();
            // true > false
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value && !other_bool })))
        }
        else{
            Err("Invalid argument: bool".to_string())
        }
    }

    fn greater_eq(&self, _program: &mut Program, other: Value) -> Result<Value, String> {
        if other.get_name() == "bool" {
            let other_bool = other.get_value().as_bool().unwrap();
            // true >= true, true >= false, false >= false
            Ok(Value::StdStruct(Arc::new(Bool { value: self.value || !other_bool })))
        }
        else{
            Err("Invalid argument: bool".to_string())
        }
    }

    fn new_default() -> Self where Self: Sized {
        Bool {
            value: false,
        }
    }

    fn clone_with_value(&self, program: &mut Program, value: VariableData) -> Result<Arc<dyn StdStruct>, String> {
        let mut new_bool = Bool::new_default();
        new_bool.from_data(program, vec![value])?;
        Ok(Arc::new(new_bool))
    }
} 