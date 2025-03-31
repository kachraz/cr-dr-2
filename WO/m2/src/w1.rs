// work one
#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::ut::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

//////////////////////////////////////////
/// Main Entry Function
//////////////////////////////////////////

pub fn w1_main() {
    clear_console();
    print_with_synthwave_gradient("booty".to_string());

    // Call the functions
    str_2();
}

//////////////////////////////////////////
/// Functions Areas  
//////////////////////////////////////////

/// Struct function

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

/*
In the code below note the {:#?} format specifier.
This is used to print the struct in a pretty format.
*/
fn str_1() {
    header("Structs Function 1");
    println!(
        "{:#?}",
        Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            age: 30,
        }
    );
}

/*
Creating Struct Instances
*/

fn str_2() {
    header("Structs Function 2 - Creating instances");

    let nina = Person {
        first_name: "Nina".to_string(),
        last_name: "Simone".to_string(),
        age: 20,
    };

    // Printing the whole struct
    println!("{:#?}", nina);

    // Field Access
    println!("First Name: {}", nina.first_name.green());
}
