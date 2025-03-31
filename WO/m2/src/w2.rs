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
}

//////////////////////////////////////////
/// Functions Areas
//////////////////////////////////////////

fn str_1() {
    header("Strings Function 1");
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = format!("{} {}", s1, s2);
    println!("{}", s3);
}
