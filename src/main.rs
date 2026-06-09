//! NetScript CLI entry point.

use netscript::App;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let input = &args[1];
        let app = App::new();
        let tokens = app.run_once(input);
        for token in tokens {
            println!("{:?}", token);
        }
        Ok(())
    } else {
        App::new().run_cli()
    }
}
