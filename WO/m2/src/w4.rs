/*
 ███████╗ ███╗   ██╗ ██╗   ██╗ ███╗   ███╗ ███████╗
 ██╔════╝ ████╗  ██║ ██║   ██║ ████╗ ████║ ██╔════╝
 █████╗   ██╔██╗ ██║ ██║   ██║ ██╔████╔██║ ███████╗
 ██╔══╝   ██║╚██╗██║ ██║   ██║ ██║╚██╔╝██║ ╚════██║
 ███████╗ ██║ ╚████║ ╚██████╔╝ ██║ ╚═╝ ██║ ███████║
 ╚══════╝ ╚═╝  ╚═══╝  ╚═════╝  ╚═╝     ╚═╝ ╚══════╝

 Working with enums
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

pub fn w4_main() {
    clear_console();
    let title = "Enums".to_string();
    print_with_synthwave_gradient(title);

    // Call the functions
    enums_1();
}

//////////////////////////////////////////
/// Sub Functions
//////////////////////////////////////////

// enum1

enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

fn enums_1() {
    let title = "Enums And Pattern Matching";
    header(title);

    let disk_type = DiskType::SSD;

    // Comparing disk types - Using match
    match disk_type {
        DiskType::SSD => println!("{}", "Disk type is SSD".green().blink()),
        DiskType::HDD => println!("{}", "Disk type is HDD".bright_yellow().bold().blink()),
    }

    divy("⚛");
    let disk_size = DiskSize::GB(500);
    println!("Disk size is {:?}", disk_size);
}
