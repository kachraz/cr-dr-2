/*
Setting up tests in their own folder for organization
*/

use lolcrab::{Gradient, Lolcrab};
use std::io::{self, Write}; // Import Write to ensure stdout works properly.

pub fn setup() {
    let text = "Hello, colorful world!";
    let gradient = Gradient::Rainbow; // Available: Rainbow, Lolcrab, Fire, etc.
    let colored_text = lolcrab::Gradient(text, gradient);
    println!("{}", colored_text);
    println!("🍫🍫🍫🍫🍫🍫🍫🍫Setting up tests");
}
