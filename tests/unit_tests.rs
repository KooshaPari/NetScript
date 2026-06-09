use netscript::{Lexer, TokenType};

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
    let tokens: Vec<TokenType> = lexer
        .tokenize()
        .iter()
        .map(|t| t.token_type.clone())
        .collect();
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
    let tokens: Vec<TokenType> = lexer
        .tokenize()
        .iter()
        .map(|t| t.token_type.clone())
        .collect();
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
    let tokens: Vec<TokenType> = lexer
        .tokenize()
        .iter()
        .map(|t| t.token_type.clone())
        .collect();
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
    let tokens: Vec<TokenType> = lexer
        .tokenize()
        .iter()
        .map(|t| t.token_type.clone())
        .collect();
    assert!(tokens.contains(&TokenType::Let));
    assert!(tokens.contains(&TokenType::Identifier("x".to_string())));
    assert!(tokens.contains(&TokenType::Identifier("y".to_string())));
}

#[test]
fn test_float_token() {
    let input = "3.14";
    let mut lexer = Lexer::new(input);
    let token = lexer.next_token();
    match token.token_type {
        TokenType::Float(f) => assert!((f - 3.14).abs() < f64::EPSILON),
        _ => panic!("Expected Float token, got {:?}", token.token_type),
    }
}

#[test]
fn test_string_escape_sequences() {
    let input = "\"hello\\nworld\\ttab\\\\quote\\\"\"";
    let mut lexer = Lexer::new(input);
    let token = lexer.next_token();
    match token.token_type {
        TokenType::String(s) => assert_eq!(s, "hello\nworld\ttab\\quote\""),
        _ => panic!("Expected String token, got {:?}", token.token_type),
    }
}

#[test]
fn test_overflow_returns_illegal() {
    let input = "999999999999999999999999999999";
    let mut lexer = Lexer::new(input);
    let token = lexer.next_token();
    assert_eq!(token.token_type, TokenType::Illegal);
}
