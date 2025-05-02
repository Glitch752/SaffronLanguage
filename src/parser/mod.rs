use ast::{BinaryOperator, Declaration, Expression, FunctionParameter, LoopStatement, Program, Statement, Type, UnaryOperator, VariableMutability};

use crate::tokenizer::{Token, TokenType};

pub mod ast;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: Option<TokenType>,
        found: Token,
        message: Option<String>
    },
    UnexpectedEndOfInput
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found, message } => {
                let message = message.as_ref().map(|s| s.as_str()).unwrap_or("");
                if let Some(expected) = expected {
                    write!(f, "Expected {:?}, found {:?}. {}", expected, found.token_type, message)
                } else {
                    write!(f, "Unexpected token: {:?}. {}", found.token_type, message)
                }
            },
            ParseError::UnexpectedEndOfInput => {
                write!(f, "Unexpected end of input")
            }
        }
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    errors: Vec<ParseError>
}

macro_rules! parse_precedence_binary {
    ($self:ident, $next_level:ident, $( ($token_type:path, $operator:expr) ),+ $(,)?) => {
        {
            let mut expr = $self.$next_level()?;
            while !$self.is_eof() && let Some(operator) = match $self.peek().token_type.clone() {
                $(
                    $token_type => Some($operator),
                )+
                _ => None
            } {
                $self.advance(); // Consume the operator

                let right = Box::new($self.$next_level()?);
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator,
                    right
                };
            }
            Ok(expr)
        }
    };
}

macro_rules! parse_precedence_unary {
    ($self:ident, $next_level:ident, $( ($token_type:path, $operator:expr) ),+ $(,)?) => {
        {
            let mut expr = $self.$next_level()?;
            while !$self.is_eof() && let Some(operator) = match $self.peek().token_type.clone() {
                $(
                    $token_type => Some($operator),
                )+
                _ => None
            } {
                $self.advance(); // Consume the operator

                let right = Box::new($self.$next_level()?);
                expr = Expression::UnaryOperation {
                    operator,
                    operand: right
                };
            }
            Ok(expr)
        }
    };
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a[Token]) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new()
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

    fn advance_if(&mut self, token_type: TokenType) -> bool {
        if self.is_match(token_type) {
            self.advance(); // Consume the token
            true
        } else {
            false
        }
    }

    /// Synchronizes the parser by skipping tokens until it finds a semicolon or EOF.
    /// This is useful for error recovery.
    fn synchronize(&mut self) {
        self.advance(); // Consume the token

        while !self.is_eof() {
            if self.advance_if(TokenType::Semicolon) {
                break; // Stop at the next semicolon
            }
            
            match self.peek().token_type {
                TokenType::FunctionKeyword | TokenType::ImportKeyword => {
                    break; // Stop at the next function or import keyword
                },
                _ => {}
            }

            self.advance(); // Consume the token
        }
    }

    /// Parses the entire program and returns a Program object. If parsing fails, it returns None.
    pub fn parse_program(&mut self) -> Option<Program> {
        let mut declarations = Vec::new();
        self.errors.clear(); // Clear previous errors

        while !self.is_eof() {
            let decl = match self.parse_declaration() {
                Ok(decl) => decl,
                Err(e) => {
                    self.errors.push(e); // Store the error
                    self.synchronize(); // Skip to the next statement
                    continue; // Try to parse the next declaration
                }
            };
            declarations.push(decl);
        }

        if !self.errors.is_empty() {
            for error in &self.errors {
                eprintln!("Error: {}", error);
            }
            return None; // Return None if there were errors
        }

        Some(Program { declarations })
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.peek().token_type.clone() {
            TokenType::Identifier(ref name) => {
                self.advance(); // Consume the identifier
                Ok(name.clone())
            },
            _ => Err(ParseError::UnexpectedToken {
                expected: Some(TokenType::Identifier("".to_string())),
                found: self.peek().clone(),
                message: Some("Expected an identifier".to_string())
            })
        }
    }

    fn expect(&mut self, token_type: TokenType, message: &str) -> Result<(), ParseError> {
        if self.is_match(token_type.clone()) {
            self.advance(); // Consume the expected token
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: Some(token_type),
                found: self.peek().clone(),
                message: Some(message.to_string())
            })
        }
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<FunctionParameter>, ParseError> {
        self.expect(TokenType::OpenParenthesis, "Expected open parentheses after function name")?; // Expect an open parenthesis
        
        let mut params = Vec::new();
        while !self.is_eof() && self.peek().token_type != TokenType::CloseParenthesis {
            let name = self.expect_identifier()?;
            self.expect(TokenType::Colon, "Expected colon after function parameter for type")?; // Expect a colon after the name
            let param_type = self.parse_type()?;
            params.push(FunctionParameter { name, param_type });

            if self.is_match(TokenType::Comma) {
                self.advance(); // Consume the comma
            } else {
                break; // No more parameters
            }
        }

        self.expect(TokenType::CloseParenthesis, "Unmatched open parentheses")?; // Expect a close parenthesis

        Ok(params)
    }

    pub(crate) fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        if self.is_match(TokenType::FunctionKeyword) {
            self.advance(); // Consume 'function'
            let name = self.expect_identifier()?;
            let params = self.parse_function_parameters()?;
            self.expect(TokenType::Arrow, "Expected arrow after function parameters for type")?; // Expect an arrow after the parameters
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

            self.expect(TokenType::Semicolon, "Expected semicolon after import path")?; // Expect a semicolon

            Ok(Declaration::Import { path })
        } else {
            Err(ParseError::UnexpectedToken {
                expected: None,
                found: self.peek().clone(),
                message: Some("Expected a function or import declaration".to_string())
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
                        found: self.peek().clone(),
                        message: Some(format!("Unknown type: {}", name))
                    })
                }
            },
            _ => Err(ParseError::UnexpectedToken {
                expected: Some(TokenType::Identifier("".to_string())),
                found: self.peek().clone(),
                message: Some("Expected a type identifier".to_string())
            })
        }
    }

    pub(crate) fn parse_block(&mut self) -> Result<Expression, ParseError> {
        self.expect(TokenType::OpenCurlyBracket, "Expected open brace")?; // Expect an open brace
        let mut statements = Vec::new();
        while !self.is_eof() && self.peek().token_type != TokenType::CloseCurlyBracket {
            let stmt = match self.parse_statement() {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.errors.push(e); // Store the error
                    self.synchronize(); // Skip to the next statement
                    continue; // Try to parse the next statement
                }
            };

            let is_result_expression = match stmt {
                Statement::Expression { result: true, .. } => true,
                _ => false
            };
            statements.push(stmt);
            if is_result_expression {
                break;
            }
        }
        self.expect(TokenType::CloseCurlyBracket, "Unmatched open brace")?; // Expect a close brace
        Ok(Expression::Block(statements))
    }

    pub(crate) fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek().token_type.clone() {
            // Easy single-keyword statements
            TokenType::BreakKeyword => {
                // TODO: Breaking with values
                self.advance(); // Consume 'break'
                self.expect(TokenType::Semicolon, "Expected semicolon after break")?; // Expect a semicolon
                Ok(Statement::Break)
            },
            TokenType::ContinueKeyword => {
                self.advance(); // Consume 'continue'
                self.expect(TokenType::Semicolon, "Expected semicolon after continue")?; // Expect a semicolon
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
                self.expect(TokenType::Colon, "Expected colon after variable name")?; // Expect a colon after the name
                let variable_type = self.parse_type()?;
                self.expect(TokenType::AssignmentOperator, "Expected assignment operator after variable type")?; // Expect an assignment operator
                let value = Box::new(self.parse_expression()?);
                self.expect(TokenType::Semicolon, "Expected semicolon after variable declaration")?; // Expect a semicolon
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
                self.expect(TokenType::Semicolon, "Expected semicolon after return")?; // Expect a semicolon
                Ok(Statement::Return(value))
            },

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

    pub(crate) fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        // Blocks are expressions
        if self.is_match(TokenType::OpenCurlyBracket) {
            return self.parse_block(); // Parse a block
        }

        // Try to parse loop statements
        if self.advance_if(TokenType::LoopKeyword) {
            // If there's a set of parentheses, this is a while loop or iterator loop
            if self.advance_if(TokenType::OpenParenthesis) {
                // If there's a let or const keyword, this is an iterator loop
                if let Some(mutability) = match self.peek().token_type.clone() {
                    TokenType::LetKeyword => Some(VariableMutability::Mutable),
                    TokenType::ConstKeyword => Some(VariableMutability::Immutable),
                    _ => None
                } {
                    self.advance(); // Consume 'let' or 'const'
                    let iterator = self.expect_identifier()?;
                    self.expect(TokenType::Colon, "Expected colon after variable name")?; // Expect a colon after the name
                    let iterable = Box::new(self.parse_expression()?);
                    self.expect(TokenType::CloseParenthesis, "Unmatched open parentheses")?; // Expect a close parenthesis
                    let body = Box::new(self.parse_block()?);
                    return Ok(Expression::Loop(LoopStatement::Iterator {
                        body,
                        mutability,
                        iterator,
                        iterable
                    }));
                }
                
                let condition = Box::new(self.parse_expression()?);

                self.expect(TokenType::CloseParenthesis, "Unmatched open parentheses")?; // Expect a close parenthesis
                let body = Box::new(self.parse_block()?);
                return Ok(Expression::Loop(LoopStatement::While {
                    condition,
                    body
                }));
            } else {
                // Otherwise, this is an infinite loop
                let body = Box::new(self.parse_block()?);
                return Ok(Expression::Loop(LoopStatement::Infinite {
                    body
                }));
            }
        }

        // Try to parse if statements
        if self.advance_if(TokenType::IfKeyword) {
            self.expect(TokenType::OpenParenthesis, "Expected open parentheses after if")?; // Expect an open parenthesis
            let condition = Box::new(self.parse_expression()?);
            self.expect(TokenType::CloseParenthesis, "Unmatched open parentheses")?; // Expect a close parenthesis
            let body = Box::new(self.parse_expression()?);

            // Optional semicolon after the if statement
            self.advance_if(TokenType::Semicolon);

            let else_branch = if self.advance_if(TokenType::ElseKeyword) {
                Some(Box::new(self.parse_expression()?)) // Parse the else branch
            } else {
                None // No else branch
            };

            return Ok(Expression::If {
                condition,
                then_branch: body,
                else_branch
            });
        }

        self.parse_equality_or_lower()
    }

    fn parse_equality_or_lower(&mut self) -> Result<Expression, ParseError> {
        parse_precedence_binary!(
            self,
            parse_comparison_or_lower,
            (TokenType::EqualOperator, BinaryOperator::Equal),
            (TokenType::NotEqualOperator, BinaryOperator::NotEqual),
        )
    }

    fn parse_comparison_or_lower(&mut self) -> Result<Expression, ParseError> {
        parse_precedence_binary!(
            self,
            parse_term_or_lower,
            (TokenType::OpenAngleBracket, BinaryOperator::Add),
            (TokenType::CloseAngleBracket, BinaryOperator::Subtract),
            (TokenType::LessThanEqualOperator, BinaryOperator::Multiply),
            (TokenType::GreaterThanEqualOperator, BinaryOperator::Divide),
        )
    }

    fn parse_term_or_lower(&mut self) -> Result<Expression, ParseError> {
        parse_precedence_binary!(
            self,
            parse_factor_or_lower,
            (TokenType::AddOperator, BinaryOperator::Add),
            (TokenType::SubtractOperator, BinaryOperator::Subtract),
        )
    }

    fn parse_factor_or_lower(&mut self) -> Result<Expression, ParseError> {
        parse_precedence_binary!(
            self,
            parse_unary_or_lower,
            (TokenType::MultiplyOperator, BinaryOperator::Multiply),
            (TokenType::DivideOperator, BinaryOperator::Divide),
            (TokenType::ModuloOperator, BinaryOperator::Modulus),
        )
    }

    fn parse_unary_or_lower(&mut self) -> Result<Expression, ParseError> {
        parse_precedence_unary!(
            self,
            parse_call_or_lower,
            (TokenType::NotOperator, UnaryOperator::Not),
            (TokenType::SubtractOperator, UnaryOperator::Negate),
        )
    }

    fn parse_call_or_lower(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary_or_lower()?;

        while !self.is_eof() {
            if self.advance_if(TokenType::OpenParenthesis) {
                expr = self.parse_function_call_after_paren(expr)?; // Parse function call
            } else if self.advance_if(TokenType::Dot) {
                let name = self.expect_identifier()?; // Expect an identifier after the dot
                expr = Expression::MemberAccess { object: Box::new(expr), member: name };
            } else {
                break; // No more function calls or member accesses
            }
        }

        Ok(expr)
    }

    fn parse_function_call_after_paren(&mut self, callee: Expression) -> Result<Expression, ParseError> {
        let mut args = Vec::new();
        while !self.is_eof() && self.peek().token_type != TokenType::CloseParenthesis {
            args.push(self.parse_expression()?);
            if self.is_match(TokenType::Comma) {
                self.advance(); // Consume the comma
            } else {
                break; // No more arguments
            }
        }
        self.expect(TokenType::CloseParenthesis, "Unmatched open parentheses")?;
        Ok(Expression::FunctionCall {
            callee: Box::new(callee),
            args
        })
    }

    fn parse_primary_or_lower(&mut self) -> Result<Expression, ParseError> {
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
            TokenType::TrueValue => {
                self.advance(); // Consume 'true'
                Ok(Expression::BooleanLiteral(true))
            },
            TokenType::FalseValue => {
                self.advance(); // Consume 'false'
                Ok(Expression::BooleanLiteral(false))
            },

            TokenType::Identifier(ref name) => {
                self.advance(); // Consume the identifier
                Ok(Expression::Variable(name.clone()))
            },

            TokenType::OpenParenthesis => {
                self.advance(); // Consume the open parenthesis
                let expr = self.parse_expression()?;
                self.expect(TokenType::CloseParenthesis, "Unmatched open parentheses")?; // Expect a close parenthesis
                Ok(expr)
            },

            _ => {
                Err(ParseError::UnexpectedToken {
                    expected: None,
                    found: self.peek().clone(),
                    message: Some("Expected an expression".to_string())
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Tokenizer;

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
    fn test_associativity() {
        assert_eq!(parse!(r#"
            1 + 2 * 3 - 4 / 5 % 6
        "#, parse_expression), 
            Expression::BinaryOperation {
                left: Box::new(Expression::BinaryOperation {
                    left: Box::new(Expression::NumberLiteral(1.0)),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::BinaryOperation {
                        left: Box::new(Expression::NumberLiteral(2.0)),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(Expression::NumberLiteral(3.0))
                    })
                }),
                operator: BinaryOperator::Subtract,
                right: Box::new(Expression::BinaryOperation {
                    left: Box::new(Expression::BinaryOperation {
                        left: Box::new(Expression::NumberLiteral(4.0)),
                        operator: BinaryOperator::Divide,
                        right: Box::new(Expression::NumberLiteral(5.0))
                    }),
                    operator: BinaryOperator::Modulus,
                    right: Box::new(Expression::NumberLiteral(6.0))
                })
            }
        );
    }
}