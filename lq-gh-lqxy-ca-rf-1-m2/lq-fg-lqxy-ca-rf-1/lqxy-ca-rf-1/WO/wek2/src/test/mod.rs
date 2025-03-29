// Testing printing here

use crate::utils::print_with_fire_gradient;
use yansi::Paint;

// This function is  the main runner for the file being called in main
pub fn test_main_runner() {
    mainban();
    tpr1();
    tpr2();
}

pub fn mainban() {
    // String to be printed
    let string = "src/test".to_string();
    // Print the string with fire gradient
    print_with_fire_gradient(string);
}

pub fn tpr1() {
    println!(
        "{}",
        Paint::blue("Hello, from src/test/mod.rs - tpr1- Smell Panty")
    );
}

pub fn tpr2() {
    println!("{}", Paint::yellow("Hello, from src/test/mod.rs - tpr1"));
}
