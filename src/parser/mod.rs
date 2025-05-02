use ast::{Declaration, Expression, FunctionParameter, Program, Statement, Type, VariableMutability};

use crate::tokenizer::{Token, TokenType};

mod ast;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: Option<TokenType>,
        found: Token
    },
    UnexpectedEndOfInput
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a[Token]) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn is_eof(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_match(&self, token_type: TokenType) -> bool {
        !self.is_eof() && self.peek().token_type == token_type
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut declarations = Vec::new();
        while !self.is_eof() {
            declarations.push(self.parse_declaration()?);
        }
        Ok(Program { declarations })
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.peek().token_type.clone() {
            TokenType::Identifier(ref name) => {
                self.advance(); // Consume the identifier
                Ok(name.clone())
            },
            _ => Err(ParseError::UnexpectedToken {
                expected: Some(TokenType::Identifier("".to_string())),
                found: self.peek().clone()
            })
        }
    }

    fn expect(&mut self, token_type: TokenType) -> Result<(), ParseError> {
        if self.is_match(token_type.clone()) {
            self.advance(); // Consume the expected token
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: Some(token_type),
                found: self.peek().clone()
            })
        }
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<FunctionParameter>, ParseError> {
        self.expect(TokenType::OpenParenthesis)?; // Expect an open parenthesis
        
        let mut params = Vec::new();
        while !self.is_eof() && self.peek().token_type != TokenType::CloseParenthesis {
            let name = self.expect_identifier()?;
            self.expect(TokenType::Colon)?; // Expect a colon after the name
            let param_type = self.parse_type()?;
            params.push(FunctionParameter { name, param_type });

            if self.is_match(TokenType::Comma) {
                self.advance(); // Consume the comma
            } else {
                break; // No more parameters
            }
        }

        self.expect(TokenType::CloseParenthesis)?; // Expect a close parenthesis

        Ok(params)
    }

    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        if self.is_match(TokenType::FunctionKeyword) {
            self.advance(); // Consume 'function'
            let name = self.expect_identifier()?;
            let params = self.parse_function_parameters()?;
            self.expect(TokenType::Arrow)?; // Expect an arrow after the parameters
            let return_type = self.parse_type()?;
            let body = self.parse_block()?;
            Ok(Declaration::Function { name, params, return_type, body: Box::new(body) })
        } else if self.is_match(TokenType::ImportKeyword) {
            self.advance(); // Consume 'import'

            let mut path = vec![
                self.expect_identifier()? // Expect the first part of the path
            ];

            while !self.is_eof() {
                if self.is_match(TokenType::Dot) {
                    self.advance(); // Consume the dot
                    path.push(self.expect_identifier()?); // Expect the next part of the path
                } else {
                    break; // No more parts of the path
                }
            }

            self.expect(TokenType::Semicolon)?;

            Ok(Declaration::Import { path })
        } else {
            Err(ParseError::UnexpectedToken {
                expected: None,
                found: self.peek().clone()
            })
        }
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match self.peek().token_type.clone() {
            TokenType::Identifier(ref name) => {
                self.advance(); // Consume the identifier
                match name.as_str() {
                    "u8" => Ok(Type::U8),
                    "u16" => Ok(Type::U16),
                    "u32" => Ok(Type::U32),
                    "u64" => Ok(Type::U64),
                    "i8" => Ok(Type::I8),
                    "i16" => Ok(Type::I16),
                    "i32" => Ok(Type::I32),
                    "i64" => Ok(Type::I64),
                    "f32" => Ok(Type::F32),
                    "f64" => Ok(Type::F64),
                    "bool" => Ok(Type::Boolean),
                    "char" => Ok(Type::Character),
                    // TODO: Handle vector types
                    _ => Err(ParseError::UnexpectedToken {
                        expected: Some(TokenType::Identifier(name.clone())),
                        found: self.peek().clone()
                    })
                }
            },
            _ => Err(ParseError::UnexpectedToken {
                expected: Some(TokenType::Identifier("".to_string())),
                found: self.peek().clone()
            })
        }
    }

    fn parse_block(&mut self) -> Result<Expression, ParseError> {
        self.expect(TokenType::OpenCurlyBracket)?; // Expect an open brace
        let mut statements = Vec::new();
        while !self.is_eof() && self.peek().token_type != TokenType::CloseCurlyBracket {
            let stmt = self.parse_statement()?;
            let is_result_expression = match stmt {
                Statement::Expression { result: true, .. } => true,
                _ => false
            };
            statements.push(stmt);
            if is_result_expression {
                break;
            }
        }
        self.expect(TokenType::CloseCurlyBracket)?; // Expect a close brace
        Ok(Expression::Block(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek().token_type.clone() {
            // Easy single-keyword statements
            TokenType::BreakKeyword => {
                // TODO: Breaking with values
                self.advance(); // Consume 'break'
                self.expect(TokenType::Semicolon)?; // Expect a semicolon
                Ok(Statement::Break)
            },
            TokenType::ContinueKeyword => {
                self.advance(); // Consume 'continue'
                self.expect(TokenType::Semicolon)?; // Expect a semicolon
                Ok(Statement::Continue)
            },

            // Variable declaration
            TokenType::LetKeyword | TokenType::ConstKeyword => {
                let mutability = if self.is_match(TokenType::LetKeyword) {
                    VariableMutability::Mutable
                } else {
                    VariableMutability::Immutable
                };
                self.advance(); // Consume 'let' or 'const'
                let name = self.expect_identifier()?;
                self.expect(TokenType::Colon)?; // Expect a colon after the name
                let variable_type = self.parse_type()?;
                self.expect(TokenType::AssignmentOperator)?; // Expect an equal sign
                let value = Box::new(self.parse_expression()?);
                self.expect(TokenType::Semicolon)?; // Expect a semicolon
                Ok(Statement::VariableDeclaration { mutability, name, variable_type, value })
            },

            // Return
            TokenType::ReturnKeyword => {
                self.advance(); // Consume 'return'
                let value = if self.is_match(TokenType::Semicolon) {
                    None
                } else {
                    Some(Box::new(self.parse_expression()?))
                };
                self.expect(TokenType::Semicolon)?; // Expect a semicolon
                Ok(Statement::Return(value))
            },

            // TODO

            _ => {
                // Try to parse as an expression statement
                let expr = self.parse_expression()?;
                // If there's a semicolon, this is an expression. Otherwise, it's a result value.
                let result = if self.is_match(TokenType::Semicolon) {
                    self.advance(); // Consume the semicolon
                    false // This is just an expression statement
                } else {
                    true // This is a result value
                };
                Ok(Statement::Expression {
                    expression: Box::new(expr),
                    result
                })
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match self.peek().token_type.clone() {
            // Simple literals
            TokenType::IntegerLiteral(ref value) => {
                self.advance(); // Consume the number
                Ok(Expression::NumberLiteral(*value as f64)) // Convert to f64
            },
            TokenType::FloatLiteral(ref value) => {
                self.advance(); // Consume the number
                Ok(Expression::NumberLiteral(*value)) // Already f64
            },
            TokenType::StringLiteral(ref value) => {
                self.advance(); // Consume the string
                Ok(Expression::StringLiteral(value.clone()))
            },
            TokenType::CharLiteral(ref value) => {
                self.advance(); // Consume the char
                Ok(Expression::CharLiteral(value.clone()))
            },

            TokenType::Identifier(ref name) => {
                self.advance(); // Consume the identifier

                // Check if this is a function call or a variable
                if self.is_match(TokenType::OpenParenthesis) {
                    // Function call
                    self.advance(); // Consume the open parenthesis
                    let mut args = Vec::new();
                    while !self.is_eof() && self.peek().token_type != TokenType::CloseParenthesis {
                        args.push(self.parse_expression()?);
                        if self.is_match(TokenType::Comma) {
                            self.advance(); // Consume the comma
                        } else {
                            break; // No more arguments
                        }
                    }
                    self.expect(TokenType::CloseParenthesis)?; // Expect a close parenthesis
                    Ok(Expression::FunctionCall { name: name.clone(), args })
                } else {
                    Ok(Expression::Variable(name.clone()))
                }
            },

            // In a recursive descent parser, we parse in the reverse order of precedence
            // _ => self.consume_assignment_or_lower()
            _ => {
                // TODO
                Err(ParseError::UnexpectedToken {
                    expected: None,
                    found: self.peek().clone()
                })
            }
        }
    }

    // TODO: These could be simplified with a macro

    // WIP

    // fn consume_assignment_or_lower(&mut self) -> Result<Expression, ParseError> {
    //     let mut expr = self.consume_logical_or_or_lower()?;
    //     while self.is_match(TokenType::AssignmentOperator) {
    //         let operator = match self.peek().token_type.clone() {
    //             TokenType::AssignmentOperator => BinaryOperator::Add,
    //             _ => unreachable!()
    //         };
    //         self.advance(); // Consume the assignment operator
    //         let value = Box::new(self.consume_logical_or_or_lower()?);
    //         expr = Expression::Assignment {
    //             variable: "".to_string(), // TODO: Get the variable name from the context
    //             value: Box::new(expr)
    //         };
    //     }
    //     Ok(expr)
    // }

    // fn consume_logical_or_or_lower(&mut self) -> Result<Expression, ParseError> {
    //     let mut expr = self.consume_logical_and_or_lower()?;
    //     while self.is_match(TokenType::OrOperator) {
    //         let operator = match self.peek().token_type.clone() {
    //             TokenType::OrOperator => BinaryOperator::Add,
    //             _ => unreachable!()
    //         };
    //         self.advance(); // Consume the logical or operator
    //         let right = Box::new(self.consume_logical_and_or_lower()?);
    //         expr = Expression::BinaryOperation {
    //             left: Box::new(expr),
    //             operator,
    //             right
    //         };
    //     }
    //     Ok(expr)
    // }

    // fn consume_logical_and_or_lower(&mut self) -> Result<Expression, ParseError> {
    //     let mut expr = self.consume_equality_or_lower()?;
    //     while self.is_match(TokenType::AndOperator) {
    //         let operator = match self.peek().token_type.clone() {
    //             TokenType::AndOperator => BinaryOperator::Add,
    //             _ => unreachable!()
    //         };
    //         self.advance(); // Consume the logical and operator
    //         let right = Box::new(self.consume_equality_or_lower()?);
    //         expr = Expression::BinaryOperation {
    //             left: Box::new(expr),
    //             operator,
    //             right
    //         };
    //     }
    //     Ok(expr)
    // }

    // fn consume_equality_or_lower(&mut self) -> Result<Expression, ParseError> {
    //     let mut expr = self.consume_comparison_or_lower()?;
    //     while self.is_match(TokenType::EqualOperator) {
    //         let operator = match self.peek().token_type.clone() {
    //             TokenType::EqualOperator => BinaryOperator::Add,
    //             _ => unreachable!()
    //         };
    //         self.advance(); // Consume the equality operator
    //         let right = Box::new(self.consume_comparison_or_lower()?);
    //         expr = Expression::BinaryOperation {
    //             left: Box::new(expr),
    //             operator,
    //             right
    //         };
    //     }
    //     Ok(expr)
    // }

    // fn consume_comparison_or_lower(&mut self) -> Result<Expression, ParseError> {
    //     let mut expr = self.consume_addition_or_lower()?;
    //     while let Some(op) = match self.peek().token_type.clone() {
    //         TokenType::OpenAngleBracket => Some(BinaryOperator::Add),
    //         TokenType::CloseAngleBracket => Some(BinaryOperator::Subtract),
    //         TokenType::LessThanEqualOperator => Some(BinaryOperator::Multiply),
    //         TokenType::GreaterThanEqualOperator => Some(BinaryOperator::Divide),
    //         _ => None
    //     } {
    //         self.advance(); // Consume the comparison operator
    //         let right = Box::new(self.consume_addition_or_lower()?);
    //         expr = Expression::BinaryOperation {
    //             left: Box::new(expr),
    //             operator,
    //             right
    //         };
    //     }
    //     Ok(expr)
    // }
}