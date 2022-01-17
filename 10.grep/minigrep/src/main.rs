use minigrep::run;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("query: {}", config.query);
    println!("filename: {}", config.filename);
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
