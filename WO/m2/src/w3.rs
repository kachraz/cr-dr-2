//
// Vector Operations
//

#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{fmt::format, vec};

use crate::ut::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

//////////////////////////////////////////
/// Main Entry Function
//////////////////////////////////////////

pub fn w3_main() {
    clear_console();
    let title = "Vector and Ops".to_string();
    print_with_synthwave_gradient(title);

    // Call the functions
    vecs_1();
}

//////////////////////////////////////////
/// Vector Related Ops
//////////////////////////////////////////

fn vecs1_ownership() {
    let numz = vec![1, 2, 3, 4, 5];
    let slice = &numz[..]; // Borrowing a slice of the vector
    println!("Slice: {:?}", slice.green().italic());
}

fn vecs1_modifiable() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let slice = &mut numbers[..]; // Mutable slice
    slice[0] = 10; // Modifying the first element
    println!("Modified Slice: {:?}", slice.yellow().italic());
}

fn vecs_1() {
    header("Vector Operations");

    // Slices and vectros are similar. But slices are immutable depending on how they are borrowed

    vecs1_ownership();
    // vecs1_modifiable();
}
