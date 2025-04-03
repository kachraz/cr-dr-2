/*
Setting up tests in their own folder for organization
*/

use lolcrab::Lolcrab;
use noise::NoiseFn;
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

fn mecol() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut lol = Lolcrab::new(None, None);
    lol.gradient = Box::new(colorgrad::preset::plasma());
    lol.noise = Box::new(noise::NoiseFn1D::new(0.0, 0.1));

    lol.colorize_str(TEXT, &mut stdout)?;

    Ok(())
}

/*
pub fn new(
gradient: Option<Box<dyn colorgrad::Gradient>>,
ns: Option<Box<dyn noise::NoiseFn<f64, 2>>>
) -> Self
- This is the proper way to use
*/
