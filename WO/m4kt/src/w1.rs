//
// Structs and Impl
//

#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::ut::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

//////////////////////////////////////////
/// Main Entry Function
//////////////////////////////////////////

pub fn w1_main() {
    clear_console();
    let main_header = "Moodules".to_string();
    print_with_synthwave_gradient(main_header);

    // Call the functions
}
