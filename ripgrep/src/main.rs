use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
struct Config {
    query: String,
    filename: Option<String>,
}

impl Config {
    pub fn new(args: &[String] ) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Wrong argument count");
        }
        Ok(Config { query: args[1].to_string(), filename: args.get(2).map(String::from) })

    }
}

fn read_file(path: &str) -> io::Result<Vec<u8>> {
    match fs::exists(path) {
        Ok(true) => fs::read(path),
        _ => Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Error parsing arguments: {e}");
        std::process::exit(1);

    });
}
