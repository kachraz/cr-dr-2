/*
Setting up tests in their own folder for organization
*/

use lolcrab::Lolcrab;
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

// fn mecol() -> Result<(), Box<dyn std::error::Error>> {
//     let stdout = io::stdout();
//     let mut stdout = stdout.lock();

//     let mut lol = Lolcrab::new(None, None);

//     lol.colorize_str(TEXT, &mut stdout)?;

//     Ok(())
// }

fn mecol() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Initialize Lolcrab with the "Magma" gradient
    let mut lol = Lolcrab::new(Some("magma".to_string()), None);

    // Define the text you want to colorize
    let text = "Your text here";

    // Colorize the text
    lol.colorize_str(text, &mut stdout)?;

    Ok(())
}
