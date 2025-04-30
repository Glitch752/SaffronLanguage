use std::{collections::HashMap, iter::Peekable, str::Chars, sync::LazyLock};

#[derive(Clone, Debug, PartialEq)]
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

    StringLiteral(String), // "hello", "world", etc.
    IntegerLiteral(i64), // 0, 1, 2, etc.
    FloatLiteral(f64), // 0.0, 0.1, 0.2, etc.
    CharLiteral(char), // 'a', 'b', 'c', etc.

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

impl Token {
    pub fn reverse_format(&self) -> String {
        match self {
            Token::ImportKeyword => "import".to_string(),
            Token::FunctionKeyword => "func".to_string(),
            Token::ReturnKeyword => "return".to_string(),
            Token::IfKeyword => "if".to_string(),
            Token::ElseKeyword => "else".to_string(),
            Token::LoopKeyword => "loop".to_string(),

            Token::TrueValue => "true".to_string(),
            Token::FalseValue => "false".to_string(),

            Token::StringLiteral(value) => format!("\"{}\"", value),
            Token::IntegerLiteral(value) => value.to_string(),
            Token::FloatLiteral(value) => value.to_string(),
            Token::CharLiteral(value) => format!("'{}'", value),

            Token::Identifier(value) => value.clone(),

            _ => {
                if let Some(symbol) = SYMBOLS.iter().find(|(_, v)| v == &self) {
                    symbol.0.to_string()
                } else if let Some(keyword) = KEYWORDS.iter().find(|(_, v)| v == &self) {
                    keyword.0.to_string()
                } else {
                    format!("{:?}", self)
                }
            }
        }
    }
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

    symbols.insert(">=", Token::GreaterThanEqualOperator);
    symbols.insert("<=", Token::LessThanEqualOperator);
    symbols.insert("==", Token::EqualOperator);
    symbols.insert("!=", Token::NotEqualOperator);
    
    symbols.insert(";", Token::Semicolon);
    symbols.insert(",", Token::Comma);
    symbols.insert(".", Token::Dot);
    symbols.insert(":", Token::Colon);
    symbols.insert("->", Token::Arrow);
    symbols.insert("|>", Token::Pipeline);

    symbols.insert("(", Token::OpenParenthesis);
    symbols.insert(")", Token::CloseParenthesis);
    symbols.insert("{", Token::OpenCurlyBracket);
    symbols.insert("}", Token::CloseCurlyBracket);
    symbols.insert("[", Token::OpenSquareBracket);
    symbols.insert("]", Token::CloseSquareBracket);
    symbols.insert("<", Token::OpenAngleBracket);
    symbols.insert(">", Token::CloseAngleBracket);
    
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

                // Keywords and identifiers
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
                    } else {
                        tokens.push(Token::Identifier(identifier));
                    }
                },

                Some(c) if c.is_numeric() => {
                    let mut number = String::new();
                    number.push(c);

                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_numeric() || next_char == '.' {
                            number.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if number.contains('.') {
                        if let Ok(value) = number.parse::<f64>() {
                            tokens.push(Token::FloatLiteral(value));
                        } else {
                            return Err(format!("Invalid float value: {}", number));
                        }
                    } else {
                        if let Ok(value) = number.parse::<i64>() {
                            tokens.push(Token::IntegerLiteral(value));
                        } else {
                            return Err(format!("Invalid integer value: {}", number));
                        }
                    }
                },

                // Floats starting with a dot
                Some('.') if chars.peek().is_some_and(|c| c.is_numeric()) => {
                    let mut number = String::new();
                    number.push('.');

                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_numeric() {
                            number.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if let Ok(value) = number.parse::<f64>() {
                        tokens.push(Token::FloatLiteral(value));
                    } else {
                        return Err(format!("Invalid float value: {}", number));
                    }
                },

                // Handle comments
                Some('/') if chars.peek().is_some_and(|&c| c == '/') => {
                    // Skip the rest of the line
                    while chars.next_if(|&c| c != '\n').is_some() {}
                },
                Some('/') if chars.peek().is_some_and(|&c| c == '*') => {
                    // Skip block comments
                    chars.next(); // Consume the '*'
                    while let Some(&c) = chars.peek() {
                        if c == '*' {
                            chars.next(); // Consume the '*'
                            if chars.peek() == Some(&'/') {
                                chars.next(); // Consume the '/'
                                break;
                            }
                        } else {
                            chars.next(); // Consume the character
                        }
                    }
                },

                // Strings
                Some('"') => {
                    let mut string_value = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == '"' {
                            chars.next(); // Consume the closing quote
                            break;
                        } else if c == '\\' {
                            chars.next(); // Consume the backslash
                            if let Some(&escaped_char) = chars.peek() {
                                string_value.push(escaped_char);
                                chars.next(); // Consume the escaped character
                            }
                        } else {
                            string_value.push(c);
                            chars.next(); // Consume the character
                        }
                    }
                    tokens.push(Token::StringLiteral(string_value));
                },

                Some(c) => {
                    if let Some(&next_char) = chars.peek() {
                        // Check for 2-character symbols
                        let two_char_symbol = format!("{}{}", c, next_char);
                        if let Some(tok) = SYMBOLS.get(two_char_symbol.as_str()) {
                            let token: Token = tok.clone();
                            tokens.push(token);
                            chars.next(); // Consume the second character
                            continue;
                        }
                    }
                    
                    if let Some(tok) = SYMBOLS.get(c.to_string().as_str()) {
                        // Check for single-character symbols
                        let token: Token = tok.clone();
                        tokens.push(token);
                        continue;
                    }
                    
                    if c == '\'' {
                        // Handle character literals
                        if let Some(&next_char) = chars.peek() {
                            if next_char != '\'' {
                                tokens.push(Token::CharLiteral(next_char));
                                chars.next(); // Consume the character
                            } else {
                                return Err("Empty character literal".to_string());
                            }
                        }
                        chars.next(); // Consume the closing quote
                        continue;
                    }
                    
                    if c.is_alphanumeric() || c == '_' {
                        let mut identifier = String::new();
                        identifier.push(c);

                        while let Some(&next_char) = chars.peek() {
                            if next_char.is_alphanumeric() || next_char == '_' {
                                identifier.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }

                        tokens.push(Token::Identifier(identifier));
                        continue;
                    }
                    
                    return Err(format!("Unexpected character: {}", c));
                }
            }
        }

        Ok(tokens)
    }
}