//Work to study ownership and borrowing model
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// --- Imports Section ---
use crate::{
    print,
    utils::{clear_console, header, print_with_synthwave_gradient},
};
use yansi::Paint;

// Main Function call
pub fn w3_main() {
    // Banner Section
    let banner = "Ownership Model ";
    print_with_synthwave_gradient(banner.to_string());
    header("w3.rs - 3rd Version of the file");

    // Calling Sub Functions
    happy_lor();
}

// Sub Functions
#[derive(Debug)]
pub struct MyPreciousRing {
    pub engraving: String,
}

impl MyPreciousRing {
    pub fn forge() -> Self {
        Self {
            engraving: "The One Ring".to_string(),
        }
    }
}

mod we_are_all_frens_here {
    #[derive(Debug, Copy, Clone)]
    pub struct MyPreciousRing {
        engraving: &'static str,
    }

    impl MyPreciousRing {
        pub fn forge() -> Self {
            Self {
                engraving: "The One Ring",
            }
        }
    }
}

#[inline(never)] // This instruction forces the function to be seperately compiled
fn happy_lor() {
    let saurons_ring = we_are_all_frens_here::MyPreciousRing::forge();
    println!("Saurons Ring Says: {saurons_ring:?}");

    // Copy data bitwise since
    let gollums_ring = saurons_ring;
    println!("My Precious Ring Says: {gollums_ring:?}");

    let bilbos_ring = gollums_ring;
    println!("Bilbos Ring Says: {bilbos_ring:?}");

    let frodo_ring = bilbos_ring;
    println!("Frodo's Ring Says: {frodo_ring:?}");
}
