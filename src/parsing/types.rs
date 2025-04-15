use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/parsing/grammar.pest"]
pub struct CharParser;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Function(FunctionDecl),
    Variable(VariableDecl),
    Statement(Statement),
    Expression(ExpressionDecl),
    Type(Type),
    Struct(StructDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<FieldDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDecl {
    pub type_info: Type,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub return_type: Type,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub type_info: Type,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecl {
    pub type_info: Type,
    pub name: String,
    pub initializer: Option<ExpressionDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Array(Box<Type>),
    Struct(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Compound(Vec<Statement>),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    DoWhile(DoWhileStatement),
    Switch(SwitchStatement),
    Return(Option<ExpressionDecl>),
    Break,
    Continue,
    Expression(ExpressionDecl),
    Declaration(VariableDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: ExpressionDecl,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: ExpressionDecl,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub initializer: Box<Statement>,
    pub condition: Option<ExpressionDecl>,
    pub increment: Option<ExpressionDecl>,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoWhileStatement {
    pub body: Box<Statement>,
    pub condition: ExpressionDecl,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchStatement {
    pub expression: ExpressionDecl,
    pub cases: Vec<CaseStatement>,
    pub default: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaseStatement {
    pub value: ExpressionDecl,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ExpressionDecl {
    Literal(Literal),
    Identifier(String),
    BinaryOp(BinaryOperator, Box<ExpressionDecl>, Box<ExpressionDecl>),
    UnaryOp(UnaryOperator, Box<ExpressionDecl>),
    Call(Box<ExpressionDecl>, Vec<ExpressionDecl>),
    Cast(Type, Box<ExpressionDecl>),
    ArrayAccess(Box<ExpressionDecl>, Box<ExpressionDecl>),
    MemberAccess(Box<ExpressionDecl>, String),
    Assignment(Box<ExpressionDecl>, Box<ExpressionDecl>),
    Conditional(Box<ExpressionDecl>, Box<ExpressionDecl>, Box<ExpressionDecl>), // ternary operator
    ArrayLiteral(Vec<ExpressionDecl>),
    Struct(String, Vec<FieldDecl>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Logical
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum UnaryOperator {
    Negate,
    Not,
    BitwiseNot,
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
} 