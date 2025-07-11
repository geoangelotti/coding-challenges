use std::{
    env,
    path::{Path, PathBuf},
};

fn get_seperator() -> char {
    if cfg!(windows) { ';' } else { ':' }
}

fn get_paths(program: &str) -> Option<Vec<PathBuf>> {
    Some(
        env::var("PATH")
            .ok()?
            .split(get_seperator())
            .map(|path| Path::new(path).join(program))
            .filter(|path| path.exists())
            .collect::<Vec<PathBuf>>(),
    )
    .filter(|v| !v.is_empty())
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
        match get_paths(program.as_str()) {
            Some(paths) => paths.iter().for_each(|path| println!("{}", path)),
            None => println!("{} not found in PATH", program),
        }
    }
    Ok(())
}
