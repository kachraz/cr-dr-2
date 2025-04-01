/*
╦  ╦ ╔═╗ ╔═╗ ╔╦╗ ╔═╗ ╦═╗ ╔═╗      ╔═╗ ╔╗╔ ╔╦╗      ╔═╗ ╔═╗ ╔╦╗ ╔═╗
 ╚╗╔╝ ║╣  ║    ║  ║ ║ ╠╦╝ ╚═╗      ╠═╣ ║║║  ║║      ║ ║ ╠═╝  ║  ╚═╗
  ╚╝  ╚═╝ ╚═╝  ╩  ╚═╝ ╩╚═ ╚═╝      ╩ ╩ ╝╚╝ ═╩╝      ╚═╝ ╩    ╩  ╚═╝
*/

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
    let title = "Vector and Ops".to_string();
    print_with_synthwave_gradient(title);

    // Call the functions
    vecs_3();
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
    // Looks like int but is a usize
    let vec = vec![10, 20, 30, 40, 50];

    // Retrieve value at specific index
    divy("o");
    let value = vec.get(index).unwrap(); // Indexing starts at 0
    println!(
        "Value at index {}: {}",
        index,
        value.to_string().green().italic()
    );
}

fn vecs2_get_item_fn() {
    let vec = vec![10, 20, 30, 40, 50];
    vecs2_get_item(3);
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

// Adding and removing elements from a vector

fn vecs_3() {
    let hea1 = "Adding and Removing Elements from a Vector";
    header(hea1);

    let mut v = vec![1, 2];

    // Adding elements to the vector
    divy("o");
    v.push(99);
    println!("After push: {:?}", v.yellow().italic());

    // Extending the vector
    divy("o");
    let more_numz = vec![3, 4, 5];
    v.extend(more_numz);
    println!("After extend: {:?}", v.yellow().italic());

    // Appends - adds given vector , requires vegctor to be mutable
    divy("o");
    let mut v2 = vec![22, 23, 32];
    v.append(&mut v2);
    println!("After append: {:?}", v.yellow().italic());

    // Inserting an element at a specific index
    divy("o");
    v.insert(2, 100);
    println!("After insert: {:?}", v.yellow().italic());

    // Removing an element from the vector
    divy("o");
    let removed_value = v.remove(2);
    println!(
        "After remove: {:?} (removed value: {})",
        v.yellow().italic(),
        removed_value.to_string().green().italic()
    );
    // Removing the last element
    divy("o");
    let last_value = v.pop();
    println!(
        "After pop: {:?} (removed value: {})",
        v.yellow().italic(),
        last_value.unwrap_or(0).to_string().green().italic()
    );

    // Checking if the vector is empty
    divy("o");
    if v.is_empty() {
        println!("Vector is empty");
    } else {
        println!("Vector is not empty");
    }

    // Checking the length of the vector
    divy("o");
    let length = v.len();
    println!("Length of vector: {}", length.to_string().green().italic());
}
