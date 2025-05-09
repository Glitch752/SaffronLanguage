use std::{collections::{HashMap, VecDeque}, sync::LazyLock};

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // keywords
    ImportKeyword, // import
    FunctionKeyword, // func
    StructKeyword, // struct
    TypeKeyword, // type
    NewKeyword, // new

    ReturnKeyword, // return
    IfKeyword, // if
    ElseKeyword, // else
    LoopKeyword, // loop
    ConstKeyword, // const
    LetKeyword, // let
    BreakKeyword, // break
    ContinueKeyword, // continue
    
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

    AndOperator, // &&
    OrOperator, // ||
    NotOperator, // !

    // TODO: Bitwise operators
    
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

impl TokenType {
    pub fn reverse_format(&self) -> String {
        match self {
            TokenType::ImportKeyword => "import".to_string(),
            TokenType::FunctionKeyword => "func".to_string(),
            TokenType::ReturnKeyword => "return".to_string(),
            TokenType::IfKeyword => "if".to_string(),
            TokenType::ElseKeyword => "else".to_string(),
            TokenType::LoopKeyword => "loop".to_string(),
            TokenType::BreakKeyword => "break".to_string(),
            TokenType::ContinueKeyword => "continue".to_string(),
            TokenType::StructKeyword => "struct".to_string(),
            TokenType::TypeKeyword => "type".to_string(),
            TokenType::NewKeyword => "new".to_string(),

            TokenType::TrueValue => "true".to_string(),
            TokenType::FalseValue => "false".to_string(),

            TokenType::ConstKeyword => "const".to_string(),
            TokenType::LetKeyword => "let".to_string(),

            TokenType::StringLiteral(value) => format!("\"{}\"", value),
            TokenType::IntegerLiteral(value) => value.to_string(),
            TokenType::FloatLiteral(value) => value.to_string(),
            TokenType::CharLiteral(value) => format!("'{}'", value),

            TokenType::Identifier(value) => value.clone(),

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

static KEYWORDS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    let mut keywords = HashMap::new();

    keywords.insert("import", TokenType::ImportKeyword);
    keywords.insert("func", TokenType::FunctionKeyword);
    keywords.insert("struct", TokenType::StructKeyword);
    keywords.insert("type", TokenType::TypeKeyword);
    keywords.insert("new", TokenType::NewKeyword);
    
    keywords.insert("return", TokenType::ReturnKeyword);
    keywords.insert("if", TokenType::IfKeyword);
    keywords.insert("else", TokenType::ElseKeyword);
    keywords.insert("loop", TokenType::LoopKeyword);
    keywords.insert("break", TokenType::BreakKeyword);
    keywords.insert("continue", TokenType::ContinueKeyword);

    keywords.insert("true", TokenType::TrueValue);
    keywords.insert("false", TokenType::FalseValue);

    keywords.insert("const", TokenType::ConstKeyword);
    keywords.insert("let", TokenType::LetKeyword);
    
    keywords
});

static SYMBOLS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    let mut symbols = HashMap::new();

    symbols.insert("+", TokenType::AddOperator);
    symbols.insert("-", TokenType::SubtractOperator);
    symbols.insert("*", TokenType::MultiplyOperator);
    symbols.insert("/", TokenType::DivideOperator);
    symbols.insert("%", TokenType::ModuloOperator);
    symbols.insert("=", TokenType::AssignmentOperator);

    symbols.insert(">=", TokenType::GreaterThanEqualOperator);
    symbols.insert("<=", TokenType::LessThanEqualOperator);
    symbols.insert("==", TokenType::EqualOperator);
    symbols.insert("!=", TokenType::NotEqualOperator);

    symbols.insert("&&", TokenType::AndOperator);
    symbols.insert("||", TokenType::OrOperator);
    symbols.insert("!", TokenType::NotOperator);
    
    symbols.insert(";", TokenType::Semicolon);
    symbols.insert(",", TokenType::Comma);
    symbols.insert(".", TokenType::Dot);
    symbols.insert(":", TokenType::Colon);
    symbols.insert("->", TokenType::Arrow);
    symbols.insert("|>", TokenType::Pipeline);

    symbols.insert("(", TokenType::OpenParenthesis);
    symbols.insert(")", TokenType::CloseParenthesis);
    symbols.insert("{", TokenType::OpenCurlyBracket);
    symbols.insert("}", TokenType::CloseCurlyBracket);
    symbols.insert("[", TokenType::OpenSquareBracket);
    symbols.insert("]", TokenType::CloseSquareBracket);
    symbols.insert("<", TokenType::OpenAngleBracket);
    symbols.insert(">", TokenType::CloseAngleBracket);
    
    symbols
});

pub struct Tokenizer {
    characters: VecDeque<char>,
    current_line: usize,
    current_column: usize,

    tokens: Vec<Token>
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        let characters = input.chars().collect();
        Tokenizer {
            characters,
            current_line: 1,
            current_column: 1,
            tokens: Vec::<Token>::new()
        }
    }

    fn next_if<F>(&mut self, predicate: F) -> Option<char> where F: Fn(char) -> bool {
        if let Some(&c) = self.peek() {
            if predicate(c) {
                return self.next();
            }
        }
        None
    }

    fn peek(&self) -> Option<&char> {
        self.characters.get(0)
    }

    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.characters.pop_front() {
            self.current_column += 1;
            if c == '\n' {
                self.current_line += 1;
                self.current_column = 1;
            }

            return Some(c);
        }
        None
    }

    fn skip_whitespace(&mut self) {
        while self.next_if(|c| c.is_whitespace()).is_some() {}
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            line: self.current_line,
            column: self.current_column
        });
    }

    pub fn tokenize(&mut self) -> Result<&Vec<Token>, String> {
        while self.peek().is_some() {
            self.skip_whitespace();

            match self.next() {
                None => break,

                // Keywords and identifiers
                Some(c) if c.is_alphabetic() || c == '_' => {
                    let mut identifier = String::new();
                    identifier.push(c);

                    while let Some(&next_char) = self.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            identifier.push(self.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if let Some(tok) = KEYWORDS.get(identifier.as_str()) {
                        let token: TokenType = tok.clone();
                        self.add_token(token);
                    } else {
                        self.add_token(TokenType::Identifier(identifier));
                    }
                },

                Some(c) if c.is_numeric() => {
                    let mut number = String::new();
                    number.push(c);

                    while let Some(&next_char) = self.peek() {
                        if next_char.is_numeric() || next_char == '.' {
                            number.push(self.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    let mut suffix = String::new();
                    while let Some(&next_char) = self.peek() {
                        if next_char.is_alphabetic() {
                            suffix.push(self.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    // For now, no suffixes are allowed
                    if !suffix.is_empty() {
                        return Err(format!("Invalid number suffix: {}", suffix));
                    }

                    if number.contains('.') {
                        if let Ok(value) = number.parse::<f64>() {
                            self.add_token(TokenType::FloatLiteral(value));
                        } else {
                            return Err(format!("Invalid float value: {}", number));
                        }
                    } else {
                        if let Ok(value) = number.parse::<i64>() {
                            self.add_token(TokenType::IntegerLiteral(value));
                        } else {
                            return Err(format!("Invalid integer value: {}", number));
                        }
                    }
                },

                // Floats starting with a dot
                Some('.') if self.peek().is_some_and(|c| c.is_numeric()) => {
                    let mut number = String::new();
                    number.push('.');

                    while let Some(&next_char) = self.peek() {
                        if next_char.is_numeric() {
                            number.push(self.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if let Ok(value) = number.parse::<f64>() {
                        self.add_token(TokenType::FloatLiteral(value));
                    } else {
                        return Err(format!("Invalid float value: {}", number));
                    }
                },

                // Handle comments
                Some('/') if self.peek().is_some_and(|&c| c == '/') => {
                    // Skip the rest of the line
                    while self.next_if(|c| c != '\n').is_some() {}
                },
                Some('/') if self.peek().is_some_and(|&c| c == '*') => {
                    // Skip block comments
                    self.next(); // Consume the '*'
                    while let Some(&c) = self.peek() {
                        if c == '*' {
                            self.next(); // Consume the '*'
                            if self.peek() == Some(&'/') {
                                self.next(); // Consume the '/'
                                break;
                            }
                        } else {
                            self.next(); // Consume the character
                        }
                    }
                },

                // Strings
                Some('"') => {
                    // TODO: Escape sequences
                    let mut string_value = String::new();
                    while let Some(&c) = self.peek() {
                        if c == '"' {
                            self.next(); // Consume the closing quote
                            break;
                        } else if c == '\\' {
                            self.next(); // Consume the backslash
                            if let Some(&escaped_char) = self.peek() {
                                string_value.push(escaped_char);
                                self.next(); // Consume the escaped character
                            }
                        } else {
                            string_value.push(c);
                            self.next(); // Consume the character
                        }
                    }
                    self.add_token(TokenType::StringLiteral(string_value));
                },

                // Handle character literals
                Some('\'') => {
                    if let Some(&next_char) = self.peek() {
                        if next_char != '\'' {
                            self.add_token(TokenType::CharLiteral(next_char));
                            self.next(); // Consume the character
                        } else {
                            return Err("Empty character literal".to_string());
                        }
                    }
                    self.next(); // Consume the closing quote
                }

                // Handle symbols and operators
                Some(c) => {
                    if let Some(&next_char) = self.peek() {
                        // Check for 2-character symbols
                        let two_char_symbol = format!("{}{}", c, next_char);
                        if let Some(tok) = SYMBOLS.get(two_char_symbol.as_str()) {
                            let token: TokenType = tok.clone();
                            self.add_token(token);
                            self.next(); // Consume the second character
                            continue;
                        }
                    }
                    
                    if let Some(tok) = SYMBOLS.get(c.to_string().as_str()) {
                        // Check for single-character symbols
                        let token: TokenType = tok.clone();
                        self.add_token(token);
                        continue;
                    }
                    
                    return Err(format!("Unexpected character: '{}'", c));
                }
            }
        }

        Ok(&self.tokens)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let input = r#"
            import hello.world;
            func add(a, b) {
                return a + b;
            }
        "#;

        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 19);
        assert_eq!(tokens[0].token_type, TokenType::ImportKeyword);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("hello".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::Dot);
        assert_eq!(tokens[3].token_type, TokenType::Identifier("world".to_string()));
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
        assert_eq!(tokens[5].token_type, TokenType::FunctionKeyword);
        assert_eq!(tokens[6].token_type, TokenType::Identifier("add".to_string()));
        assert_eq!(tokens[7].token_type, TokenType::OpenParenthesis);
        assert_eq!(tokens[8].token_type, TokenType::Identifier("a".to_string()));
        assert_eq!(tokens[9].token_type, TokenType::Comma);
        assert_eq!(tokens[10].token_type, TokenType::Identifier("b".to_string()));
        assert_eq!(tokens[11].token_type, TokenType::CloseParenthesis);
        assert_eq!(tokens[12].token_type, TokenType::OpenCurlyBracket);
        assert_eq!(tokens[13].token_type, TokenType::ReturnKeyword);
        assert_eq!(tokens[14].token_type, TokenType::Identifier("a".to_string()));
        assert_eq!(tokens[15].token_type, TokenType::AddOperator);
        assert_eq!(tokens[16].token_type, TokenType::Identifier("b".to_string()));
        assert_eq!(tokens[17].token_type, TokenType::Semicolon);
        assert_eq!(tokens[18].token_type, TokenType::CloseCurlyBracket);
    }

    #[test]
    fn test_strings() {
        let input = r#""hello world""#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::StringLiteral("hello world".to_string()));
    }

    #[test]
    fn test_char_literals() {
        let input = r#"'a'"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::CharLiteral('a'));
    }

    #[test]
    fn test_float_literals() {
        let input = r#"3.14"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::FloatLiteral(3.14));

        let input = r#".5"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::FloatLiteral(0.5));

        let input = r#"5."#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::FloatLiteral(5.0));
    }

    #[test]
    fn test_integer_literals() {
        let input = r#"42"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::IntegerLiteral(42));
    }

    #[test]
    fn test_keywords() {
        let input = r#"import func return if else loop const let break continue"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].token_type, TokenType::ImportKeyword);
        assert_eq!(tokens[1].token_type, TokenType::FunctionKeyword);
        assert_eq!(tokens[2].token_type, TokenType::ReturnKeyword);
        assert_eq!(tokens[3].token_type, TokenType::IfKeyword);
        assert_eq!(tokens[4].token_type, TokenType::ElseKeyword);
        assert_eq!(tokens[5].token_type, TokenType::LoopKeyword);
        assert_eq!(tokens[6].token_type, TokenType::ConstKeyword);
        assert_eq!(tokens[7].token_type, TokenType::LetKeyword);
        assert_eq!(tokens[8].token_type, TokenType::BreakKeyword);
        assert_eq!(tokens[9].token_type, TokenType::ContinueKeyword);
    }

    #[test]
    fn test_operators() {
        let input = r#"+ - * / % = && || ! == != >= <= < >"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();

        let expected = vec![
            TokenType::AddOperator,
            TokenType::SubtractOperator,
            TokenType::MultiplyOperator,
            TokenType::DivideOperator,
            TokenType::ModuloOperator,
            TokenType::AssignmentOperator,
            TokenType::AndOperator,
            TokenType::OrOperator,
            TokenType::NotOperator,
            TokenType::EqualOperator,
            TokenType::NotEqualOperator,
            TokenType::GreaterThanEqualOperator,
            TokenType::LessThanEqualOperator,
            TokenType::OpenAngleBracket,
            TokenType::CloseAngleBracket
        ];

        assert_eq!(tokens.len(), expected.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, expected[i]);
        }
    }

    #[test]
    fn test_unknown_character() {
        let input = r#"$"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unexpected character: '$'".to_string());
    }

    #[test]
    fn test_empty_character_literal() {
        let input = r#"''"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Empty character literal".to_string());
    }

    #[test]
    fn test_invalid_float() {
        let input = r#"3.14.15"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid float value: 3.14.15".to_string());
    }

    #[test]
    fn test_invalid_suffix() {
        let input = r#"42abc"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid number suffix: abc".to_string());
    }
}