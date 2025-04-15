use std::{fmt::Debug, sync::Arc};
use crate::ir::Action;

#[derive(Debug, Clone, PartialEq)]
pub struct Input{
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function{
    pub name: String,
    pub parameters: Vec<Input>,
    pub body: Vec<Action>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct{
    pub name: String,
    pub fields: Vec<Input>,
}

#[derive(Debug)]
pub enum Value{
    StdStruct(Arc<dyn StdStruct>),
    Struct(Arc<Struct>),
    Array(Vec<Value>),
    Null,
}

impl Clone for Value{
    fn clone(&self) -> Self {
        match self{
            Value::StdStruct(s) => Value::StdStruct(s.clone()),
            Value::Struct(s) => Value::Struct(s.clone()),
            Value::Array(a) => Value::Array(a.clone()),
            Value::Null => Value::Null,
        }
    }
}

impl Value{
    pub fn get_name(&self) -> String{
        match self{
            Value::StdStruct(s) => s.get_name(),
            Value::Struct(s) => s.name.clone(),
            Value::Array(_) => "Array".to_string(),
            Value::Null => "Null".to_string(),
        }
    }
    pub fn get_value(&self) -> serde_json::Value{
        match self{
            Value::StdStruct(s) => s.get_value(),
            Value::Struct(s) => s.fields.iter().map(|f| f.value.get_value()).collect(),
            Value::Array(a) => a.iter().map(|v| v.get_value()).collect(),
            Value::Null => serde_json::Value::Null,
        }
    }
}

impl PartialEq for Value{
    fn eq(&self, other: &Self) -> bool {
        match (self, other){
            (Value::StdStruct(s1), Value::StdStruct(s2)) => s1.get_name() == s2.get_name() && s1.get_fields() == s2.get_fields(),
            (Value::Struct(s1), Value::Struct(s2)) => s1 == s2,
            (Value::Array(a1), Value::Array(a2)) => a1 == a2,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable{
    pub name: String,
    pub typing: String,
    pub value: Value,
}

// Import the traits to avoid circular dependencies
#[allow(unused_imports)]
use super::traits::{StdStruct, StdFunction}; 