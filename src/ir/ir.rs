use std::collections::HashMap;
use crate::parsing::{StructDecl, Token, FunctionDecl, VariableDecl, Statement, ExpressionDecl, Type};
use super::types::{Operator, Literal, Item, Typing, VariableData};
use super::expressions::{Expression, Operation, FunctionCall};
use super::actions::{Action, Function, Variable, Struct, Conditional};

/// The IR, aka "Intermediate Representation", is the intermediate representation of the source code.
/// It contains 4 parts:
/// - functions: the functions in the source code
/// - variables: the variables in the source code
/// - operations: the operations in the source code
/// - actions: the actions to be performed
#[derive(Debug, Clone, PartialEq)]
pub struct IR {
    functions: HashMap<String, Function>,
    variables: HashMap<String, Variable>,
    operations: HashMap<String, Operation>,
    structs: HashMap<String, Struct>,
    pub actions: Vec<Action>,
    expressions: Vec<Expression>,
}

impl IR {
    pub fn new() -> Self {
        IR {
            functions: HashMap::new(),
            variables: HashMap::new(),
            operations: HashMap::new(),
            structs: HashMap::new(),
            actions: Vec::new(),
            expressions: Vec::new(),
        }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        let mut ir = IR::new();
        ir.actions = ir.to_actions(tokens).unwrap();
        ir
    }

    pub fn from_actions(actions: Vec<Action>) -> Self {
        let mut ir = IR::new();
        ir.actions = actions;
        ir
    }

    pub fn to_actions(&self, tokens: Vec<Token>) -> Result<Vec<Action>, String> {
        let mut actions = Vec::new();
        for token in tokens {
            match token {
                Token::Struct(structure) => {
                    let action = self.from_struct(structure);
                    actions.push(action);
                }
                Token::Function(function) => {
                    let action = self.from_function(function);
                    if action.is_err() {
                        return Err(action.unwrap_err());
                    }
                    actions.push(action.unwrap());
                }
                Token::Variable(variable) => {
                    let action = self.from_variable(variable);
                    actions.push(action);
                }
                Token::Statement(statement) => {
                    let action = self.from_statement(statement);
                    actions.push(action);
                }
                Token::Expression(expression) => {
                    let expr = self.from_expression(expression)?;
                    actions.push(Action::Operation(Operation {
                        operator: Operator::Comma,
                        left: Box::new(expr),
                        right: Box::new(Expression::Literal(Literal::Integer(0))),
                    }));
                }
                Token::Type(_) => {
                    unreachable!("there should be no case where type should be parsed as an action");
                }
            }
        }
        Ok(actions)
    }

    fn from_struct(&self, structure: StructDecl) -> Action {
        // Convert struct declaration to IR struct
        let mut fields = Vec::new();
        
        // Process each field in the struct
        for field in structure.fields {
            let typing = match field.type_info {
                Type::Array(base_type) => {
                    // For array types, we need to count the dimensions
                    let mut array_dimensions = 1;
                    let mut current_type = *base_type;
                    
                    while let Type::Array(next_type) = current_type {
                        array_dimensions += 1;
                        current_type = *next_type;
                    }
                    
                    // Get the base type name
                    let type_name = match current_type {
                        Type::Struct(name) => name,
                        _ => panic!("Unsupported base type for array"),
                    };
                    
                    Typing {
                        name: type_name,
                        array_dimensions,
                    }
                },
                Type::Struct(name) => {
                    Typing {
                        name,
                        array_dimensions: 0,
                    }
                },
            };
            
            fields.push(Item {
                name: field.name,
                typing,
            });
        }
        
        // Create the struct
        let ir_struct = Struct {
            name: structure.name,
            fields,
        };
        
        Action::Struct(ir_struct)
    }

    fn from_function(&self, function: FunctionDecl) -> Result<Action, String> {
        // Convert function declaration to IR function
        let mut params = Vec::new();
        
        // Process each parameter in the function
        for param in &function.parameters {
            let typing = Typing {
                name: param.name.clone().unwrap_or_default(),
                array_dimensions: 0,
            };
            
            params.push(Item {
                name: param.name.clone().unwrap_or_default(),
                typing,
            });
        }
        
        // Process function body
        let body = self.to_actions(function.body);

        if body.is_err() {
            return Err(body.unwrap_err());
        }
        
        // Create the function
        Ok(Action::Function(Function {
            name: function.name.clone(),
            params,
            body: body.unwrap(),
        }))
    }

    fn from_variable(&self, variable: VariableDecl) -> Action {
        let data = match variable.initializer {
            Some(initializer) => {
                match self.from_expression(initializer) {
                    Ok(expr) => VariableData::Expression(Box::new(expr)),
                    Err(_) => VariableData::Null,
                }
            }
            None => VariableData::Null,
        };

        // Create the variable
        let ir_variable = Variable {
            name: variable.name,
            data,
        };
        
        Action::Variable(ir_variable)
    }

    fn from_statement(&self, statement: Statement) -> Action {
        match statement {
            Statement::If(if_stmt) => {
                // Convert the condition to an Expression
                let condition = match self.from_expression(if_stmt.condition) {
                    Ok(expr) => expr,
                    Err(_) => Expression::Literal(Literal::Boolean(false)),
                };
                
                // Process the then branch
                let mut then_actions = Vec::new();
                then_actions.push(self.from_statement(*if_stmt.then_branch));
                
                // Process the else branch if it exists
                let mut else_actions = Vec::new();
                if let Some(else_branch) = if_stmt.else_branch {
                    else_actions.push(self.from_statement(*else_branch));
                }
                
                Action::Conditional(Conditional {
                    condition,
                    then_actions,
                    else_actions,
                })
            },
            Statement::Return(ret_stmt) => {
                if let Some(expr) = ret_stmt {
                    let expr = match self.from_expression(expr) {
                        Ok(expr) => expr,
                        Err(_) => Expression::Literal(Literal::Integer(0)),
                    };
                    
                    Action::Operation(Operation {
                        operator: Operator::Return,
                        left: Box::new(expr),
                        right: Box::new(Expression::Literal(Literal::Integer(0))),
                    })
                } else {
                    Action::Operation(Operation {
                        operator: Operator::Return,
                        left: Box::new(Expression::Literal(Literal::Integer(0))),
                        right: Box::new(Expression::Literal(Literal::Integer(0))),
                    })
                }
            },
            Statement::Expression(expr_stmt) => {
                let expr = match self.from_expression(expr_stmt) {
                    Ok(expr) => expr,
                    Err(_) => Expression::Literal(Literal::Integer(0)),
                };
                
                Action::Operation(Operation {
                    operator: Operator::Expression,
                    left: Box::new(expr),
                    right: Box::new(Expression::Literal(Literal::Integer(0))),
                })
            },
            Statement::While(while_stmt) => {
                let condition = match self.from_expression(while_stmt.condition) {
                    Ok(expr) => expr,
                    Err(_) => Expression::Literal(Literal::Boolean(false)),
                };
                
                let mut body_actions = Vec::new();
                body_actions.push(self.from_statement(*while_stmt.body));
                
                Action::Conditional(Conditional {
                    condition,
                    then_actions: body_actions,
                    else_actions: Vec::new(),
                })
            },
            Statement::For(for_stmt) => {
                let mut init_actions = Vec::new();
                init_actions.push(self.from_statement(*for_stmt.initializer));
                
                let condition = match for_stmt.condition {
                    Some(expr) => match self.from_expression(expr) {
                        Ok(expr) => expr,
                        Err(_) => Expression::Literal(Literal::Boolean(true)),
                    },
                    None => Expression::Literal(Literal::Boolean(true)),
                };
                
                let mut increment_actions = Vec::new();
                if let Some(increment) = for_stmt.increment {
                    if let Ok(expr) = self.from_expression(increment) {
                        increment_actions.push(Action::Expression(expr));
                    }
                }
                
                let mut body_actions = Vec::new();
                body_actions.push(self.from_statement(*for_stmt.body));
                
                let mut all_actions = init_actions;
                all_actions.push(Action::Conditional(Conditional {
                    condition,
                    then_actions: body_actions,
                    else_actions: Vec::new(),
                }));
                all_actions.extend(increment_actions);
                
                if all_actions.is_empty() {
                    Action::Operation(Operation {
                        operator: Operator::Expression,
                        left: Box::new(Expression::Literal(Literal::Integer(0))),
                        right: Box::new(Expression::Literal(Literal::Integer(0))),
                    })
                } else {
                    all_actions.remove(0)
                }
            },
            Statement::DoWhile(do_while_stmt) => {
                let mut body_actions = Vec::new();
                body_actions.push(self.from_statement(*do_while_stmt.body));
                
                let condition = match self.from_expression(do_while_stmt.condition) {
                    Ok(expr) => expr,
                    Err(_) => Expression::Literal(Literal::Boolean(false)),
                };
                
                Action::Conditional(Conditional {
                    condition,
                    then_actions: body_actions,
                    else_actions: Vec::new(),
                })
            },
            Statement::Switch(switch_stmt) => {
                let expr = match self.from_expression(switch_stmt.expression) {
                    Ok(expr) => expr,
                    Err(_) => Expression::Literal(Literal::Integer(0)),
                };
                
                let mut case_actions = Vec::new();
                for case in switch_stmt.cases {
                    let case_value = match self.from_expression(case.value) {
                        Ok(expr) => expr,
                        Err(_) => Expression::Literal(Literal::Integer(0)),
                    };
                    
                    let mut body_actions = Vec::new();
                    for stmt in case.statements {
                        body_actions.push(self.from_statement(stmt));
                    }
                    
                    let case_condition = Operation {
                        operator: Operator::Equal,
                        left: Box::new(expr.clone()),
                        right: Box::new(case_value),
                    };
                    
                    case_actions.push(Action::Conditional(Conditional {
                        condition: Expression::Operation(case_condition),
                        then_actions: body_actions,
                        else_actions: Vec::new(),
                    }));
                }
                
                let mut default_actions = Vec::new();
                if let Some(default_statements) = switch_stmt.default {
                    for stmt in default_statements {
                        default_actions.push(self.from_statement(stmt));
                    }
                }
                
                if case_actions.is_empty() {
                    if default_actions.is_empty() {
                        Action::Operation(Operation {
                            operator: Operator::Expression,
                            left: Box::new(Expression::Literal(Literal::Integer(0))),
                            right: Box::new(Expression::Literal(Literal::Integer(0))),
                        })
                    } else {
                        default_actions.remove(0)
                    }
                } else {
                    case_actions.remove(0)
                }
            },
            Statement::Break => {
                Action::Operation(Operation {
                    operator: Operator::Break,
                    left: Box::new(Expression::Literal(Literal::Integer(0))),
                    right: Box::new(Expression::Literal(Literal::Integer(0))),
                })
            },
            Statement::Continue => {
                Action::Operation(Operation {
                    operator: Operator::Continue,
                    left: Box::new(Expression::Literal(Literal::Integer(0))),
                    right: Box::new(Expression::Literal(Literal::Integer(0))),
                })
            },
            Statement::Declaration(decl) => {
                self.from_variable(decl)
            },
            Statement::Compound(statements) => {
                let mut actions = Vec::new();
                for stmt in statements {
                    actions.push(self.from_statement(stmt));
                }
                
                if actions.is_empty() {
                    Action::Operation(Operation {
                        operator: Operator::Expression,
                        left: Box::new(Expression::Literal(Literal::Integer(0))),
                        right: Box::new(Expression::Literal(Literal::Integer(0))),
                    })
                } else {
                    actions.remove(0)
                }
            },
        }
    }

    fn from_expression(&self, expression: ExpressionDecl) -> Result<Expression, String> {
        match expression {
            ExpressionDecl::Literal(literal) => {
                match literal {
                    crate::parsing::Literal::Integer(i) => Ok(Expression::Literal(Literal::Integer(i))),
                    crate::parsing::Literal::Float(f) => Ok(Expression::Literal(Literal::Float(f))),
                    crate::parsing::Literal::String(s) => Ok(Expression::Literal(Literal::String(s))),
                    crate::parsing::Literal::Char(c) => Ok(Expression::Literal(Literal::Character(c))),
                }
            },
            ExpressionDecl::Identifier(name) => {
                Ok(Expression::Variable(name))
            },
            ExpressionDecl::BinaryOp(op, left, right) => {
                let left_expr = self.from_expression(*left)?;
                let right_expr = self.from_expression(*right)?;
                
                Ok(Expression::Operation(Operation {
                    operator: self.to_operator(&op),
                    left: Box::new(left_expr),
                    right: Box::new(right_expr),
                }))
            },
            ExpressionDecl::UnaryOp(op, expr) => {
                let operator = self.to_unary_operator(&op);
                let expr_result = self.from_expression(*expr)?;
                
                Ok(Expression::Operation(Operation {
                    operator,
                    left: Box::new(expr_result),
                    right: Box::new(Expression::Literal(Literal::Integer(0))),
                }))
            },
            ExpressionDecl::Call(func, args) => {
                let name = match *func {
                    ExpressionDecl::Identifier(name) => name,
                    _ => return Err("Function call must have an identifier".to_string()),
                };
                
                if let Some(_function) = self.lookup_function(&name) {
                    let mut processed_args = Vec::new();
                    for arg in args {
                        processed_args.push(self.from_expression(arg)?);
                    }
                    
                    Ok(Expression::FunctionCall(FunctionCall {
                        name,
                        args: processed_args,
                    }))
                } else {
                    let mut processed_args = Vec::new();
                    for arg in args {
                        processed_args.push(self.from_expression(arg)?);
                    }
                    
                    Ok(Expression::FunctionCall(FunctionCall {
                        name,
                        args: processed_args,
                    }))
                }
            },
            ExpressionDecl::Cast(_, expr) => {
                self.from_expression(*expr)
            },
            ExpressionDecl::ArrayAccess(array, index) => {
                let array_expr = self.from_expression(*array)?;
                let index_expr = self.from_expression(*index)?;
                
                Ok(Expression::Operation(Operation {
                    operator: Operator::ArrayAccess,
                    left: Box::new(array_expr),
                    right: Box::new(index_expr),
                }))
            },
            ExpressionDecl::MemberAccess(obj, member) => {
                let obj_expr = self.from_expression(*obj)?;
                
                Ok(Expression::Operation(Operation {
                    operator: Operator::MemberAccess,
                    left: Box::new(obj_expr),
                    right: Box::new(Expression::Literal(Literal::String(member))),
                }))
            },
            ExpressionDecl::Assignment(left, right) => {
                let left_expr = self.from_expression(*left)?;
                let right_expr = self.from_expression(*right)?;
                
                Ok(Expression::Operation(Operation {
                    operator: Operator::Assignment,
                    left: Box::new(left_expr),
                    right: Box::new(right_expr),
                }))
            },
            ExpressionDecl::Conditional(condition, then_expr, else_expr) => {
                let condition_expr = self.from_expression(*condition)?;
                let then_expr_result = self.from_expression(*then_expr)?;
                let else_expr_result = self.from_expression(*else_expr)?;
                
                Ok(Expression::Operation(Operation {
                    operator: Operator::Conditional,
                    left: Box::new(condition_expr),
                    right: Box::new(Expression::Operation(Operation {
                        operator: Operator::Comma,
                        left: Box::new(then_expr_result),
                        right: Box::new(else_expr_result),
                    })),
                }))
            },
            ExpressionDecl::ArrayLiteral(elements) => {
                let mut result = Expression::Literal(Literal::Integer(0));
                
                for element in elements.into_iter().rev() {
                    let element_expr = self.from_expression(element)?;
                    result = Expression::Operation(Operation {
                        operator: Operator::Comma,
                        left: Box::new(element_expr),
                        right: Box::new(result),
                    });
                }
                
                Ok(result)
            },
            ExpressionDecl::Struct(name, fields) => {
                self.from_expression(ExpressionDecl::Struct(name.clone(), fields))
            },
        }
    }

    fn to_operator(&self, op: &crate::parsing::BinaryOperator) -> Operator {
        match op {
            crate::parsing::BinaryOperator::Add => Operator::Add,
            crate::parsing::BinaryOperator::Subtract => Operator::Subtract,
            crate::parsing::BinaryOperator::Multiply => Operator::Multiply,
            crate::parsing::BinaryOperator::Divide => Operator::Divide,
            crate::parsing::BinaryOperator::Modulo => Operator::Modulo,
            crate::parsing::BinaryOperator::Equal => Operator::Equal,
            crate::parsing::BinaryOperator::NotEqual => Operator::NotEqual,
            crate::parsing::BinaryOperator::Less => Operator::Less,
            crate::parsing::BinaryOperator::LessEqual => Operator::LessEqual,
            crate::parsing::BinaryOperator::Greater => Operator::Greater,
            crate::parsing::BinaryOperator::GreaterEqual => Operator::GreaterEqual,
            crate::parsing::BinaryOperator::And => Operator::And,
            crate::parsing::BinaryOperator::Or => Operator::Or,
        }
    }

    fn to_unary_operator(&self, op: &crate::parsing::UnaryOperator) -> Operator {
        match op {
            crate::parsing::UnaryOperator::Negate => Operator::Subtract,
            crate::parsing::UnaryOperator::Not => Operator::NotEqual,
            _ => panic!("Unsupported unary operator: {:?}", op),
        }
    }

    pub fn lookup_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }
}

impl From<Vec<Token>> for IR {
    fn from(tokens: Vec<Token>) -> Self {
        IR::from_tokens(tokens)
    }
}

