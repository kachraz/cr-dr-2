// work one
#![allow(unused)]
#![allow(dead_code)]

use crate::ut::{clear_console, header, print_with_synthwave_gradient};

//////////////////////////////////////////
/// Main Entry Function
//////////////////////////////////////////

pub fn w1_main() {
    clear_console();
    print_with_synthwave_gradient("booty".to_string());
    header("W3");
    println!("W1 is a module that does something interesting.");
    println!("It is currently under bootysmells.");
}

//////////////////////////////////////////
/// Functions Areas  
//////////////////////////////////////////

/// Struct function

struct Person {
    first_name: String,
    last_name: String,
    age: u32,
}
