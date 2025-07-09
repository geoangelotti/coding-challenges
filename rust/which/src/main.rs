use std::{env, path::Path};

fn get_seperator() -> char {
    if cfg!(windows) { ';' } else { ':' }
}

fn get_path(program: &String) -> Option<String> {
    if let Ok(path_variables) = env::var("PATH") {
        for path in path_variables.split(get_seperator()) {
            let executable_path = Path::new(path).join(program);
            if executable_path.exists() {
                return Some(executable_path.to_string_lossy().to_string());
            }
        }
    };
    None
}

const USAGE: &str = "Usage: which program ...";
fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        println!("{}", USAGE);
        return;
    }
    let program = &arguments[1];
    match get_path(program) {
        Some(path) => println!("{}", path),
        None => println!("{} not found in PATH", program),
    }
}
