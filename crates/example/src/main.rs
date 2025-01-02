use countdown;
use std::io::{self, Write};

fn main() {
    // countdown timer
    println!("Countdown timer: ");
    countdown::countdown::callback_countdown(20, |seconds| {
        let escape = "\x1b";
        let erase_current_line = "[2K";
        print!(
            "\r{}{}{} seconds remaining",
            escape, erase_current_line, seconds
        );
        io::stdout().flush().unwrap();
    });
    println!("\nTime's up!");
    println!("Hello, world!");
}
