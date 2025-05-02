use value::Value;

use crate::parser::ast::{Declaration, Expression, Program, Statement};

mod value;

pub struct Interpreter<'a> {
    program: &'a Program
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Program) -> Self {
        Interpreter {
            program
        }
    }
    pub fn run(&mut self) -> Result<(), String> {
        // Initialize the interpreter state

        self.interpret_program(&self.program)?;

        Ok(())
    }

    fn interpret_program(&mut self, program: &Program) -> Result<(), String> {
        for statement in &program.declarations {
            self.interpret_declaration(statement)?;
        }
        Ok(())
    }
    fn interpret_declaration(&mut self, declaration: &Declaration) -> Result<(), String> {
        match declaration {
            Declaration::Function { name, params, return_type, body } => {
                // TODO: Functions
                // TEMPORARY
                if name == "main" {
                    self.interpret_expression(body);
                }
            },
            Declaration::Import { path } => {
                // TODO: Imports
            }
        }
        Ok(())
    }
    fn interpret_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Expression { expression, result } => {
                self.interpret_expression(expression)?;
                // TODO: Result expressions
            },
            // TODO
            _ => {
                return Err(format!("Unsupported statement: {:?}", statement));
            }
        };
        Ok(())
    }
    fn interpret_expression(&mut self, expression: &Expression) -> Result<Value, String> {
        match expression {
            Expression::CharLiteral(c) => {
                Ok(Value::Char(*c))
            },
            Expression::StringLiteral(s) => {
                Ok(Value::String(s.clone()))
            },
            Expression::NumberLiteral(n) => {
                Ok(Value::Number(*n))
            },

            Expression::FunctionCall { callee, args } => {
                // TODO
                // TEMPORARY
                if let Expression::Variable(name) = callee.as_ref() {
                    if name == "print" {
                        for arg in args {
                            let value = self.interpret_expression(arg)?;
                            println!("{}", value);
                        }
                        return Ok(Value::default());
                    } else {
                        return Err(format!("Unknown function: {}", name));
                    }
                }
                return Err(format!("Unsupported function call: {:?}", expression));
            },

            _ => todo!()
        }
    }
} 