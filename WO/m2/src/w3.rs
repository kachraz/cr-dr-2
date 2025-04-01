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
    vecs_2();
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

fn vecs_2() {
    let hea1 = "Retreiving Elements from a Vector";
    header(hea1);

    let vec = vec![1, 2, 3, 4, 5];

    //
}
