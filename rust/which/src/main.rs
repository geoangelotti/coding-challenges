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

const USAGE: &str = "Usage: which [-a] program ...";

fn parse_arguments<I>(args: I) -> Result<(bool, Vec<String>), &'static str>
where
    I: IntoIterator<Item = String>,
{
    let mut flag_all = false;
    let programs: Vec<String> = args
        .into_iter()
        .skip(1)
        .filter_map(|arg| match arg.as_str() {
            "-a" => {
                flag_all = true;
                None
            }
            _ => Some(arg),
        })
        .collect();

    if programs.is_empty() {
        Err(USAGE)
    } else {
        Ok((flag_all, programs))
    }
}

fn iterate(programs: &[String], flag_all: bool) {
    for program in programs {
        display(program, get_paths(program), flag_all);
    }
}

fn display(program: &str, paths: Option<Vec<PathBuf>>, flag_all: bool) {
    match paths {
        Some(paths) => {
            let iter = paths.iter().take(if flag_all { paths.len() } else { 1 });
            for path in iter {
                println!("{}", path.to_string_lossy());
            }
        }
        None => println!("{} not found in PATH", program),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (flag_all, arguments) = parse_arguments(env::args())?;
    let programs = &arguments[..];
    iterate(programs, flag_all);
    Ok(())
}
