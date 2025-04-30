use crate::tokenizer::{Token, TokenType};


#[derive(Debug, PartialEq)]
enum ParseError {
    UnexpectedToken {
        expected: TokenType,
        found: Token
    },
    UnexpectedEndOfInput
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Block(Vec<Statement>),

    NumberLiteral(f64),
    StringLiteral(String),
    Variable(String),

    FunctionCall {
        name: String,
        args: Vec<Expression>
    },
    
    BinaryOperation {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>
    },
    UnaryOperation {
        operator: UnaryOperator,
        operand: Box<Expression>
    },
    
    Assignment {
        variable: String,
        value: Box<Expression>
    },

    If {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>
    },
    Loop(LoopStatement)
}

#[derive(Debug, PartialEq)]
pub enum VariableMutability {
    Mutable,
    Immutable
}

#[derive(Debug, PartialEq)]
pub enum LoopStatement {
    While {
        condition: Box<Expression>,
        body: Box<Expression>
    },
    Infinite {
        body: Box<Expression>
    },
    Iterator {
        mutability: VariableMutability,
        iterator: String,
        iterable: Box<Expression>,
        body: Box<Expression>
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Negate
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration {
        mutability: VariableMutability,
        name: String,
        variable_type: Type,
        value: Box<Expression>
    },
    Break,
    Continue,
    Return(Option<Box<Expression>>)
}

#[derive(Debug, PartialEq)]
pub enum Declaration {
    Function {
        name: String,
        params: Vec<FunctionParameter>,
        return_type: Type,
        body: Box<Expression>
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionParameter {
    name: String,
    param_type: Type
}

#[derive(Debug, PartialEq)]
pub enum Type {
    U8, U16, U32, U64,
    I8, I16, I32, I64,
    F32, F64,
    Boolean,
    Character,
    Vector(Box<Type>) // Strings are vectors of characters
}

#[derive(Debug, PartialEq)]
pub struct Program {
    declarations: Vec<Declaration>
}