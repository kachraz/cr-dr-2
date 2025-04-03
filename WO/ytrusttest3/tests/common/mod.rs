/*
Setting up tests in their own folder for organization
*/

use lolcrab::Lolcrab;
use std::io::{self, BufReader};

pub fn setup() {
    mecol();
    println!("🍫🍫🍫🍫🍫🍫🍫🍫Setting up tests");
}

fn mecol() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut lol = Lolcrab::new(None, None);
    lol.gradient = Box::new(colorgrad::preset::viridis());
}
