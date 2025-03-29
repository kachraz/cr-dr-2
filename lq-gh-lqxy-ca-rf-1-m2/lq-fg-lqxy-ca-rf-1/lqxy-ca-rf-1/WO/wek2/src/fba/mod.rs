// Loope and control flow
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(deprecated)]

use crate::utils::{clear_console, header, print_with_synthwave_gradient};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
use yansi::Paint;

// Main function that will call other functions in the file
pub fn fba_main() {
    // clear_console();
    print_with_synthwave_gradient("Function Basics".to_string());
    er_ha_ma_2();
}

// ******************
// Various number operations
// ******************

// Variou operations on numbers
fn proc_num(numbers: &[i32]) {
    header("Various number operations");

    // Init sum to 0
    let mut sum = 0;

    // Iterate over the numbers, adding each one to sum
    for num in numbers {
        sum += num;
    }

    // Print sum of numbers
    println!("Sum of numbers: {}", sum.bright_blue());

    // If sum even print message
    if sum % 2 == 0 {
        println!("Sum is even");
    } else {
        println!("Sum is odd");
    }
}

fn proc_num_call() {
    // Create a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers2 = [100, 200, 300, 400, 500];

    // Call proc_num with the vector
    proc_num(&[100, 200, 300, 400, 500]);
    println!("Numbers: {:?}", numbers2.green());
}

// Explicit return tyoes

fn split_strings(s: String, delimiter: char, field: usize) -> String {
    header("Split Strings");
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);

    result.expect("Fuck").to_string()
}
fn split_strings_call() {
    let chunk = split_strings("Smell, Panty".to_string(), ',', 1);
    println!("Chunk: {}", chunk.magenta());
}

// Using parameters
fn sum_arg(n: &[i32]) -> i32 {
    let text = "Sum funtion with Args";
    header(text);

    let mut result = 0;
    for number in n {
        result += number
    }
    result
}

fn sum_arg_call() {
    let num = [20, 5, 6, 7, 8];
    let res = sum_arg(&num);
    println!("Numbz = {:?}", num.red());
    println!("Sum iz {}", res.green())
}

// ******************
// Understanding Panic KW in Rust
// ******************

fn loop_panic_1(num: Vec<i32>) {
    header("Loops and Panic");
    for i in num {
        if i < 0 {
            panic!("FUCK NEGATIVE NUMBAZ");
        }

        println!("i: {}", i.green());
    }
}

// Call above function
fn loop_panic_1_call() {
    let new_string = String::from("hi Hi");
    // println!("{}", new_string[100]); // Going out of index
    loop_panic_1(vec![6, 9, -9, 0]);
}

// ******************
// Error Handling with March
// ******************

// Trying to open a file which doesnt exist to handle error
fn er_ha_ma_1() {
    header("Read File and Panic");
    let file = File::open("smellpanty.txt");

    // Checking existence of the file
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("[x] NO FILE: {}", error)
            }
            _ => {
                panic!("Cant Open File: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap().green())
    }
}

// Diff from above fuction , the errors are not depended on the variable type
// - Above it has to be file type and println! wont work
// - Function below we are depending on the file type

// Above function refactor
fn er_ha_ma_2() {
    header("Search for file , read or panic");
    let file = File::open("smellpanty.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                print!("{}", line.unwrap().green())
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("NO FILE : {}", error)
            }
            _ => {
                panic!("cant open bastard {}", error)
            }
        },
    }
}
