use std::str::FromStr;

// Token types for the NetScript lexer
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),
    String(String),
    Identifier(String),
    Boolean(bool),

    // Keywords
    Let,
    Fn,
    If,
    Else,
    While,
    Return,
    Print,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equals,
    EqualsEquals,
    BangEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    Bang,
    And,
    Or,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,

    // Special
    Eof,
    Illegal,
}

// Token with position information
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self { token_type, line, column }
    }
}

// Lexer for NetScript
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let input_chars: Vec<char> = input.chars().collect();
        let mut lexer = Self {
            input: input_chars,
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> i64 {
        let start = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let num_str: String = self.input[start..self.position].iter().collect();
        i64::from_str(&num_str).unwrap_or(0)
    }

    fn read_string(&mut self) -> String {
        let start_pos = self.position + 1;
        self.read_char(); // consume opening quote
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        // Do NOT consume closing quote - next_token() will call read_char()
        // which will advance past it, leaving self.ch at the next token
        let end_pos = self.position;
        self.input[start_pos..end_pos].iter().collect()
    }

    fn skip_comment(&mut self) {
        while self.ch != '\n' && self.ch != '\0' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '+' => Token::new(TokenType::Plus, self.line, self.column),
            '-' => Token::new(TokenType::Minus, self.line, self.column),
            '*' => Token::new(TokenType::Star, self.line, self.column),
            '/' => {
                if self.peek_char() == '/' {
                    self.skip_comment();
                    return self.next_token();
                }
                Token::new(TokenType::Slash, self.line, self.column)
            }
            '%' => Token::new(TokenType::Percent, self.line, self.column),
            '(' => Token::new(TokenType::LeftParen, self.line, self.column),
            ')' => Token::new(TokenType::RightParen, self.line, self.column),
            '{' => Token::new(TokenType::LeftBrace, self.line, self.column),
            '}' => Token::new(TokenType::RightBrace, self.line, self.column),
            ';' => Token::new(TokenType::Semicolon, self.line, self.column),
            ',' => Token::new(TokenType::Comma, self.line, self.column),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::EqualsEquals, self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Equals, self.line, self.column)
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::BangEquals, self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Bang, self.line, self.column)
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::LessEquals, self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Less, self.line, self.column)
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::GreaterEquals, self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Greater, self.line, self.column)
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    self.read_char();
                    Token::new(TokenType::And, self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Illegal, self.line, self.column)
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    self.read_char();
                    Token::new(TokenType::Or, self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Illegal, self.line, self.column)
                }
            }
            '"' => {
                let s = self.read_string();
                Token::new(TokenType::String(s), self.line, self.column)
            }
            '\0' => Token::new(TokenType::Eof, self.line, self.column),
            _ => {
                if self.ch.is_ascii_digit() {
                    let num = self.read_number();
                    return Token::new(TokenType::Integer(num), self.line, self.column);
                } else if self.ch.is_ascii_alphabetic() || self.ch == '_' {
                    let id = self.read_identifier();
                    let keyword = match id.as_str() {
                        "let" => TokenType::Let,
                        "fn" => TokenType::Fn,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        "return" => TokenType::Return,
                        "print" => TokenType::Print,
                        "true" => TokenType::Boolean(true),
                        "false" => TokenType::Boolean(false),
                        _ => TokenType::Identifier(id),
                    };
                    return Token::new(keyword, self.line, self.column);
                } else {
                    Token::new(TokenType::Illegal, self.line, self.column)
                }
            }
        };

        self.read_char();
        token
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token.clone());
            if token.token_type == TokenType::Eof {
                break;
            }
        }
        tokens
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_token() {
        let input = "42";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.token_type, TokenType::Integer(42));
    }

    #[test]
    fn test_string_token() {
        let input = "\"hello world\"";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        match token.token_type {
            TokenType::String(s) => assert_eq!(s, "hello world"),
            _ => panic!("Expected String token, got {:?}", token.token_type),
        }
    }

    #[test]
    fn test_identifier() {
        let input = "myVariable";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        match token.token_type {
            TokenType::Identifier(id) => assert_eq!(id, "myVariable"),
            _ => panic!("Expected Identifier token, got {:?}", token.token_type),
        }
    }

    #[test]
    fn test_keywords() {
        let input = "let fn if else while return print";
        let mut lexer = Lexer::new(input);
        let expected = vec![
            TokenType::Let,
            TokenType::Fn,
            TokenType::If,
            TokenType::Else,
            TokenType::While,
            TokenType::Return,
            TokenType::Print,
        ];
        for exp in expected {
            let token = lexer.next_token();
            assert_eq!(token.token_type, exp);
        }
    }

    #[test]
    fn test_operators() {
        let input = "+ - * / % == != < <= > >= ! && ||";
        let mut lexer = Lexer::new(input);
        let expected = vec![
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Percent,
            TokenType::EqualsEquals,
            TokenType::BangEquals,
            TokenType::Less,
            TokenType::LessEquals,
            TokenType::Greater,
            TokenType::GreaterEquals,
            TokenType::Bang,
            TokenType::And,
            TokenType::Or,
        ];
        for exp in expected {
            let token = lexer.next_token();
            assert_eq!(token.token_type, exp);
        }
    }

    #[test]
    fn test_delimiters() {
        let input = "( ) { } ; ,";
        let mut lexer = Lexer::new(input);
        let expected = vec![
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Semicolon,
            TokenType::Comma,
        ];
        for exp in expected {
            let token = lexer.next_token();
            assert_eq!(token.token_type, exp);
        }
    }

    #[test]
    fn test_booleans() {
        let input = "true false";
        let mut lexer = Lexer::new(input);
        let token1 = lexer.next_token();
        match token1.token_type {
            TokenType::Boolean(b) => assert!(b),
            _ => panic!("Expected Boolean(true)"),
        }
        let token2 = lexer.next_token();
        match token2.token_type {
            TokenType::Boolean(b) => assert!(!b),
            _ => panic!("Expected Boolean(false)"),
        }
    }

    #[test]
    fn test_simple_assignment() {
        let input = "let x = 42;";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<TokenType> = lexer.tokenize().iter().map(|t| t.token_type.clone()).collect();
        let expected = vec![
            TokenType::Let,
            TokenType::Identifier("x".to_string()),
            TokenType::Equals,
            TokenType::Integer(42),
            TokenType::Semicolon,
            TokenType::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_function_call() {
        let input = "print(\"hello\");";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<TokenType> = lexer.tokenize().iter().map(|t| t.token_type.clone()).collect();
        let expected = vec![
            TokenType::Print,
            TokenType::LeftParen,
            TokenType::String("hello".to_string()),
            TokenType::RightParen,
            TokenType::Semicolon,
            TokenType::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_if_statement() {
        let input = "if x > 0 { return 1; }";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<TokenType> = lexer.tokenize().iter().map(|t| t.token_type.clone()).collect();
        let expected = vec![
            TokenType::If,
            TokenType::Identifier("x".to_string()),
            TokenType::Greater,
            TokenType::Integer(0),
            TokenType::LeftBrace,
            TokenType::Return,
            TokenType::Integer(1),
            TokenType::Semicolon,
            TokenType::RightBrace,
            TokenType::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comments_skipped() {
        let input = "let x = 1; // this is a comment\nlet y = 2;";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<TokenType> = lexer.tokenize().iter().map(|t| t.token_type.clone()).collect();
        // Should skip the comment and continue tokenizing
        assert!(tokens.contains(&TokenType::Let));
        assert!(tokens.contains(&TokenType::Identifier("x".to_string())));
        assert!(tokens.contains(&TokenType::Identifier("y".to_string())));
    }
}
