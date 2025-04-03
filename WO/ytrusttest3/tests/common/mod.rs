/*
Setting up tests in their own folder for organization
*/

use lolcrab::{Gradient, Rainbow};

pub fn setup() {
    let text = "Hello, colorful world!";
    let gradient = Gradient::Rainbow; // You can choose other gradients like 'Lolcrab', 'Fire', etc.
    let rainbow = Rainbow::new(gradient, 0.1, 0.0);
    for (colored_char, _) in rainbow.wrap(text.chars()) {
        print!("{}", colored_char);
    }
    println!();
    println!("🍫🍫🍫🍫🍫🍫🍫🍫Setting up tests");
}
