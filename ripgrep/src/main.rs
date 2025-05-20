use atty::Stream;
use std::env;
use std::fs;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Config {
    query: String,
    filename: Option<String>,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Wrong argument count");
        }

        let ignore_case = if let Ok(_) = env::var("IGNORE_CASE") {
            true
        } else {
            false
        };

        Ok(Config {
            query: args[1].to_string(),
            filename: args.get(2).map(String::from),
            ignore_case: ignore_case,
        })
    }
}

fn read_file(path: &str) -> io::Result<String> {
    match fs::exists(path) {
        Ok(true) => fs::read_to_string(path),
        _ => Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
    }
}

fn read_pipe() -> io::Result<String> {
    if atty::is(Stream::Stdin) {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
    }
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    let contents =
        std::str::from_utf8(buffer.as_ref()).expect("Failed to convert buffer to string");
    Ok(String::from(contents))
}

fn search<'a>(query: &str, content: &'a str, lowercase: bool) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if lowercase && line.to_lowercase().contains(query) {
            result.push(line);
        } else if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Error parsing arguments: {e}");
        std::process::exit(1);
    });

    println!("{:?}", config);

    let content = match config.filename {
        Some(filename) => read_file(&filename).unwrap_or_else(|e| {
            println!("Error reading file: {e}");
            std::process::exit(1);
        }),
        None => read_pipe().unwrap_or_else(|e| {
            std::process::exit(0);
        }),
    };

    println!(
        "{0}",
        search(&config.query, &content, config.ignore_case).join("\n")
    );
}
