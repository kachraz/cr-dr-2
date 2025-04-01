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

use std::{fmt::format, result, vec};

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
    enum_5_2()
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
        JusRegions::East => println!("{}", "East region is supported ✅".yellow()),
        _ => println!("{:?} : Region Not Supported ❌", j.red()),
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

    divy("⚛");
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

    divy("⚛");
    enums_2_supported_regions(jus1.region);
    enums_2_supported_regions(JusRegions::East);
}

//////// Option Enum Testing ///////

fn enum3_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 { None } else { Some(x / y) }
}

fn enum_3() {
    let title = "Option Enum";
    header(title);

    let a = 3000;
    let b = 10;

    let rez = enum3_div(a, b);

    // Using the enum
    divy("⚛");
    println!("{} / {} = ", a, b);
    match rez {
        Some(v) => println!("Result: {}", v.green()),
        None => println!("Division by zero!"),
    }
}

// enum3 function but with float values

fn enum4_div(x: f32, y: f32) -> Option<f32> {
    if y == 0.0 { None } else { Some(x / y) }
}

fn enum_4() {
    let title = "Option Enum";
    header(title);

    let a: f32 = 3000.22;
    let b: f32 = 100.1234;

    let rez = enum4_div(a, b);

    // Using the enum
    divy("⚛");
    println!("{} / {} = ", a, b);
    match rez {
        Some(v) => println!("Result: {}", v.green()),
        None => println!("Division by zero!"),
    }
}

///////////////////////////////////////////////////////////////
// Applied Enums -

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

fn enum5_format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        _ => FileSize::Gigabytes(size / 1_000_000_000),
    };

    // Formatting the above outputut
    match filesize {
        FileSize::Bytes(b) => format!("{} Bytes", b),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
    }
}

fn enum_5() {
    let title = "Enums - File Size";
    header(title);

    let result = enum5_format_size(854515558745688);
    println!("File size: {}", result.green());
}

// Second Version

enum FileSize2 {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

// impl - Implementation block of the enum FileSize2
impl FileSize2 {
    fn format_size(&self) -> String {
        match self {
            FileSize2::Bytes(bytes) => format!("{} Bytes", bytes),
            FileSize2::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
            FileSize2::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
            FileSize2::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
        }
    }
}

fn enum_5_2() {
    let title = "Enums - File Size 2";
    header(title);

    let size = 2_000_000_000;
    let filesize = match size {
        0..=999 => FileSize2::Bytes(size),
        1000..=999_999 => FileSize2::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize2::Megabytes(size / 1_000_000),
        _ => FileSize2::Gigabytes(size / 1_000_000_000),
    };

    println!("File size 2: {}", filesize.format_size().green());
}
