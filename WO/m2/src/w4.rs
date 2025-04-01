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
    enums_2();
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
    println!("Disk size is {:?}", disk_size.blue());
}

/////////// Enm2 /////////

// Main Enum
#[derive(Debug)]
enum JusRegions {
    North,
    South,
    East,
    West,
}

// Using enum as a type in this struct
struct Juse {
    name: String,
    region: JusRegions, // uice regions used as a type
}

// Running the pattern matching on the enum
fn enums_2_supported_regions(j: JusRegions) {
    // Pattern matching one condition - you can add multiple here
    match j {
        JusRegions::East => println!("{}", "East region is supported".yellow()),
        _ => println!("{}", j, "Other regions are not supported".red()),
    }
}

fn enums_2() {
    let title = "Jus Regions";
    header(title);

    let jus1 = Juse {
        name: String::from("North Juice"),
        region: JusRegions::North,
    };

    let jus2 = Juse {
        name: String::from("South Juice"),
        region: JusRegions::South,
    };

    println!(
        "Jus1 : {} from {:?}",
        jus1.name.yellow(),
        jus1.region.yellow()
    );
    println!(
        "Jus2 : {} from {:?}",
        jus2.name.green(),
        jus2.region.green()
    );

    enums_2_supported_regions(jus1.region);
    enums_2_supported_regions(JusRegions::East);
}
