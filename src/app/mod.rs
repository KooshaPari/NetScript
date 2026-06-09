//! App layer — composition root. Wires adapters to domain.

use crate::adapters::CliAdapter;
use crate::domain::{Lexer, Token};

pub struct App {
    adapter: CliAdapter,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            adapter: CliAdapter::new(),
        }
    }

    pub fn run_cli(&self) -> std::io::Result<()> {
        self.adapter.run_interactive()
    }

    pub fn run_once(&self, input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        lexer.tokenize()
    }
}
