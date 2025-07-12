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

const USAGE: &str = "Usage: which [-as] program ...";

fn parse_arguments() -> Result<(bool, bool, Vec<String>), &'static str> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        return Err(USAGE);
    }

    let mut flag_all = false;
    let mut flag_silent = false;
    let mut programs: Vec<String> = Vec::new();

    arguments
        .iter()
        .for_each(|argument| match argument.as_str() {
            "-a" => flag_all = true,
            "-s" => flag_silent = true,
            "-as" => {
                flag_all = true;
                flag_silent = true;
            }
            _ => programs.push(argument.clone()),
        });
    if programs.is_empty() {
        return Err(USAGE);
    }
    Ok((flag_all, flag_silent, programs))
}

fn iterate(programs: &[String], flag_all: bool, flag_silent: bool) {
    for program in programs {
        display(program, get_paths(program), flag_all);
    }
}

fn display(program: &str, paths: Option<Vec<PathBuf>>, flag_all: bool) {
    match paths {
        Some(paths) => {
            let iter: Box<dyn Iterator<Item = &PathBuf>> = if flag_all {
                Box::new(paths.iter())
            } else {
                Box::new(paths.iter().take(1))
            };
            iter.for_each(|path| println!("{}", path.to_string_lossy()));
        }
        None => println!("{} not found in PATH", program),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (flag_all, flag_silent, arguments) = parse_arguments()?;
    let programs = &arguments[1..];
    iterate(programs, flag_all, flag_silent);
    Ok(())
}
