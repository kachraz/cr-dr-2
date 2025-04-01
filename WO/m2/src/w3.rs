//
// Vector Operations
//

#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{fmt::format, vec};

use crate::ut::{clear_console, divy, header, print_with_synthwave_gradient};
use yansi::Paint;

//////////////////////////////////////////
/// Main Entry Function
//////////////////////////////////////////

pub fn w3_main() {
    clear_console();
    let title = "Vector and Ops".to_string();
    print_with_synthwave_gradient(title);

    // Call the functions
    // vecs_2();
    vecs2_get_item_fn();
}

//////////////////////////////////////////
/// Vector Related Ops
//////////////////////////////////////////

fn vecs1_ownership() {
    let numz = vec![1, 2, 3, 4, 5];
    let slicez = &numz[..]; // Borrowing a slice of the vector
    println!("Slicez: {:?}", slicez.green().italic());
}

fn vecs1_modifiable() {
    let mut numz = vec![1, 2, 3, 4, 5];
    let slicez = &mut numz[..]; // Mutable slice
    slicez[0] = 91230; // Modifying the first element
    println!("Modified Slicez: {:?}", slicez.yellow().italic());
}

fn vecs_1() {
    header("Vector Operations");

    // Slices and vectros are similar. But slices are immutable depending on how they are borrowed

    // vecs1_ownership();
    vecs1_modifiable();
}

// Retrieving elements from a vector

fn vecs2_get_item(index: usize) {
    let index = 3; // Looks like int but is a usize
    let vec = vec![10, 20, 30, 40, 50];

    // Retrieve value at specific index
    divy("o");
    let value = vec[index]; // Indexing starts at 0
    println!(
        "Value at index {}: {}",
        index,
        value.to_string().green().italic()
    );
}

fn vecs2_get_item_fn() {
    let vec = vec![10, 20, 30, 40, 50];

    // Get input to find the index
    println!("Enter desired index: ");
    let mut input = String::new();

    let fail_msg = "Invalid input. Please enter a valid index."
        .red()
        .to_string();

    std::io::stdin().read_line(&mut input).expect(&fail_msg);
    vecs2_get_item(2);
}

fn vecs_2() {
    let hea1 = "Retreiving Elements from a Vector";
    header(hea1);

    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve value at specific index
    divy("o");
    let third_value = vec[2]; // Indexing starts at 0
    println!("Third value: {}", third_value.to_string().green().italic());

    // Retrieve last value
    divy("o");
    let last_value = vec.last().unwrap();
    println!("Last value: {}", last_value.to_string().green().italic());

    // Retrieve first value
    divy("o");
    let first_value = vec[0]; // First value
    println!("First value: {}", first_value.to_string().green().italic());

    // Retrieve first value using pattern matching
    divy("o");
    match vec.first() {
        Some(first) => println!(
            "First value using pattern matching: {}",
            first.to_string().green().italic()
        ),
        None => println!("Vector is empty"),
    }
}
