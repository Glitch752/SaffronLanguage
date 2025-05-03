#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ExpressionId(pub u32);

#[derive(Debug, PartialEq)]
pub enum Expression {
    Block(Vec<Statement>),

    NumberLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    Variable {
        name: String,
        expression_id: ExpressionId
    },
    BooleanLiteral(bool),

    FunctionCall {
        callee: Box<Expression>,
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
        value: Box<Expression>,
        expression_id: ExpressionId
    },
    MemberAccess {
        object: Box<Expression>,
        member: String
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
    Modulus,

    And,
    Or,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulus => "%",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::GreaterThanOrEqual => ">="
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Not
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            UnaryOperator::Negate => "-",
            UnaryOperator::Not => "!"
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression {
        expression: Box<Expression>,
        result: bool // true if this is a result value, false if it's just an expression statement
    },
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
    },
    // Struct {
    //     name: String,
    //     declarations: Vec<Box<Declaration>>,

    // },
    Import {
        path: Vec<String>
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: Type
}

#[derive(Debug, PartialEq)]
pub enum Type {
    U8, U16, U32, U64,
    I8, I16, I32, I64,
    F32, F64,
    Boolean,
    Character,
    Identifier {
        name: String,
        generic_args: Vec<Type> // Generic arguments for the type
    },
    /// Nil is the return type for functions that don't return a value.
    /// Nil can only have the value of `nil` (which, itself, is only valid for the type Nil), and is invalid in other contexts.
    Nil
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub declarations: Vec<Declaration>
}