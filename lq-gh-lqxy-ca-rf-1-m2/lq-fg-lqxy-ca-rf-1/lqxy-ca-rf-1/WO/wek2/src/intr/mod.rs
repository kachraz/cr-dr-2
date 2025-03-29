// Introduction to Rust Chapter

use crate::utils::{header, print_with_synthwave_gradient};
use yansi::Paint;

pub fn intr_main() {
    print_with_synthwave_gradient("Introduction to Rust".to_string());
    demo_variable_assignment();
    ctrl_flow_1();
    mutability_1()
}

fn demo_variable_assignment() {
    // Vaible Assigment
    let message = "Panty Smellers";
    let fetish_number = 69.2;
    let booty_weight = fetish_number / 2.2;

    header("Variable Assignment");

    println!(
        "{} {} {}",
        message.bold(),
        fetish_number.magenta(),
        booty_weight.yellow(),
    );
}

// learning about control flow here
fn ctrl_flow_1() {
    header("Control Flow");
    let proceed = true;
    if proceed {
        println!("{}", "Sniffing".green());
    } else {
        println!("Not Proceeding");
    }

    // If Else blocks
    let height = 60;
    if height < 69 {
        println!("{}", "Short 69".green());
    } else if height == 69 {
        println!("{}", "Straight 69".blue());
    } else {
        println!("{}", "More 69".red());
    }
}

// mutability
fn mutability_1() {
    header("Mutability Test");
    let mut height = 100;
    height = height * 20;

    let result = if height > 200 {
        println!("{}", "Height is > 200".green())
    } else if height > 300 {
        println!("{}", "Height is > 300".red())
    } else {
        println!("{}", "Fuck".magenta())
    };

    println!("Result : {:?}", result) // This works since the color cant be printed
}
