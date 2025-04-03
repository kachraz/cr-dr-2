/*
Setting up tests in their own folder for organization
*/

use lolcrab::{Gradient, Lolcrab};
use std::io::{self, Write}; // Import Write to ensure stdout works properly.

pub fn setup() {
    mecol();
    println!("🍫🍫🍫🍫🍫🍫🍫🍫Setting up tests");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Hello, colorful world!";
    let mut stdout = io::stdout();

    // Create a Lolcrab instance with a Rainbow gradient and default noise.
    let mut lol = Lolcrab::new(Some(Gradient::Rainbow), None);

    // Apply the gradient effect to the text
    let colorful_text = lol.colorize(text);

    // Write the colorful text to stdout
    writeln!(stdout, "{}", colorful_text)?;

    Ok(())
}
