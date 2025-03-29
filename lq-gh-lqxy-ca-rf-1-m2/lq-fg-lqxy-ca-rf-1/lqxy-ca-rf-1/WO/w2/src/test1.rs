// use yansi and to write some colored text
#![allow(dead_code)] // Required to remove dead code warning
#![allow(unused_imports)] // Allowin unused imports

use cfonts::{BgColors, Colors, Fonts, Options, Rgb, say};
use yansi::Paint;

// Simple colored text with yansi
pub fn col_text1() {
    println!(
        "{} {} {}",
        Paint::red("Hello"),
        Paint::green("PantySmeller"),
        Paint::blue("!")
    );
}

// Simple banner - Note using cfonts
pub fn ban1() {
    let string = "PantySmeller!";

    // Function strucutre is as per the official documentation
    say(Options {
        text: String::from(string),
        font: Fonts::FontChrome,
        gradient: vec![String::from("#ff8800"), String::from("#88ff00")],
        ..Options::default()
    });
}
