/*
Setting up tests in their own folder for organization
*/

use lolcrab::{Gradient, Lolcrab};
use std::io;

pub fn setup() {
    mecol();
    println!("🍫🍫🍫🍫🍫🍫🍫🍫Setting up tests");
}

fn mecol() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Hello, colorful world!";
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut lol = Lolcrab::new(Some(Gradient::Rainbow), None);

    lol.colorize_str(text, &mut stdout)?;

    Ok(())
}
