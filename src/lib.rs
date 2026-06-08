//! NetScript — a minimal scripting language lexer and tokenizer.
//!
//! Hexagonal architecture:
//! - `domain` — pure logic (Token, TokenType, Lexer)
//! - `ports` — trait contracts (LexerPort, Tokenizer)
//! - `adapters` — concrete implementations (CliAdapter)
//! - `app` — composition root (App)

pub mod domain;
pub mod ports;
pub mod adapters;
pub mod app;

pub use domain::{Lexer, Token, TokenType};
pub use ports::{LexerPort, Tokenizer};
pub use adapters::CliAdapter;
pub use app::App;
