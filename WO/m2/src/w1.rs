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
    print_with_synthwave_gradient("Structs".to_string());

    // Call the functions
    str_3();
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

#[derive(Debug)]
struct Person1 {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

fn str_2() {
    header("Structs Function 2 - Creating instances");

    // Must pass all the field values in the struc
    let nina = Person1 {
        first_name: "Nina".to_string(),
        last_name: "Simone".to_string(),
        age: Some(99),
    };

    // Printing the whole struct
    println!("{:#?}", nina);

    // Field Access
    // Note you have to unwrap the value since Some is an enum
    println!("First Name: {}", nina.first_name.green());
    println!("Age: {:#?}", nina.age.unwrap().blue());
}

// More Structs
#[derive(Debug)]
struct User1 {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

// Associated function doesnt require self

// Constructor Function
impl User1 {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn str_3() {
    header("Structs Function 3 - More Structs");

    let divy = "+".repeat(30);

    let mut new_user = User1::new(
        String::from("nina"),
        String::from("nina@mistress.com"),
        String::from("http://bootyfarts.com"),
    );

    println!("Username: {}", new_user.username.green());
    println!(
        "Account = {}, Active = {}",
        new_user.username.green(),
        new_user.active.green()
    );

    new_user.deactivate();
    println!(
        "Account = {}, Active = {}",
        new_user.username.red(),
        new_user.active.red()
    );
}
