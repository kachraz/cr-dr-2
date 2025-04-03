/*
Setting up tests in their own folder for organization
*/

use lolcrab::{Gradient, Lolcrab};
use std::io::{self, Write}; // Import Write to ensure stdout works properly.

pub fn setup() {
    mecol();
    println!("🍫🍫🍫🍫🍫🍫🍫🍫Setting up tests");
}

fn mecol() -> Result<(), Box<dyn std::error::Error>> {
    let text = "WomanBoobSniff";
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Create a Lolcrab instance with a Rainbow gradient and default noise.
    let mut lol = Lolcrab::new(Some(Gradient::Rainbow), None);

    // Apply the gradient effect to the text
    lol.colorize_str(text, &mut stdout)?;
    stdout.flush()?; // Ensure output is immediately flushed

    Ok(())
}
