/*
Setting up tests in their own folder for organization
*/

use lolcrab::{Gradient, Lolcrab};
use std::io;

pub fn setup() {
    mecol();
    println!("🍫🍫🍫🍫🍫🍫🍫🍫Setting up tests");
}
const TEXT: &str = "\
•••••••••••••••••••••••••••••••••••••••••••
••56505384476•••••••••••••••••39761609699••
••47928752907•• { lolcrab } ••33810561851••
•••••••••••••••••••••••••••••••••••••••••••
";

fn mecol() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut lol = Lolcrab::new(Some(Gradient::Cividis), None);

    lol.colorize_str(TEXT, &mut stdout)?;

    Ok(())
}
