#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    ArrayAccess,
    MemberAccess,
    Assignment,
    Conditional,
    Comma,
    Return,
    Break,
    Continue,
    Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Character(char),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub typing: Typing,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Typing {
    pub name: String,
    pub array_dimensions: usize,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum VariableData {
    Literal(Literal),
    StructInstance(String, Vec<(String, VariableData)>),
    Array(Vec<VariableData>),
    Expression(Box<super::expressions::Expression>),
    Null,
} 