use std::{env, path::Path};

fn get_seperator() -> char {
    if cfg!(windows) { ';' } else { ':' }
}

fn get_path(program: &String) -> Option<String> {
    env::var("PATH")
        .ok()?
        .split(get_seperator())
        .map(|path| Path::new(path).join(program))
        .find(|path| path.exists())
        .map(|p| p.to_string_lossy().to_string())
}

const USAGE: &str = "Usage: which program ...";

fn parse_arguments() -> Result<Vec<String>, String> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        return Err(USAGE.to_string());
    }
    Ok(arguments)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = parse_arguments()?;
    let programs = &arguments[1..];
    for program in programs {
        match get_path(program) {
            Some(path) => println!("{}", path),
            None => println!("{} not found in PATH", program),
        }
    }
    Ok(())
}
