use super::types::{Operator, Literal};

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub operator: Operator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Operation(Operation),
    Variable(String),
    FunctionCall(FunctionCall),
} 