// Module focussed on borrows and lifetimes
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// --- Imports Section ---
use crate::w2::{heat, MyPreciousRing};
use crate::{
    print,
    utils::{clear_console, header, print_with_synthwave_gradient},
};
use std::{cell::RefCell, rc::Rc};
use yansi::Paint;

// Main Function call
pub fn w4_main() {
    // Banner Section
    let banner = "Ownership Model ";
    print_with_synthwave_gradient(banner.to_string());
    header("w4.rs - 4th Version of the file");

    // Calling Sub Functions
    // share_the_ring();
    share_and_alter();
}

fn share_the_ring() {
    let saurons_ring = MyPreciousRing::forge();
    println!("Saurons Ring Says: {saurons_ring:?}");

    // Somehow the ring get to frodo - RC Ref Counter - reference the same object with the same memeory location - UnSafe Programming
    let frodos_ring = Rc::new(saurons_ring);
    println!("The Ring Now Has {} Owners", Rc::strong_count(&frodos_ring));
    // println!("have to destroy it... ({:?})", frodos_ring.engraving);
    println!("Frodo's Ring Says: {frodos_ring:?}");

    // Cloning the ring - Clone Ref Counter - reference the same object with the same memeory location - Safe Programming
    let samwises_ring = Clone::clone(&frodos_ring);
    println!("Samwise's Ring Says: {samwises_ring:?}");

    println!("The Ring Now Has {} Owners", Rc::strong_count(&frodos_ring));
    println!(
        "\t{:p}\n\t{:p}",
        frodos_ring.as_ref(),
        samwises_ring.as_ref()
    );

    // let mut frodos_ring = frodos_ring;
    // heat(&mut frodos_ring);
}

// New method
fn share_and_alter() {
    let saurons_ring = MyPreciousRing::forge();
    // Somehow , the ring gets to Frodo...

    let frodos_ring = Rc::new(RefCell::new(saurons_ring));
    println!("Have to destroy it... ({:?})", frodos_ring.borrow());

    let samwises_ring = Clone::clone(&frodos_ring);
    println!("Ring now has {} owners", Rc::strong_count(&frodos_ring));
    println!(
        "\t{:p}\n\t{:p}",
        frodos_ring.as_ref(),
        samwises_ring.as_ref()
    );

    heat(&mut frodos_ring.borrow_mut());
    println!("The ring says :{:?}", samwises_ring.borrow());

    let mut frodos_ring_mut = frodos_ring.borrow_mut();
    let mut samwise_ring_mut = samwises_ring.borrow_mut();
    heat(&mut frodos_ring_mut);
    heat(&mut samwise_ring_mut);
}
