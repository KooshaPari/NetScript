//! App layer — composition root. Wires adapters to domain.

use crate::adapters::CliAdapter;

pub struct App;

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn run_cli(&self) -> std::io::Result<()> {
        let adapter = CliAdapter::new();
        adapter.run_interactive()
    }
}
