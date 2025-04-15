use super::types::{Item, Typing, VariableData};
use super::expressions::{Expression, Operation};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub typing: Typing,
    pub data: VariableData
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Item>,
    pub body: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    pub condition: Expression,
    pub then_actions: Vec<Action>,
    pub else_actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Function(Function),
    Variable(Variable),
    Struct(Struct),
    Operation(Operation),
    Conditional(Conditional),
    Expression(Expression),
} 