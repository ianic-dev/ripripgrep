use ripripgrep::Config;
use std::env;
use std::process;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    let runconfig = Config::runconfig(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if runconfig.version {
        let version: String = VERSION.unwrap_or("unknown").to_string();
        eprintln!("ripripgrep {version}");
        return;
    }

    if let Err(e) = ripripgrep::run(runconfig) {
        eprintln!("Execution error: {e}");
        process::exit(1);
    }
}
