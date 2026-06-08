//! NetScript CLI entry point.

use netscript::App;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let input = &args[1];
        let _app = App::new();
        let adapter = netscript::CliAdapter::new();
        let tokens = adapter.run_once(input);
        for token in tokens {
            println!("{:?}", token);
        }
        Ok(())
    } else {
        App::new().run_cli()
    }
}
