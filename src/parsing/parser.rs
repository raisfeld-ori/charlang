use pest::iterators::Pair;
use pest::Parser;
use std::error::Error;

use crate::parsing::types::*;

impl CharParser {
    fn parse_type(pair: Pair<Rule>) -> Type {
        match pair.as_rule() {
            Rule::typing => {
                let mut inner = pair.into_inner();
                let primitive = inner.next().expect("Type must have a primitive type");
                let mut base_type = Self::parse_type(primitive);
                
                // Handle type suffixes (arrays)
                for suffix in inner {
                    match suffix.as_rule() {
                        Rule::array_suffix => {
                            base_type = Type::Array(Box::new(base_type.clone()));
                        }
                        _ => {
                            println!("Unexpected rule in type suffix: {:?}", suffix.as_rule()); // Debug print
                            unreachable!()
                        }
                    }
                }
                base_type
            }
            Rule::identifier => {
                return Type::Struct(pair.as_str().to_string());
            }
            _ => {
                panic!("Unexpected rule in parse_type: {:?}", pair.as_rule())
            }
        }
    }

    fn parse_unary_operator(pair: Pair<Rule>) -> UnaryOperator {
        match pair.as_str() {
            "-" => UnaryOperator::Negate,
            "!" => UnaryOperator::Not,
            "~" => UnaryOperator::BitwiseNot,
            _ => panic!("Unexpected unary operator: {}", pair.as_str())
        }
    }
    
    fn parse_expression(pair: Pair<Rule>) -> ExpressionDecl {
        match pair.as_rule() {
            Rule::expression => {
                let mut expressions = pair.into_inner().map(Self::parse_expression).collect::<Vec<_>>();
                if expressions.len() == 1 {
                    expressions.remove(0)
                } else {
                    // Handle comma expressions
                    let last = expressions.pop().unwrap();
                    expressions.into_iter().fold(last, |acc, expr| {
                        ExpressionDecl::BinaryOp(
                            BinaryOperator::Add, // Temporary, should be comma operator
                            Box::new(expr),
                            Box::new(acc)
                        )
                    })
                }
            }
            Rule::struct_declaration => {
                let mut inner = pair.into_inner();
                let name = inner.next().expect("Missing struct name").as_str().to_string();
                let fields = inner.next().expect("Struct fields missing").into_inner();
                let fields = fields.map(|t| FieldDecl {
                    type_info: Self::parse_type(t.clone().into_inner().next().expect("Field type missing")),
                    name: t.into_inner().nth(1).expect("Field name missing").as_str().to_string(),
                }).collect();
                ExpressionDecl::Struct(name, fields)
            }
            Rule::assignment_expression => {
                let mut inner = pair.into_inner();
                let left = Self::parse_expression(inner.next().expect("Missing left side of assignment"));
                let operation = inner.next();
                if let Some(right) = inner.next() {
                    let right = Self::parse_expression(right);
                    if operation.is_some() {
                        match operation.unwrap().as_str() {
                            "=" => return ExpressionDecl::Assignment(Box::new(left), Box::new(right)),
                            "-" => return ExpressionDecl::BinaryOp(BinaryOperator::Subtract, Box::new(left), Box::new(right)),
                            "+" => return ExpressionDecl::BinaryOp(BinaryOperator::Add, Box::new(left), Box::new(right)),
                            _ => {}
                        }
                    }
                    return ExpressionDecl::Assignment(Box::new(left), Box::new(right));
                } else {
                    left
                }
            }
            Rule::postfix_expression => {
                let mut inner = pair.into_inner();
                let base = Self::parse_expression(inner.next().expect("Missing base expression"));
                let mut result = base;
                
                for op in inner {
                    match op.as_rule() {
                        Rule::array_access => {
                            let index = Self::parse_expression(op.into_inner().next().expect("Missing array index"));
                            result = ExpressionDecl::ArrayAccess(Box::new(result), Box::new(index));
                        }
                        Rule::function_call => {
                            let args = op.into_inner()
                                .map(Self::parse_expression)
                                .collect();
                            result = ExpressionDecl::Call(Box::new(result), args);
                        }
                        Rule::member_access => {
                            let member = op.into_inner().next().expect("Missing member name").as_str().to_string();
                            result = ExpressionDecl::MemberAccess(Box::new(result), member);
                        }
                        _ => panic!("Unexpected postfix operator: {:?}", op.as_rule())
                    }
                }
                result
            }
            Rule::array_literal => {
                let elements: Vec<ExpressionDecl> = pair.into_inner()
                    .map(Self::parse_expression)
                    .collect();
                ExpressionDecl::ArrayLiteral(elements)
            }
            Rule::identifier => ExpressionDecl::Identifier(pair.as_str().to_string()),
            Rule::number => {
                let num_str = pair.as_str();
                if num_str.contains('.') {
                    ExpressionDecl::Literal(Literal::Float(num_str.parse().unwrap()))
                } else {
                    ExpressionDecl::Literal(Literal::Integer(num_str.parse().unwrap()))
                }
            }
            Rule::string => {
                let content = pair.as_str();
                ExpressionDecl::Literal(Literal::String(content[1..content.len()-1].to_string()))
            }
            Rule::char => {
                let content = pair.as_str();
                ExpressionDecl::Literal(Literal::Char(content[1..content.len()-1].parse().unwrap()))
            }
            Rule::conditional_expression => {
                let mut inner = pair.into_inner();
                return Self::parse_expression(inner.next().expect("Missing condition"));
            }
            Rule::logical_or_expression => {
                let mut inner = pair.into_inner();
                let mut result = Self::parse_expression(inner.next().expect("Missing first operand"));
                while let Some(right) = inner.next() {
                    result = ExpressionDecl::BinaryOp(BinaryOperator::Or, Box::new(result), Box::new(Self::parse_expression(right)));
                }
                result
            }
            Rule::logical_and_expression => {
                let mut inner = pair.into_inner();
                let mut result = Self::parse_expression(inner.next().expect("Missing first operand"));
                while let Some(right) = inner.next() {
                    result = ExpressionDecl::BinaryOp(BinaryOperator::And, Box::new(result), Box::new(Self::parse_expression(right)));
                }
                result
            }
            Rule::equality_expression => {
                let mut inner = pair.into_inner();
                let mut result = Self::parse_expression(inner.next().expect("Missing first operand"));
                while let Some(op) = inner.next() {
                    let right = Self::parse_expression(inner.next().expect("Missing right operand"));
                    let operator = match op.as_str() {
                        "==" => BinaryOperator::Equal,
                        "!=" => BinaryOperator::NotEqual,
                        _ => panic!("Unexpected equality operator: {}", op.as_str())
                    };
                    result = ExpressionDecl::BinaryOp(operator, Box::new(result), Box::new(right));
                }
                result
            }
            Rule::relational_expression => {
                let mut inner = pair.into_inner();
                let mut result = Self::parse_expression(inner.next().expect("Missing first operand"));
                while let Some(op) = inner.next() {
                    let right = Self::parse_expression(inner.next().expect("Missing right operand"));
                    let operator = match op.as_str() {
                        "<" => BinaryOperator::Less,
                        "<=" => BinaryOperator::LessEqual,
                        ">" => BinaryOperator::Greater,
                        ">=" => BinaryOperator::GreaterEqual,
                        _ => panic!("Unexpected relational operator: {}", op.as_str())
                    };
                    result = ExpressionDecl::BinaryOp(operator, Box::new(result), Box::new(right));
                }
                result
            }
            Rule::additive_expression => {
                let mut inner = pair.into_inner();
                let result = Self::parse_expression(inner.next().expect("Missing first operand"));
                while let Some(op) = inner.next() {
                    let expr = Self::parse_expression(op.clone());
                    return ExpressionDecl::BinaryOp(BinaryOperator::Add, Box::new(result), Box::new(expr));
                }
                result
            }
            Rule::multiplicative_expression => {
                let mut inner = pair.into_inner();
                let left = Self::parse_expression(inner.next().expect("Missing first operand"));
                while let Some(op) = inner.next() {
                    let right = Self::parse_expression(inner.next().expect("Missing right operand"));
                    let operator = match op.as_str() {
                        "*" => BinaryOperator::Multiply,
                        "/" => BinaryOperator::Divide,
                        "%" => BinaryOperator::Modulo,
                        _ => panic!("Unexpected multiplicative operator: {}", op.as_str())
                    };
                    return ExpressionDecl::BinaryOp(operator, Box::new(left), Box::new(right));
                }
                left
            }
            Rule::unary_expression => {
                let mut inner = pair.into_inner();
                if inner.peek().unwrap().as_rule() == Rule::postfix_expression{
                    let expr = Self::parse_expression(inner.next().expect("Missing expression in unary operation"));
                    return ExpressionDecl::UnaryOp(UnaryOperator::Negate, Box::new(expr));
                }
                let op = inner.next().expect("Missing unary operator");
                let expr = Self::parse_expression(inner.next().expect("Missing expression in unary operation"));
                match op.as_rule() {
                    Rule::unary_operator => {
                        ExpressionDecl::UnaryOp(Self::parse_unary_operator(op), Box::new(expr))
                    }
                    _ => panic!("Unexpected rule in unary expression: {:?}", op.as_rule())
                }
            }
            Rule::unary_operator => {
                let op = pair.as_str();
                let expr = Self::parse_expression(pair.into_inner().next().expect("Missing expression in unary operator"));
                match op {
                    "-" => ExpressionDecl::UnaryOp(UnaryOperator::Negate, Box::new(expr)),
                    "!" => ExpressionDecl::UnaryOp(UnaryOperator::Not, Box::new(expr)),
                    "~" => ExpressionDecl::UnaryOp(UnaryOperator::BitwiseNot, Box::new(expr)),
                    _ => panic!("Unexpected unary operator: {}", op)
                }
            }
            Rule::primary_expression => {
                let mut inner = pair.into_inner();
                match inner.peek().unwrap().as_rule() {
                    Rule::number => {
                        let num_str = inner.next().unwrap().as_str();
                        if num_str.contains('.') {
                            return ExpressionDecl::Literal(Literal::Float(num_str.parse().unwrap()));
                        } else {
                            return ExpressionDecl::Literal(Literal::Integer(num_str.parse().unwrap()));
                        }
                    }
                    Rule::string => {
                        let content = inner.next().unwrap().as_str();
                        return ExpressionDecl::Literal(Literal::String(content[1..content.len()-1].to_string()));
                    }
                    Rule::char => {
                        let content = inner.next().unwrap().as_str();
                        return ExpressionDecl::Literal(Literal::Char(content[1..content.len()-1].parse().unwrap()));
                    }
                    Rule::identifier => {
                        return ExpressionDecl::Identifier(inner.next().unwrap().as_str().to_string());
                    }
                    Rule::array_literal => {
                        let elements: Vec<ExpressionDecl> = inner.next().unwrap().into_inner()
                            .map(Self::parse_expression)
                            .collect();
                        return ExpressionDecl::ArrayLiteral(elements);
                    }
                    _ => {
                        println!("primary_expression: {:?}", inner.next().unwrap().as_rule());
                        panic!("");
                    }
                }
            }
            Rule::argument_expression_list => {
                let mut inner = pair.into_inner();
                let mut result = Vec::new();
                while let Some(expr) = inner.next() {
                    result.push(Self::parse_expression(expr));
                }
                return ExpressionDecl::ArrayLiteral(result);
            }
            _ => {
                println!("Unexpected rule in parse_expression: {:?}", pair.as_rule());
                unreachable!()
            }
        }
    }

    fn parse_initializer(pair: Pair<Rule>) -> ExpressionDecl {
        match pair.as_rule() {
            Rule::initializer => {
                let inner = pair.into_inner().next().expect("Empty initializer");
                match inner.as_rule() {
                    Rule::array_literal => Self::parse_expression(inner),
                    _ => Self::parse_expression(inner),
                }
            }
            Rule::array_literal => {
                let elements: Vec<ExpressionDecl> = pair.into_inner()
                    .map(Self::parse_expression)
                    .collect();
                ExpressionDecl::ArrayLiteral(elements)
            }
            _ => panic!("Expected initializer, got {:?}", pair.as_rule())
        }
    }

    fn parse_statement(pair: Pair<Rule>) -> Statement {
        match pair.as_rule() {
            Rule::compound_statement => {
                let statements = pair.into_inner()
                    .map(Self::parse_statement)
                    .collect();
                Statement::Compound(statements)
            }
            Rule::if_statement => {
                let mut inner = pair.into_inner();
                let condition = Self::parse_expression(inner.next().expect("If condition missing"));
                let then_branch = Box::new(Self::parse_statement(inner.next().expect("If body missing")));
                let else_branch = inner.next()
                    .map(|p| Box::new(Self::parse_statement(p)));
                
                Statement::If(IfStatement {
                    condition,
                    then_branch,
                    else_branch,
                })
            }
            Rule::while_statement => {
                let mut inner = pair.into_inner();
                let condition = Self::parse_expression(inner.next().expect("While condition missing"));
                let body = Box::new(Self::parse_statement(inner.next().expect("While body missing")));
                
                Statement::While(WhileStatement {
                    condition,
                    body,
                })
            }
            Rule::for_statement => {
                let mut inner = pair.into_inner();
                let initializer = Box::new(Self::parse_statement(inner.next().expect("For initializer missing")));
                let condition = inner.next().map(Self::parse_expression);
                let increment = inner.next().map(Self::parse_expression);
                let body = Box::new(Self::parse_statement(inner.next().expect("For body missing")));
                
                Statement::For(ForStatement {
                    initializer,
                    condition,
                    increment,
                    body,
                })
            }
            Rule::do_while_statement => {
                let mut inner = pair.into_inner();
                let body = Box::new(Self::parse_statement(inner.next().expect("Do-while body missing")));
                let condition = Self::parse_expression(inner.next().expect("Do-while condition missing"));
                
                Statement::DoWhile(DoWhileStatement {
                    body,
                    condition,
                })
            }
            Rule::switch_statement => {
                let mut inner = pair.into_inner();
                let expression = Self::parse_expression(inner.next().expect("Switch expression missing"));
                let mut cases = Vec::new();
                let mut default = None;
                
                for case in inner {
                    match case.as_rule() {
                        Rule::case_statement => {
                            let mut case_inner = case.into_inner();
                            let value = Self::parse_expression(case_inner.next().expect("Case value missing"));
                            let statements = case_inner
                                .map(Self::parse_statement)
                                .collect();
                            cases.push(CaseStatement { value, statements });
                        }
                        Rule::default_statement => {
                            default = Some(case.into_inner()
                                .map(Self::parse_statement)
                                .collect());
                        }
                        _ => unreachable!("Unexpected rule in switch statement")
                    }
                }
                
                Statement::Switch(SwitchStatement {
                    expression,
                    cases,
                    default,
                })
            }
            Rule::return_statement => {
                let expr = pair.into_inner().next().map(Self::parse_expression);
                Statement::Return(expr)
            }
            Rule::break_statement => Statement::Break,
            Rule::continue_statement => Statement::Continue,
            Rule::expression_statement => {
                let expr = pair.into_inner().next()
                    .map(Self::parse_expression)
                    .unwrap_or(ExpressionDecl::Literal(Literal::Integer(0)));
                Statement::Expression(expr)
            }
            Rule::declaration_statement => {
                let mut inner = pair.into_inner();
                let type_info = Self::parse_type(inner.next().expect("Declaration type missing"));
                let declarations = inner.next().expect("Declaration list missing");
                
                // Handle multiple declarations in one statement
                let mut vars = Vec::new();
                for decl in declarations.into_inner() {
                    let mut decl_inner = decl.into_inner();
                    let name = decl_inner.next().expect("Variable name missing").as_str().to_string();
                    
                    let mut initializer = None;
                    
                    for item in decl_inner {
                        match item.as_rule() {
                            Rule::initializer => {
                                initializer = Some(Self::parse_initializer(item));
                            }
                            Rule::array_suffix => {
                            }
                            _ => panic!("Unexpected rule in declaration: {:?}", item.as_rule())
                        }
                    }
                    
                    vars.push(VariableDecl {
                        type_info: type_info.clone(),
                        name,
                        initializer,
                    });
                }
                
                // If there's only one declaration, return it directly
                if vars.len() == 1 {
                    Statement::Declaration(vars.remove(0))
                } else {
                    // If there are multiple declarations, wrap them in a compound statement
                    Statement::Compound(
                        vars.into_iter()
                            .map(Statement::Declaration)
                            .collect()
                    )
                }
            }
            _ => {
                panic!("Unexpected statement rule: {:?}", pair.as_rule());
            }
        }
    }

    fn parse_struct_declaration(pair: Pair<Rule>) -> Result<StructDecl, Box<dyn Error>> {
        let mut inner = pair.into_inner();
        let name = inner.next().expect("Struct name missing").as_str().to_string();
        let fields = inner.next().expect("Struct fields missing").into_inner();
        let fields = fields.map(|t| FieldDecl {
            type_info: Self::parse_type(t.clone().into_inner().next().expect("Field type missing")),
            name: t.into_inner().nth(1).expect("Field name missing").as_str().to_string(),
        }).collect();
        Ok(StructDecl { name, fields })
    }

    fn parse_function_declaration(pair: Pair<Rule>) -> Result<FunctionDecl, Box<dyn Error>> {
        let mut inner = pair.into_inner();
        
        // Parse return type
        let type_pair = inner.next().ok_or("Missing return type")?;
        let return_type = Self::parse_type(type_pair);
        
        // Parse function name
        let name = inner.next()
            .ok_or("Missing function name")?
            .as_str()
            .to_string();

        let params_exist = inner.len() > 1;
        
        let mut parameters = Vec::new();
        
        // Parse parameters if they exist
        if params_exist {
            if let Some(param_list) = inner.next() {
            if param_list.as_rule() == Rule::parameter_list {
                for param in param_list.into_inner() {
                    let mut param_inner = param.into_inner();
                    let type_info = Self::parse_type(param_inner.next().ok_or("Missing parameter type")?);
                    let name = param_inner.next().map(|p| p.as_str().to_string());
                    parameters.push(Parameter { type_info, name });
                }
            }
        }}
        // Parse function body
        let body = if let Some(body_pair) = inner.next() {
            match body_pair.as_rule() {
                Rule::compound_statement => {
                    let mut statements = Vec::new();
                    for stmt in body_pair.into_inner() {
                        let stmt = stmt.into_inner().next().unwrap();
                        match stmt.as_rule() {
                            Rule::declaration_statement |
                            Rule::expression_statement |
                            Rule::if_statement |
                            Rule::while_statement |
                            Rule::for_statement |
                            Rule::do_while_statement |
                            Rule::switch_statement |
                            Rule::return_statement |
                            Rule::break_statement |
                            Rule::continue_statement |
                            Rule::compound_statement => {
                                statements.push(Token::Statement(Self::parse_statement(stmt)));
                            }
                            Rule::function_declaration => {
                                statements.push(Token::Function(Self::parse_function_declaration(stmt)?));
                            }
                            Rule::struct_declaration => {
                                statements.push(Token::Struct(Self::parse_struct_declaration(stmt)?));
                            }
                            Rule::expression => {
                                statements.push(Token::Expression(Self::parse_expression(stmt)));
                            }
                            _ => panic!("Unexpected statement in function body: {:?}", stmt.as_rule())
                        }
                    }
                    statements
                }
                _ => panic!("Expected compound statement for function body, got {:?}", body_pair.as_rule())
            }
        } else {
            Vec::new()
        };
        
        Ok(FunctionDecl {
            return_type,
            name,
            parameters,
            body,
        })
    }
}

/// Parses C code into an intermediate representation (IR)
/// 
/// # Arguments
/// 
/// * `input` - A string slice containing C source code
/// 
/// # Returns
/// 
/// * `Result<Vec<Token>, Box<dyn Error>>` - A vector of IR tokens if successful, or an error if parsing fails
/// 
/// # Examples
/// 
/// ```
/// use charlang::parsing::parse;
/// 
/// let input = "int main() { return 0; }";
/// let tokens = parse(input).unwrap();
/// ```
/// 
/// The parser handles:
/// - Function declarations and definitions
/// - Variable declarations and initializations 
/// - Expressions and statements
/// - Control flow (if, while, for, etc)
/// - Type declarations
/// - Array and pointer types
/// 
/// The resulting IR tokens can be used for further compilation stages like type checking and code generation.
pub fn parse(input: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let pairs = CharParser::parse(Rule::program, input)?;
    let mut tokens = Vec::new();
    
    for pair in pairs.into_iter().next().ok_or("Empty program")?.into_inner() {
        match pair.as_rule() {
            Rule::function_declaration => {
                tokens.push(Token::Function(CharParser::parse_function_declaration(pair)?));
            }
            Rule::declaration_statement => {
                let statement = CharParser::parse_statement(pair);
                if let Statement::Declaration(var_decl) = statement {
                    tokens.push(Token::Variable(var_decl));
                }
            }
            Rule::expression => {
                let expression = CharParser::parse_expression(pair);
                tokens.push(Token::Expression(expression));
            }
            Rule::struct_declaration => {
                let struct_decl = CharParser::parse_struct_declaration(pair);
                if struct_decl.is_err() {return Err(struct_decl.err().unwrap())}
                tokens.push(Token::Struct(struct_decl.unwrap()));
            }
            Rule::EOI => {
                break;
            }
            _ => {
                println!("Unknown rule: {:?}", pair.as_rule());
                unreachable!()
            }
        }
    }
    
    Ok(tokens)
}
