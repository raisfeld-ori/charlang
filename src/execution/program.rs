use std::{collections::HashMap, fmt::{Debug, Display}, sync::Arc};
use crate::ir::{IR, Variable as IRVariable, VariableData, Function as IRFunction, Struct as IRStruct, Action, Literal, Expression, Operator};
use super::types::{Value, Input, Function, Struct, Variable};
use super::traits::{StdFunction, StdStruct};

#[derive(Debug)]
pub struct Program{
    pub std_functions: HashMap<String, Arc<dyn StdFunction>>,
    pub std_structs: HashMap<String, Arc<dyn StdStruct>>,
    pub functions: HashMap<String, Function>,
    pub structs: HashMap<String, Struct>,
    pub variables: HashMap<String, Variable>,
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Program {
            std_functions: self.std_functions.clone(),
            std_structs: self.std_structs.clone(),
            functions: self.functions.clone(),
            structs: self.structs.clone(),
            variables: self.variables.clone(),
        }
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        // Can't compare dyn traits, so we skip those fields
        self.functions == other.functions &&
        self.structs == other.structs &&
        self.variables == other.variables
    }
}

impl Program{
    pub fn new() -> Self{
        Program{
            std_functions: HashMap::new(),
            std_structs: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            variables: HashMap::new(),
        }
    }
    pub fn include_std_struct(&mut self, struct_: Arc<dyn StdStruct>){
        self.std_structs.insert(struct_.get_name(), struct_);
    }
    pub fn include_std_function(&mut self, function: Arc<dyn StdFunction>){
        self.std_functions.insert(function.get_name(), function);
    }
    pub fn include_std_library(&mut self, structs: Vec<Arc<dyn StdStruct>>, functions: Vec<Arc<dyn StdFunction>>){
        for struct_ in structs{
            self.include_std_struct(struct_);
        }
        for function in functions{
            self.include_std_function(function);
        }
    }
    pub fn run(&mut self, ir: &IR) -> Result<Value, String> {
        // Process each action in the IR
        for action in &ir.actions {
            match action {
                Action::Function(function) => {
                    // Process function declarations
                    if let Err(err) = self.run_function(function) {
                        return Err(err);
                    }
                },
                Action::Variable(variable) => {
                    // Process variable declarations
                    if let Err(err) = self.run_variable(variable) {
                        return Err(err);
                    }
                },
                Action::Struct(ir_struct) => {
                    // Process struct declarations
                    if let Err(err) = self.run_struct(ir_struct) {
                        return Err(err);
                    }
                },
                Action::Expression(expr) => {
                    let data = VariableData::Expression(Box::new(expr.clone()));
                    let value = self.extract_value(&data)?;
                    return Ok(value);
                }
                Action::Operation(operation) => {
                    let data = VariableData::Expression(Box::new(Expression::Operation(operation.clone())));
                    let value = self.extract_value(&data)?;
                    return Ok(value);
                },
                Action::Conditional(_conditional) => {
                    // Process conditional statements (if, while, etc.)
                    // This would be implemented in a more complete version
                    // For now, we'll just skip conditionals
                    unimplemented!()
                },
            }
        }
        // Return a success message
        Ok(Value::Null)
    }
    fn run_variable(&mut self, variable: &IRVariable) -> Result<(), String>{
        let type_valid = self.extract_value(&variable.data);
        if type_valid.is_err(){
            let type_valid = type_valid.unwrap_err();
            return Err(format!("On variable {}: {}", variable.name, type_valid));
        } 
        let type_valid = type_valid.unwrap();
        let variable = Variable{
            name: variable.name.clone(),
            value: type_valid,
        };
        self.variables.insert(variable.name.clone(), variable);
        Ok(())
    }
    fn run_function(&mut self, function: &IRFunction) -> Result<(), String> {
        // Check if the function already exists
        if self.functions.contains_key(&function.name) {
            return Err(format!("Function '{}' is already defined", function.name));
        }
        
        // Convert IR function parameters to execution function parameters
        let mut parameters = Vec::new();
        for param in &function.params {
            parameters.push(Input {
                name: param.name.clone(),
                value: Value::Null,
            });
        }

        let _ = IR::from_actions(function.body.clone());
        
        // Create the execution function
        let execution_function = Function {
            name: function.name.clone(),
            parameters,
            body: function.body.clone(),
        };
        
        // Add the function to the program
        self.functions.insert(execution_function.name.clone(), execution_function);
        
        Ok(())
    }
    fn run_struct(&mut self, ir_struct: &IRStruct) -> Result<(), String> {
        // Check if the struct already exists
        for existing_struct in self.structs.values() {
            if existing_struct.name == ir_struct.name {
                return Err(format!("Struct '{}' is already defined", ir_struct.name));
            }
        }
        for existing_struct in self.std_structs.values() {
            if existing_struct.get_name() == ir_struct.name {
                return Err(format!("Struct '{}' is already defined", ir_struct.name));
            }
        }
        
        // Convert IR struct fields to execution struct fields
        let mut fields = Vec::new();
        for field in &ir_struct.fields {
            fields.push(Input { 
                name: field.name.clone(),
                value: Value::Null,
            });
        }
        
        // Create the execution struct
        let execution_struct = Struct {
            name: ir_struct.name.clone(),
            fields,
            value: serde_json::Value::Null,
        };
        
        // Add the struct to the program
        self.structs.insert(execution_struct.name.clone(), execution_struct);
        
        Ok(())
    }
    fn extract_value(&mut self, values: &VariableData) -> Result<Value, String> {
        match values {
            VariableData::Literal(literal) => {
                match literal {
                    Literal::Integer(i) => {
                        let int_value = self.get_value(String::from("int"));
                        match int_value {
                            Value::StdStruct(s) => {
                                let result = s.clone_with_value(self, VariableData::Literal(Literal::Integer(*i)))?;
                                Ok(Value::StdStruct(result))
                            },
                            _ => Err("Unknown type: int".to_string()),
                        }
                    },
                    Literal::Float(f) => {
                        let float_value = self.get_value(String::from("float"));
                        match float_value {
                            Value::StdStruct(s) => {
                                let result = s.clone_with_value(self, VariableData::Literal(Literal::Float(*f)))?;
                                Ok(Value::StdStruct(result))
                            },
                            _ => Err("Unknown type: float".to_string()),
                        }
                    },
                    Literal::String(str) => {
                        let string_value = self.get_value(String::from("string"));
                        match string_value {
                            Value::StdStruct(s) => {
                                let result = s.clone_with_value(self, VariableData::Literal(Literal::String(str.clone())))?;
                                Ok(Value::StdStruct(result))
                            },
                            _ => Err("Unknown type: string".to_string()),
                        }
                    },
                    Literal::Character(c) => {
                        let char_value = self.get_value(String::from("char"));
                        match char_value {
                            Value::StdStruct(s) => {
                                let result = s.clone_with_value(self, VariableData::Literal(Literal::Character(*c)))?;
                                Ok(Value::StdStruct(result))
                            },
                            _ => Err("Unknown type: char".to_string()),
                        }
                    },
                    Literal::Boolean(b) => {
                        let bool_value = self.get_value(String::from("bool"));
                        match bool_value {
                            Value::StdStruct(s) => {
                                let result = s.clone_with_value(self, VariableData::Literal(Literal::Boolean(*b)))?;
                                Ok(Value::StdStruct(result))
                            },
                            _ => Err("Unknown type: bool".to_string()),
                        }
                    }
                }
            },
            VariableData::StructInstance(name, fields) => {
                let struct_def = self.structs.get(name);
                if let Some(struct_def) = struct_def {
                    // First collect all field information
                    let field_info: Vec<_> = fields.iter()
                        .filter_map(|(field_name, field_value)| {
                            struct_def.fields.iter()
                                .find(|f| f.name == **field_name)
                                .map(|_field| (field_name.clone(), field_value.clone()))
                        })
                        .collect();
                    
                    // Then process each field
                    let mut field_values = Vec::new();
                    for (field_name, field_value) in field_info {
                        let value = self.extract_value(&field_value)?;
                        field_values.push(Input {
                            name: field_name,
                            value,
                        });
                    }
                    
                    Ok(Value::Struct(Arc::new(Struct {
                        name: name.clone(),
                        fields: field_values,
                        value: serde_json::Value::Null,
                    })))
                } else {
                    Err(format!("Struct type {} not found", name))
                }
            },
            VariableData::Array(elements) => {
                let mut array = Vec::new();
                for element in elements{
                    let value = self.extract_value(&element)?;
                    array.push(value);
                }
                Ok(Value::Array(array))
            },
            VariableData::Expression(expr) => {
                match &**expr{
                    Expression::Literal(lit) => {
                        self.extract_value(&VariableData::Literal(lit.clone()))
                    },
                    Expression::Operation(op) => {
                        let left = self.extract_value(&VariableData::Expression(op.left.clone()))?;
                        let right = self.extract_value(&VariableData::Expression(op.right.clone()))?;
                        self.run_operation(&op.operator, left, right)
                    }
                    Expression::FunctionCall(func) => {
                        let function_name = func.name.clone();
                        let mut args = Vec::new();
                        for arg in &func.args {
                            args.push(self.extract_value(&VariableData::Expression(Box::new(arg.clone())))?);
                        }
                        if let Some(function) = self.functions.get(&function_name) {
                            let mut function_clone = function.clone();
                            let res = function_clone.run(self, args);
                            if res.is_err(){
                                return Err(res.unwrap_err());
                            }
                            Ok(res.unwrap())
                        }
                        else if let Some(std_function) = self.std_functions.get(&function_name){
                            let std_function_clone = std_function.clone();
                            let res = std_function_clone.run(self, args);
                            if res.is_err(){
                                return Err(res.unwrap_err());
                            }
                            Ok(res.unwrap())
                        }
                        else if let Some(struct_) = self.structs.get(&function_name){
                            let field_names: Vec<String> = struct_.fields.iter().map(|f| f.name.clone()).collect();
                            let mut fields = Vec::new();
                            for i in 0..field_names.len() {
                                let value = self.extract_value(&VariableData::Expression(Box::new(func.args[i].clone())))?;
                                fields.push(Input { name: field_names[i].clone(), value });
                            }
                            Ok(Value::Struct(Arc::new(Struct { 
                                name: function_name.clone(),
                                fields,
                                value: serde_json::Value::Null,
                            })))
                        }
                        else if let Some(std_struct) = self.std_structs.get(&function_name){
                            let mut std_struct_clone = std_struct.clone();
                            let res = if let Some(mut_struct) = Arc::get_mut(&mut std_struct_clone) {
                                mut_struct.from_value(self, args)
                            } else {
                                return Err("Cannot get mutable reference to Arc".to_string());
                            };
                            if res.is_err(){
                                return Err(res.unwrap_err());
                            }
                            Ok(Value::StdStruct(std_struct_clone))
                        }
                        else{
                            Err(format!("Function '{}' not found", function_name))
                        }
                    }
                    Expression::Variable(var) => {
                        let variable = self.variables.get(var);
                        if let Some(variable) = variable {
                            Ok(variable.value.clone())
                        } else{
                            Err(format!("Variable '{}' not found", var))
                        }
                    }

                }
            },
            VariableData::Null => Ok(Value::Null),
        }
    }
    #[allow(dead_code)]
    pub fn get_variable(&self, name: String) -> Option<&Variable>{
        self.variables.get(&name)
    }
    pub fn get_value(&self, name: String) -> Value{
        let std = self.std_structs.get(&name);
        if let Some(std) = std{
            return Value::StdStruct(std.clone());
        }
        let struct_ = self.structs.get(&name);
        if let Some(struct_) = struct_{
            return Value::Struct(Arc::new(struct_.clone()));
        }
        Value::Null
    }
    fn run_operation(&mut self, operator: &Operator, left: Value, right: Value) -> Result<Value, String>{
        match operator{
            Operator::Add => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.add(self, right)
                    }
                    _ => Err("Cannot add non-std structs".to_string()),
                }
            }
            Operator::Subtract => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.sub(self, right)
                    }
                    _ => {
                        if right.get_value().as_i64().unwrap_or(-1) == 0{
                            return Ok(left);
                        }
                        Err("Cannot subtract non-std structs".to_string())
                    },
                }
            }
            Operator::Multiply => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.mul(self, right)
                    }
                    _ => Err("Cannot multiply non-std structs".to_string()),
                }
            }
            Operator::Divide => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.div(self, right)
                    }
                    _ => Err("Cannot divide non-std structs".to_string()),
                }
            }
            Operator::Modulo => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.modulo(self, right)
                    }
                    _ => Err("Cannot modulo non-std structs".to_string()),
                }
            }
            Operator::Equal => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.eq(self, right)
                    }
                    _ => Err("Cannot compare non-std structs".to_string()),
                }
            }
            Operator::NotEqual => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.neq(self, right)
                    }
                    _ => Err("Cannot compare non-std structs".to_string()),
                }
            }
            Operator::Less => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.less(self, right)
                    }
                    _ => Err("Cannot compare non-std structs".to_string()),
                }
            }
            Operator::LessEqual => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.less_eq(self, right)
                    }
                    _ => Err("Cannot compare non-std structs".to_string()),
                }
            }
            Operator::Greater => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.greater(self, right)
                    }
                    _ => Err("Cannot compare non-std structs".to_string()),
                }
            }
            Operator::GreaterEqual => {
                match left{
                    Value::StdStruct(s1) => {
                        s1.greater_eq(self, right)
                    }
                    _ => Err("Cannot compare non-std structs".to_string()),
                }
            }
            Operator::And => {
                if (left.get_name() == "bool") && (right.get_name() == "bool"){
                    match left{
                        Value::StdStruct(s1) => {
                            s1.add(self, right)
                        }
                        _ => Err("Cannot and non-std structs".to_string()),
                    }
                } else{
                    Err("Cannot and non-bool types".to_string())
                }
            }
            Operator::Or => {
                if (left.get_name() == "bool") && (right.get_name() == "bool"){
                    match left{
                        Value::StdStruct(s1) => {
                            s1.add(self, right)
                        }
                        _ => Err("Cannot or non-std structs".to_string()),
                    }
                } else{
                    Err("Cannot or non-bool types".to_string())
                }
            }
            Operator::Comma => {
                Ok(left)
            }
            Operator::Return => {
                Ok(left)
            }
            _ => Err(format!("Cannot run operation {:?}", operator)),
        }
    }
}

impl Display for Program{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"
Charlang program state:

Functions:
    {}
Structs:
    {}
Variables:
    {}
"#, self.functions.keys().map(|f| f.to_string()).collect::<Vec<String>>().join("\n    "),
        self.structs.keys().map(|s| s.to_string()).collect::<Vec<String>>().join("\n    "),
        self.variables.keys().map(|v| v.to_string()).collect::<Vec<String>>().join("\n    "))
    }
} 