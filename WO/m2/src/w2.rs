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
    str_1("Hello, world!");
    str_2("Hello, world!".to_string());
}

//////////////////////////////////////////
/// Functions Areas
//////////////////////////////////////////

fn str_1(s: &str) {
    // Print the string
    println!("String: {}", s.green());
}

fn str_2(s: String) {
    // Print the string
    println!("String: {}", s.green());
}
