use crate::{parser::ast::Program, visitor::Visitor};

pub struct Interpreter {
    program: Program
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Interpreter {
            program
        }
    }
} 

impl Visitor for Interpreter {
    fn visit_program(&mut self, program: &Program) -> Result<(), String> {
        // ...
        Ok(())
    }
}