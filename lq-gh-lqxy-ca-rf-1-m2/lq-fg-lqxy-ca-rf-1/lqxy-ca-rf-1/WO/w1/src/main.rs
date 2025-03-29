use yansi::Paint;

fn main() {
    println!("Hello, world!");
    println!("Hello, world!, what mean this ");
    test();
}

// This function is used to test the yansi crate
fn test() {
    println!(
        "Testing, {}, {}, {}, {}!",
        "Ready".bold(),
        "Set".black().on_yellow().invert().italic(),
        "STOP".white().on_red().bright().underline().bold(),
        "Multi Panty Smellis".blue().on_green().bold()
    );
}
