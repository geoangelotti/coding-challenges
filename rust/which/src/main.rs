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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_paths_found() {
        // 'ls' should exist on most Unix systems
        #[cfg(not(windows))]
        assert!(get_paths("ls").is_some());

        // 'cmd' should exist on Windows
        #[cfg(windows)]
        assert!(get_paths("cmd").is_some());
    }

    #[test]
    fn test_get_paths_not_found() {
        assert!(get_paths("definitelynotarealprogram").is_none());
    }

    #[test]
    fn test_parse_arguments_empty() {
        let args = vec!["which".to_string()];
        let result = parse_arguments(args);
        assert!(result.is_err())
    }

    #[test]
    fn test_parse_arguments_without_flag() -> Result<(), Box<dyn std::error::Error>> {
        let args = vec!["which".to_string(), "ls".to_string()];
        let result = parse_arguments(args);
        assert!(result.is_ok());
        let (flag_all, programs) = result?;
        assert!(!flag_all);
        assert_eq!(programs.len(), 1);
        Ok(())
    }

    #[test]
    fn test_parse_arguments_with_flag() -> Result<(), Box<dyn std::error::Error>> {
        let args = vec!["which".to_string(), "-a".to_string(), "ls".to_string()];
        let result = parse_arguments(args);
        assert!(result.is_ok());
        let (flag_all, programs) = result?;
        assert!(flag_all);
        assert_eq!(programs.len(), 1);
        Ok(())
    }
}
