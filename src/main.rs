//! NetScript CLI entry point.
//!
//! The lexer, token types, and tests live in `lib.rs`. This binary is a
//! thin shim so future parsing/REPL work can grow here without
//! duplicating the lexer, and so the lexer is reusable as a library
//! (e.g. for embedding in other tools, or for a forthcoming `netscript fmt`).

fn main() {
    println!("NetScript Lexer");
    println!("Usage: echo 'let x = 42;' | netscript");
}
