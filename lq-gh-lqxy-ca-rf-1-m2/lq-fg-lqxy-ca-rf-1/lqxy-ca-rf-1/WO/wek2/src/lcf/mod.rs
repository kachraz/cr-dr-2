// Loope and control flow
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(deprecated)]

use crate::utils::{clear_console, header, print_with_synthwave_gradient};
use std::io;
use yansi::Paint;

// Main function that will call other functions in the file
pub fn lcf_main() {
    clear_console();
    print_with_synthwave_gradient("LCF".to_string());
    match_break_1();
}

// Loops function

fn loops_1() {
    header("Loop Test");
    let mut x = 1;
    // continue loop until x > 5
    loop {
        println!("{}, {}", "x Looped to".blue(), x.magenta());
        x += 1;
        if x > 5 {
            break;
        }
    }
}

// Enahnced Loops function that will accept input and then loop
fn loops_2() {
    header("Enhanced Loop Test");

    let mut input = String::new();
    println!("➡️ Fucker Put Number: ");
    io::stdin().read_line(&mut input).unwrap();

    // Handle invalid or empty input
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Invalid input. Please enter a valid number.".red());
            return;
        }
    };

    let mut x = 1; // Start the loop counter
    loop {
        println!("{}, {}", "x Looped to".blue(), x.magenta());
        x += 1;
        if x > input {
            break;
        }
    }
}

// Exploring if let
fn if_let_1() {
    header("If Let Test");
    // let mut maybe_number = some(69)
    let may_num = Some(69);
    if let Some(num) = may_num {
        println!("Number: {}", num.magenta());
    } else {
        println!("No number");
    }
}

// While Loops function

fn while_loops_1() {
    header("While Loop Test");
    let mut x = 1;
    while x <= 5 {
        println!("{}{}", "x Looped = ".blue(), x.magenta());
        x += 1;
    }
}

// Advanced While Loops function with input
fn while_loop_advanced_1() {
    let mut input = String::new();
    while input.trim() != "Stop" {
        input.clear();
        println!("Enter 'Stop' to stop the loop: ");
        io::stdin().read_line(&mut input).expect("Failed");
        println!("You entered: {}", input.trim().blue());
    }
}

// WOrks , the issue was the input.clear was not being cleared
fn while_loop_advanced_2() {
    let mut input = String::new();
    while input.trim() != "Stop" {
        // Explicitly call String's clear() to avoid yansi conflict
        String::clear(&mut input); // <-- Fix here

        println!("Enter 'Stop' to stop the loop:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("You entered: {}", input.trim().blue());
    }
}

//==================================================================================================
// For Loop Training
//==================================================================================================

fn for_loop_1() {
    header("For Loop Test");
    let line = "~".repeat(20);

    println!("{}\n {}", "For 1", line.blue());
    for i in 1..=6 {
        println!("{}{}", "i Looped = ".blue(), i.magenta());
    }

    // printing in reverse - Not the rev keyword
    println!("{}\n{}", "For 2 - In Reverse", line.blue());
    for i in (1..6).rev() {
        println!("{}{}", "i Looped = ".blue(), i.magenta());
    }

    println!("{}\n{}", "For 3 - Looping throw array ", line.blue());
    let numbers = vec![10, 20, 30, 40, 50];
    for n in numbers {
        println!("{}{}", "n Looped = ".green(), n.magenta());
    }
}

//==================================================================================================
// match and break statements
// Aaplies to while loops ,
//==================================================================================================

fn for_mb_1() {
    header("Match and Break Test");

    for i in 1..=10 {
        if i % 2 == 0 {
            println!("{}{}", "i Skipped = ".blue(), i.magenta());
            continue;
        }
        println!("i = {}", i.green());
        if i == 7 {
            break;
        }
    }
}

//==================================================================================================
// match control flow patterm NMNo
// Aaplies to while loops ,
//==================================================================================================

fn match_break_1() {
    header("Match and Break Test");

    println!("Enter Fetish:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Raped");
    println!("You entered: {}", input.trim().blue());

    // Writing the match case statement
    match input.trim() {
        "Feet" | "sweat" => println!("{}", "Foot Fetish".red()),
        "Fart" => println!("{}", "Fart Sniiffing".green()),
        _ => println!("{}", "Scat Fetish".blue()),
    }
}
