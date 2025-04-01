//
// Exploring Strings and vectors in Rust
//

#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fmt::format;

use crate::ut::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

//////////////////////////////////////////
/// Main Entry Function
//////////////////////////////////////////

pub fn w2_main() {
    clear_console();
    print_with_synthwave_gradient("Strings".to_string());

    // Call the functions
    strman_4();
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

    // Note - this is how to add to a existing string object
    new_string.push_str(" is a string slice 🦧🦧🦧🦧");
    println!("String Slice: {}", new_string.green());
}

// formatting string
fn print_formatted_string(s: &str) {
    // format! = This returns a string slice that can be stored in a variable
    let new_string = format!("{} is a string slice", s.magenta());
    println!("Formatted String: {}", new_string.green());
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

    print_formatted_string(s);
}

//////////////////////////////////////////////
/// String Maniuplation
//////////////////////////////////////////////

// Making String Slices
fn strman_1() {
    header("String Manipulation - 1");

    let sentence = "Dancing is good for the soul";
    println!("Sentence Slice: {}", &sentence[0..=3].green());
}

// Concatenation with formatting
fn strman_2() {
    header("String Manipulation - 2");

    let sent1 = "Dancing is good for the soul";
    let description = format!(
        "Title: Quick Start Guide\n{}",
        sent1.yellow().bold().italic()
    );

    println!("Description: {}", description.green());
}

// Iteration over characters
fn strman_3() {
    header("String Manipulation\nIteration over Characters");

    let divy = "~_~".repeat(10);

    let sentence = "Vi Dango";

    // Printing the color text on match
    println!("{}\n{}", "ColorText", divy.blue());
    for c in sentence.chars() {
        match c {
            'a' => print!("{}", c.red()),
            'e' => print!("{}", c.blue()),
            'i' => print!("{}", c.green()),
            'o' => print!("{}", c.yellow()),
            'u' => print!("{}", c.cyan()),
            _ => print!("{}", c),
        }
    }

    // Matching a certain conditoon
    println!("{}\n{}", "Matcha and print", divy.blue());
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("{} - Vowel", c.red()),
            _ => continue,
        }
    }
}

// Splitting and collecting in a vector
fn strman_4() {
    header("String Manipulation - Split by words");

    let sentence = "Splitting a string into words";
}
