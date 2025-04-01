//
// Exploring Strings and vectors in Rust
//

#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::ut::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

//////////////////////////////////////////
/// Main Entry Function
//////////////////////////////////////////

pub fn w2_main() {
    clear_console();
    print_with_synthwave_gradient("Strings".to_string());

    // Call the functions
    str_3();
}

//////////////////////////////////////////
/// Functions Areas
//////////////////////////////////////////

// String slices
fn print_str(s: &str) {
    // Print the string
    println!("String Slice: {}", s.green());
}

// String type
fn print_string(s: String) {
    // Print the string
    println!("String: {}", s.green());
}

fn str_3() {
    header("String Manipulation");

    let s = "Love is love";
    print_str(s);
}
