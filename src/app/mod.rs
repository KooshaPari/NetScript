//! App layer — composition root. Wires adapters to domain.

use crate::adapters::CliAdapter;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn run_cli(&self) -> std::io::Result<()> {
        let adapter = CliAdapter::new();
        adapter.run_interactive()
    }
}
