use std::{io, env};

use crate::days::*;

mod days;

fn main() {
    println!("Advent of Code 2021");
    println!("-- -- -- -- -- -- -- --");
    println!("Enter the day to run:");

    let args: Vec<String> = env::args().collect();
    let mut day = match args.get(1) {
        None => String::new(),
        Some(arg) => arg.to_string()
    };

    if day.is_empty() {
        io::stdin()
        .read_line(&mut day)
        .expect("Failed to read day");
    }

    match day.replace("\n", "").as_str() {
        "1" => one::run(),
        _ => println!("That isn't available yet")
    }
}
