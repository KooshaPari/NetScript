use std::str::FromStr;

use crate::ports::LexerPort;

// Token types for the NetScript lexer
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),
    Float(f64),
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
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
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

    fn read_number(&mut self) -> TokenType {
        let start = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        if self.ch == '.' && self.peek_char().is_ascii_digit() {
            self.read_char(); // consume '.'
            while self.ch.is_ascii_digit() {
                self.read_char();
            }
            let num_str: String = self.input[start..self.position].iter().collect();
            return f64::from_str(&num_str)
                .map(TokenType::Float)
                .unwrap_or(TokenType::Illegal);
        }
        let num_str: String = self.input[start..self.position].iter().collect();
        i64::from_str(&num_str)
            .map(TokenType::Integer)
            .unwrap_or(TokenType::Illegal)
    }

    fn read_string(&mut self) -> String {
        let mut result = String::new();
        self.read_char(); // consume opening quote
        while self.ch != '"' && self.ch != '\0' {
            if self.ch == '\\' {
                self.read_char();
                match self.ch {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    _ => result.push(self.ch),
                }
            } else {
                result.push(self.ch);
            }
            self.read_char();
        }
        result
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
                    let token_type = self.read_number();
                    return Token::new(token_type, self.line, self.column);
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

impl LexerPort for Lexer {
    fn next_token(&mut self) -> Token {
        Lexer::next_token(self)
    }

    fn tokenize(&mut self) -> Vec<Token> {
        Lexer::tokenize(self)
    }
}
