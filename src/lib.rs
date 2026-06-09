//! NetScript — a minimal scripting language lexer and tokenizer.
//!
//! Hexagonal architecture:
//! - `domain` — pure logic (Token, TokenType, Lexer)
//! - `ports` — trait contracts (LexerPort, Tokenizer)
//! - `adapters` — concrete implementations (CliAdapter)
//! - `app` — composition root (App)

pub mod adapters;
pub mod app;
pub mod domain;
pub mod ports;

pub use adapters::CliAdapter;
pub use app::App;
pub use domain::{Lexer, Loc, Span, Token, TokenType};
pub use ports::{LexerPort, Tokenizer};
