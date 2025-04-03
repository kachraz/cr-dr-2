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
██████╗   ██████╗   ██████╗  ████████╗ ██╗   ██╗
██╔══██╗ ██╔═══██╗ ██╔═══██╗ ╚══██╔══╝ ╚██╗ ██╔╝
██████╔╝ ██║   ██║ ██║   ██║    ██║     ╚████╔╝ 
██╔══██╗ ██║   ██║ ██║   ██║    ██║      ╚██╔╝  
██████╔╝ ╚██████╔╝ ╚██████╔╝    ██║       ██║   
╚═════╝   ╚═════╝   ╚═════╝     ╚═╝       ╚═╝   
";

// fn mecol() -> Result<(), Box<dyn std::error::Error>> {
//     let stdout = io::stdout();
//     let mut stdout = stdout.lock();

//     let mut lol = Lolcrab::new(None, None);

//     lol.colorize_str(TEXT, &mut stdout)?;

//     Ok(())
// }

/*
pub fn new(
gradient: Option<Box<dyn colorgrad::Gradient>>,
ns: Option<Box<dyn noise::NoiseFn<f64, 2>>>
) -> Self
- This is the proper way to use
*/

fn mecol() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut lol = Lolcrab::new(None, None);
    lol.gradient = Box::new(colorgrad::preset::plasma());
    lol.ns = Box::new(noise::NoiseFn::(1, 2));

    lol.colorize_str(TEXT, &mut stdout)?;

    Ok(())
}


fn mecol2() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Initialize Lolcrab with the Magma gradient and Perlin noise
    let mut lol = Lolcrab::new(
        Some(Box::new(colorgrad::preset::plasma())),
        Some(Box::noise(NoiseFn(1, 2))),
    );

    // Define the text you want to colorize
    let text = "Your text here";

    // Colorize the text
    lol.colorize_str(text, &mut stdout)?;

    Ok(())
}
