use std::fmt::Debug;
use crate::ir::VariableData;
use crate::execution::Program;
use super::types::{Value, Input};
use std::sync::Arc;

#[allow(dead_code)]
pub trait StdFunction: Debug {
    fn run(&self, program: &mut Program, args: Vec<Value>) -> Result<Value, String>;
    fn get_name(&self) -> String;
    fn get_parameters(&self, program: &mut Program) -> Vec<Input>;
    fn new() -> Self where Self: Sized;
}

pub trait StdStruct: Debug {
    fn from_data(&mut self, program: &mut Program, args: Vec<VariableData>) -> Result<(), String>;
    fn from_value(&mut self, program: &mut Program, args: Vec<Value>) -> Result<(), String>;
    fn new_default() -> Self where Self: Sized;
    fn get_fields(&self) -> Vec<Input>;
    fn get_name(&self) -> String;
    fn get_value(&self) -> serde_json::Value;
    fn add(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn sub(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn mul(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn div(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn modulo(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn eq(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn neq(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn less(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn less_eq(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn greater(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn greater_eq(&self, program: &mut Program, other: Value) -> Result<Value, String>;
    fn clone_with_value(&self, program: &mut Program, value: VariableData) -> Result<Arc<dyn StdStruct>, String>;
} 