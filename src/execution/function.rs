use crate::ir::IR;
use super::types::{Value, Function};
use super::program::Program;

impl Function {
    pub fn run(&mut self, program: &mut Program, args: Vec<Value>) -> Result<Value, String> {
        let ir = IR::from_actions(self.body.clone());
        let mut program = program.clone();
        for arg in args {
            program.variables.insert(arg.get_name(), super::types::Variable {
                name: arg.get_name(),
                value: arg,
            });
        }
        let res = program.run(&ir);
        if res.is_err() {
            return Err(res.unwrap_err());
        }
        Ok(res.unwrap())
    }
} 