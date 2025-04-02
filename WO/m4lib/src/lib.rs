/*
Module4 - Learning the library
*/

mod ut;

use std::io::{BufRead, BufReader};

use ut::{clear_console, divy, header, print_with_synthwave_gradient};

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
