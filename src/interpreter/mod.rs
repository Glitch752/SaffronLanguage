use value::Value;

use crate::parser::ast::{BinaryOperator, Declaration, Expression, LoopStatement, Program, Statement, Type, UnaryOperator};

mod value;

#[derive(Debug, PartialEq)]
pub enum InterpreterControl {
    Continue,
    Break,
    Return(Value),
    RuntimeError(String)
}

pub type InterpreterResult<T = Value> = Result<T, InterpreterControl>;

macro_rules! runtime_error {
    ($msg:expr) => {
        Err(InterpreterControl::RuntimeError($msg.to_string()))
    };
    ($fmt:expr, $($arg:tt)+) => {
        Err(InterpreterControl::RuntimeError(format!($fmt, $($arg)+)))
    };
}

pub struct Interpreter {
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
        }
    }
    pub fn run(&mut self, program: &Program) -> InterpreterResult<()> {
        // Initialize the interpreter state

        self.interpret_program(program)?;

        Ok(())
    }

    fn interpret_program(&mut self, program: &Program) -> InterpreterResult<()> {
        for statement in &program.declarations {
            self.interpret_declaration(statement)?;
        }
        Ok(())
    }
    fn interpret_declaration(&mut self, declaration: &Declaration) -> InterpreterResult<()> {
        match declaration {
            Declaration::Function { name, params, return_type, body } => {
                // TODO: Functions
                // TEMPORARY
                if name == "main" {
                    self.interpret_expression(body)?;
                }
            },
            Declaration::Import { path } => {
                // TODO: Imports
            }
        }
        Ok(())
    }
    fn interpret_statement(&mut self, statement: &Statement) -> InterpreterResult<()> {
        match statement {
            Statement::Break => {
                return Err(InterpreterControl::Break);
            },
            Statement::Continue => {
                return Err(InterpreterControl::Continue);
            },
            Statement::Return(value) => {
                return Err(InterpreterControl::Return(value
                    .as_ref()
                    .map(|v| self.interpret_expression(&v))
                    .unwrap_or(Ok(Value::Nil))?
                ));
            },

            Statement::Expression { expression, result } => {
                let value = self.interpret_expression(expression)?;
                if *result {
                    return Err(InterpreterControl::Return(value));
                } else {
                    return Ok(());
                }
            },

            Statement::VariableDeclaration { mutability, name, variable_type, value } => {
                todo!()
            }
        };
    }
    fn interpret_expression(&mut self, expression: &Expression) -> InterpreterResult {
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
            Expression::BooleanLiteral(b) => {
                Ok(Value::Boolean(*b))
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
                        return runtime_error!("Unknown function: {}", name);
                    }
                }
                return runtime_error!("Unsupported function call: {:?}", expression);
            },

            Expression::BinaryOperation { left, operator, right } => {
                let left_value = self.interpret_expression(left)?;
                let right_value = self.interpret_expression(right)?;
                match (operator, left_value, right_value) {
                    (BinaryOperator::Add, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l + r))
                    },
                    (BinaryOperator::Add, Value::String(l), Value::String(r)) => {
                        Ok(Value::String(format!("{}{}", l, r)))
                    },

                    (BinaryOperator::Subtract, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l - r))
                    },
                    (BinaryOperator::Multiply, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l * r))
                    },
                    (BinaryOperator::Divide, Value::Number(l), Value::Number(r)) => {
                        if r == 0.0 {
                            return runtime_error!("Division by zero");
                        }
                        Ok(Value::Number(l / r))
                    },
                    (BinaryOperator::Modulus, Value::Number(l), Value::Number(r)) => {
                        if r == 0.0 {
                            return runtime_error!("Division by zero");
                        }
                        Ok(Value::Number(l % r))
                    },
                    (BinaryOperator::Equal, l, r) => {
                        Ok(Value::Boolean(l == r))
                    },
                    (BinaryOperator::NotEqual, l, r) => {
                        Ok(Value::Boolean(l != r))
                    },

                    (BinaryOperator::LessThan, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l < r))
                    },
                    (BinaryOperator::LessThanOrEqual, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l <= r))
                    },
                    (BinaryOperator::GreaterThan, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l > r))
                    },
                    (BinaryOperator::GreaterThanOrEqual, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l >= r))
                    },
                    
                    (BinaryOperator::And, Value::Boolean(l), Value::Boolean(r)) => {
                        Ok(Value::Boolean(l && r))
                    },
                    (BinaryOperator::Or, Value::Boolean(l), Value::Boolean(r)) => {
                        Ok(Value::Boolean(l || r))
                    },

                    (_, l, r) => {
                        return runtime_error!("Unsupported binary operation: {} {} {}", l, operator, r);
                    }
                }
            },

            Expression::UnaryOperation { operator, operand } => {
                let operand_value = self.interpret_expression(operand)?;
                match (operator, operand_value) {
                    (UnaryOperator::Negate, Value::Number(n)) => {
                        Ok(Value::Number(-n))
                    },
                    (UnaryOperator::Not, Value::Boolean(b)) => {
                        Ok(Value::Boolean(!b))
                    },
                    (_, operand_value) => {
                        return runtime_error!("Unsupported unary operation: {} {}", operator, operand_value);
                    }
                }
            },

            Expression::Block(statements) => {
                for statement in statements {
                    if let Statement::Expression { result: true, expression } = statement {
                        return Ok(self.interpret_expression(expression)?);
                    }
                    _ = self.interpret_statement(statement)?;
                }
                Ok(Value::default())
            },

            Expression::Loop(LoopStatement::Infinite { body }) => {
                loop {
                    match self.interpret_expression(body) {
                        Err(InterpreterControl::Break) => {
                            return Ok(Value::default());
                        },
                        Err(InterpreterControl::Continue) => {
                            continue;
                        },

                        Err(e) => {
                            return Err(e);
                        },
                        Ok(_) => (),
                    };
                }
            },
            Expression::Loop(LoopStatement::While { condition, body }) => {
                loop {
                    let condition_value = self.interpret_expression(condition)?;
                    if let Value::Boolean(false) = condition_value {
                        return Ok(Value::default());
                    }
                    match self.interpret_expression(body) {
                        Err(InterpreterControl::Break) => {
                            return Ok(Value::default());
                        },
                        Err(InterpreterControl::Continue) => {
                            continue;
                        },

                        Err(e) => {
                            return Err(e);
                        },
                        _ => (),
                    };
                }
            },
            Expression::Loop(LoopStatement::Iterator { mutability, iterator, iterable, body }) => {
                todo!()
            },

            Expression::If { condition, then_branch, else_branch } => {
                let condition_value = self.interpret_expression(condition)?;
                if let Value::Boolean(true) = condition_value {
                    return self.interpret_expression(then_branch);
                } else if let Some(else_branch) = else_branch {
                    return self.interpret_expression(else_branch);
                } else {
                    return Ok(Value::default());
                }
            },
            
            _ => todo!("Unsupported expression: {:?}", expression)
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser::{ast::{BinaryOperator, Declaration, Expression, LoopStatement, Program, Statement, Type, UnaryOperator}, Parser}, tokenizer::Tokenizer};

    macro_rules! parse {
        ($input:expr, $parse_fn:ident) => {
            {
                let mut tokenizer = Tokenizer::new($input.to_string());
                let tokens = tokenizer.tokenize().unwrap();
                let mut parser = Parser::new(&tokens);
                let expression = parser.$parse_fn().unwrap();
                expression
            }
        };
    }

    #[test]
    fn test_interpreter() {
        let program = Program {
            declarations: vec![
                Declaration::Function {
                    name: "main".to_string(),
                    params: vec![],
                    return_type: Type::F64,
                    body: Box::new(Expression::Block(vec![
                        Statement::Expression {
                            expression: Box::new(Expression::BinaryOperation {
                                left: Box::new(Expression::NumberLiteral(5.0)),
                                operator: BinaryOperator::Add,
                                right: Box::new(Expression::NumberLiteral(3.0))
                            }),
                            result: true
                        }
                    ]))
                }
            ]
        };

        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_associativity() {
        let result = Interpreter::new().interpret_expression(&parse!(r#"
            1 + 2 * 3 - 4 / 5 % 6
        "#, parse_expression));

        assert_eq!(result, Ok(Value::Number(1.0 + 2.0 * 3.0 - 4.0 / 5.0 % 6.0)));
    }
}