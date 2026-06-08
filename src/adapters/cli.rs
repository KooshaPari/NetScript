use std::io::{self, BufRead, Write};

use crate::domain::{Lexer, Token};
use crate::ports::LexerPort;

/// CLI adapter that reads from stdin and writes tokenized output.
pub struct CliAdapter;

impl CliAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn run_interactive(&self) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdout_lock = stdout.lock();
        let mut stderr = io::stderr();

        writeln!(stderr, "NetScript Lexer (interactive mode)")?;
        writeln!(stderr, "Type expressions and press Enter. Ctrl-D to exit.")?;

        for line in stdin.lock().lines() {
            let input = line?;
            let mut lexer = Lexer::new(&input);
            let tokens = lexer.tokenize();
            for token in tokens {
                writeln!(stdout_lock, "{:?}", token)?;
            }
        }
        Ok(())
    }

    pub fn run_once(&self, input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        lexer.tokenize()
    }
}
