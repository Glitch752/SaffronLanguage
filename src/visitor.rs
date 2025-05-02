use crate::parser::ast::Program;

pub trait Visitor {
    fn visit_program(&mut self, program: &Program) -> Result<(), String>;
}