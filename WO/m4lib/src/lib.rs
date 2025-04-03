//! Fuck off bastard
//! This is for sucking and fucking
//! Fuck all night
//! ```
//! use cli_utils::read_stding
//! let input = read_stdin()
//! ```
//! Smell her ass

/*
Module4 - Learning the library
*/

pub mod config;
mod ut;
use std::io::{BufRead, BufReader};
use ut::header;

// Main function

// Reading from stdin which is the cli

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect(" ❌ Failed to read line");
    line.trim().to_string()
}

pub fn read_func() {
    header("Looper");
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type stop to exit)");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("You entered: {}", input.trim());
    }

    println!("Lick her ass")
}
