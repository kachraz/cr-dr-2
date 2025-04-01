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

use crate::ut::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

//////////////////////////////////////////
/// Main Entry Function
//////////////////////////////////////////

pub fn w4_main() {
    clear_console();
    let title = "Enums".to_string();
    print_with_synthwave_gradient(title);

    // Call the functions
}

//////////////////////////////////////////
/// Sub Functions
//////////////////////////////////////////

enum Disktype {
    SSD,
    HDD,
}

#[derive(Debug)]
fn enums_1() {}
