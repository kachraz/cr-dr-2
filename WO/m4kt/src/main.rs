// main entry point

mod ut;
mod w1;
use m4lib::read_func;
use w1::w1_main;

fn main() {
    w1_main();
    read_func();
}
