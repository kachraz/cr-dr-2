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

fn print_str_2(s: &str) {
    let mut new_string = s.to_string();
    new_string.push_str(" is a string slice");
    println!("String Slice: {}", new_string.green());
}

// String type
fn print_string(s: String) {
    // Print the string
    println!("String Type: {}", s.green());
}

fn str_3() {
    header("String Manipulation");

    let s = "Love is love";
    print_str(s);

    // String is frowable and mutable where as str is not
    // String is owned by the code that creates it

    let salutation = String::from("Saluations");
    print_string(salutation);

    print_str_2(s);
}
