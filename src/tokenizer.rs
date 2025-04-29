use std::{collections::HashMap, iter::Peekable, str::Chars, sync::LazyLock};

#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    // keywords
    ImportKeyword, // import
    FunctionKeyword, // func
    ReturnKeyword, // return
    IfKeyword, // if
    ElseKeyword, // else
    LoopKeyword, // loop
    
    // values
    TrueValue, // true
    FalseValue, // false
    IntegerValue(i64), // 0, 1, 2, etc.
    FloatValue(f64), // 0.0, 0.1, 0.2, etc.
    CharValue(char), // 'a', 'b', 'c', etc.

    Identifier(String), // variable names, function names, etc.

    // operators
    AddOperator, // +
    SubtractOperator, // -
    MultiplyOperator, // *
    DivideOperator, // /
    ModuloOperator, // %
    AssignmentOperator, // =
    
    Semicolon, // ;
    Comma, // ,
    Dot, // .
    Colon, // :
    Arrow, // ->
    Pipeline, // |>

    // comparison
    EqualOperator, // ==
    NotEqualOperator, // !=
    // Greater than and less than are angle brackets
    GreaterThanEqualOperator, // >=
    LessThanEqualOperator, // <=

    // brackets
    OpenParenthesis, // (
    CloseParenthesis, // )
    OpenCurlyBracket, // {
    CloseCurlyBracket, // }
    OpenSquareBracket, // [
    CloseSquareBracket, // ]
    OpenAngleBracket, // <
    CloseAngleBracket // >
}

static KEYWORDS: LazyLock<HashMap<&str, Token>> = LazyLock::new(|| {
    let mut keywords = HashMap::new();

    keywords.insert("import", Token::ImportKeyword);
    keywords.insert("func", Token::FunctionKeyword);
    keywords.insert("return", Token::ReturnKeyword);
    keywords.insert("if", Token::IfKeyword);
    keywords.insert("else", Token::ElseKeyword);
    keywords.insert("loop", Token::LoopKeyword);

    keywords.insert("true", Token::TrueValue);
    keywords.insert("false", Token::FalseValue);
    
    keywords
});

static SYMBOLS: LazyLock<HashMap<&str, Token>> = LazyLock::new(|| {
    let mut symbols = HashMap::new();

    symbols.insert("+", Token::AddOperator);
    symbols.insert("-", Token::SubtractOperator);
    symbols.insert("*", Token::MultiplyOperator);
    symbols.insert("/", Token::DivideOperator);
    symbols.insert("%", Token::ModuloOperator);
    symbols.insert("=", Token::AssignmentOperator);

    symbols.insert("==", Token::EqualOperator);
    symbols.insert("!=", Token::NotEqualOperator);
    
    symbols.insert(";", Token::Semicolon); // ;
    symbols.insert(",", Token::Comma); // ,
    symbols.insert(".", Token::Dot); // .
    symbols.insert(":", Token::Colon); // :
    symbols.insert("->", Token::Arrow); // ->
    symbols.insert("|>", Token::Pipeline); // |>
    
    symbols
});

pub struct Tokenizer {
    input: String
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Tokenizer { input }
    }

    fn skip_whitespace(&mut self, chars: &mut Peekable<Chars>) {
        while chars.next_if(|c| c.is_whitespace()).is_some() {}
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let input = self.input.clone();
        let mut chars = input.chars().peekable();

        while chars.peek().is_some() {
            self.skip_whitespace(&mut chars);

            match chars.next() {
                None => break,

                // Keywords
                Some(c) if c.is_alphabetic() || c == '_' => {
                    let mut identifier = String::new();
                    identifier.push(c);

                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            identifier.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if let Some(tok) = KEYWORDS.get(identifier.as_str()) {
                        let token: Token = tok.clone();
                        tokens.push(token);
                    }
                },

                Some(c) if c.is_numeric() || c == '.' => {
                    let mut identifier = String::new();
                    identifier.push(c);

                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_numeric() || next_char == '.' {
                            identifier.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    let id_string: &str = identifier.as_str();

                    if id_string.contains('.') {
                        // float
                        let value: f64 = id_string.parse::<f64>().expect("Non-valid float literal.");
                        
                        tokens.push(Token::FloatValue(value));
                    } else {
                        // int
                        let value: i64 = id_string.parse::<i64>().expect("Non-valid integer literal.");

                        tokens.push(Token::IntegerValue(value));
                    }
                },

                Some(c) => {
                    let mut identifier = String::new();
                    identifier.push(c);

                    while let Some(&next_char) = chars.peek() {
                        if !next_char.is_alphanumeric() {
                            identifier.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if let Some(tok) = SYMBOLS.get(identifier.as_str()) {
                        let token: Token = tok.clone();
                        tokens.push(token);
                    }
                }
            }
        }

        Ok(tokens)
    }
}