//! Adapters layer — concrete implementations of ports.

pub mod cli;
pub mod repl;

pub use cli::CliAdapter;
pub use repl::ReplAdapter;
