//! Advent of Code 2021

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::path;

mod days;

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    let mut args = env::args();
    let program = args.next().unwrap();
    let path = args.next().expect(&format!("Usage: {} <input-path>", program));
    let path = path::Path::new(&path);
    if !(path.is_dir() || path.exists()) {
        println!("A directory is expected!");
    } else {
        loop {
            println!("Welcome to Advent of Code 2021!");
            print!("Please, select one day: ");
            stdout().flush()?;
            if let Err(reason) = stdin().read_line(&mut buf) {
                println!("Something were wrong: {}!", reason);
                break;
            }
            let day = match buf.trim_end().parse::<usize>() {
                Ok(day) if day > 0 && day < 26 => day,
                _ => {
                    println!("Wrong number!");
                    continue;
                }
            };
            let input = path.join(format!("day{}.txt", day));
            println!("Reading input from {}...", input.display());
            let mut file = match File::open(&input) {
                Err(reason) => panic!("Could not open {}: {}!", input.display(), reason),
                Ok(file) => file,
            };
            let mut data = String::new();
            if let Err(why) = file.read_to_string(&mut data) {
                panic!("Could not read file {}: {}", input.display(), why);
            }
            match days::run(day, &data) {
                Ok(result) => println!("Result: {}", result),
                Err(reason) => println!("Error: {}", reason),
            };
        }
    }
    Ok(())
}
