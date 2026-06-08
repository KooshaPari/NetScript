//! Domain layer — pure logic, no IO, no framework dependencies.

pub mod lexer;

pub use lexer::{Lexer, Token, TokenType};
