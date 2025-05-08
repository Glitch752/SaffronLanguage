use std::collections::HashMap;

use crate::parser::ast::{Declaration, Expression, ExpressionId, LoopType, Program, Statement, Type};

use super::Interpreter;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new()
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
    
    /// Declares a variable in the topmost scope as "being defined".
    fn declare(&mut self, name: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, false);
        }
    }
    
    /// Declares a variable in the topmost scope as defined.
    fn define(&mut self, name: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, true);
        }
    }

    pub fn resolve_program(&mut self, program: &Program) -> Result<(), String> {
        for declaration in &program.declarations {
            self.resolve_declaration(declaration)?;
        }
        Ok(())
    }

    fn resolve_declaration(&mut self, declaration: &Declaration) -> Result<(), String> {
        match declaration {
            Declaration::Function { name, params, return_type, body, generic_args } => {
                todo!()
            },
            Declaration::Import { path } => {
                todo!()
            },
            Declaration::Struct { name, elements: declarations, generic_args } => {
                todo!()
            },
            Declaration::TypeDeclaration { name, alias, generic_args } => {
                todo!()
            }
        }
        Ok(())
    }

    fn resolve_expression(&mut self, expression: &Expression) -> Result<(), String> {
        match expression {
            Expression::Assignment { name: variable, value, expression_id } => {
                self.resolve_expression(value)?;
                self.record_local_depth(*expression_id, variable.to_string())?;
            },
            Expression::BinaryOperation { left, right, .. } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            },
            Expression::UnaryOperation { operand, .. } => {
                self.resolve_expression(&operand)?;
            },
            Expression::Block(statements) => {
                self.begin_scope();

                for statement in statements {
                    self.resolve_statement(statement)?;
                }

                self.end_scope();
            },
            Expression::BooleanLiteral(_) | Expression::CharLiteral(_) | Expression::NumberLiteral(_) | Expression::StringLiteral(_) => {
                // Nothing
            },
            Expression::FunctionCall { callee, args } => {
                self.resolve_expression(&callee)?;
                for arg in args {
                    self.resolve_expression(arg)?;
                }
            },
            Expression::Variable { name, expression_id } => {
                if let Some(scope) = self.scopes.last() {
                    if scope.get(name) == Some(&false) {
                        return Err(format!("Error: Tried to read {} in its own declaration.", name));
                    }
                }

                self.record_local_depth(*expression_id, name.to_string())?
            },
            Expression::If { condition, then_branch, else_branch } => {
                self.resolve_expression(&condition)?;
                self.resolve_expression(&then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.resolve_expression(&else_branch)?;
                }
            },
            Expression::Loop(LoopType::Infinite { body }) => {
                self.resolve_expression(&body)?;
            },
            Expression::Loop(LoopType::While { condition, body }) => {
                self.resolve_expression(&condition)?;
                self.resolve_expression(&body)?;
            },
            Expression::Loop(LoopType::Iterator { iterator, iterable, body, .. }) => {
                self.declare(iterator.to_string());
                self.resolve_expression(&iterable)?;
                self.define(iterator.to_string());

                self.resolve_expression(&body)?;
            },
            Expression::MemberAccess { object, .. } => {
                self.resolve_expression(&object)?;
            },
            Expression::Array { array_type, size, initial_value } => {
                todo!()
            },
            Expression::StructCreation { struct_type, fields } => {
                todo!()
            },
        }
        Ok(())
    }

    fn resolve_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Declaration(declaration) => {
                self.resolve_declaration(declaration)?;
            },
            Statement::Break | Statement::Continue => {
                // Nothing to do here
            },
            Statement::Expression { expression, .. } => {
                self.resolve_expression(expression)?;
            },
            Statement::Return(value) => {
                if let Some(value) = value {
                    self.resolve_expression(value)?;
                }
            },
            Statement::VariableDeclaration { name, variable_type, value, .. } => {
                self.declare(name.to_string());
                self.resolve_expression(value)?;
                self.define(name.to_string());

                self.resolve_type(variable_type);
            }
        }
        Ok(())
    }

    fn record_local_depth(&mut self, expression_id: ExpressionId, name: String) -> Result<(), String> {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name) {
                self.interpreter.resolve(expression_id, i);
                return Ok(());
            }
        }
        Ok(())
    }

    fn resolve_type(&self, ty: &Type) {
        match ty {
            _ => todo!()
        };
    }
}