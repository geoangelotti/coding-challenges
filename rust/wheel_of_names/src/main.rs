use rand::{thread_rng, Rng};
use std::io::prelude::*;

const COLORS: &[(u8, u8, u8)] = &[(255, 0, 0), (0, 255, 0), (0, 0, 255), (123, 0, 123)];

fn main() {
    println!("Wheel of names!");
    let names = read_stdin();
    println!("Winner {}", get_winner(names));
}

fn read_stdin() -> Vec<String> {
    use rand::seq::SliceRandom;
    let stdin = std::io::stdin();
    let mut rng = thread_rng();
    let mut names: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    println!();
    names.shuffle(&mut rng);
    names
}

fn sleep_time(i: usize) {
    use std::thread::sleep;
    use std::time::Duration;
    let sleep_ms = 15 + i * 5;
    let sleep_duration = Duration::from_millis(sleep_ms as u64);
    sleep(sleep_duration);
}

fn colored(c: (u8, u8, u8), text: &str) -> String {
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.0, c.1, c.2, text)
}

fn get_winner(names: Vec<String>) -> String {
    let stop = rand::thread_rng().gen_range(7 * names.len()..12 * names.len());
    for (i, name) in names.iter().cycle().enumerate() {
        sleep_time(i);
        let color: (u8, u8, u8) = COLORS[i % 4];
        println!("{}", colored(color, name));
        if i == stop {
            sleep_time(i);
            return colored(color, name);
        }
    }
    "".into()
}
