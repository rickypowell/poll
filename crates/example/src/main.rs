use countdown::countdown::callback_countdown;
use std::io::{self, Write};

fn main() {
    // countdown timer
    println!("Countdown timer: ");
    callback_countdown(12, |seconds| {
        let erase_current_line = "\x1b[2K";
        let warning = match seconds {
            // yellow
            4..=10 => "\x1b[33m",
            // red
            0..=3 => "\x1b[31m",
            // blue
            _ => "\x1b[34m",
        };
        print!(
            "\r{}{}{} seconds remaining\x1b[0m",
            erase_current_line, warning, seconds
        );
        io::stdout().flush().unwrap();
    });
    println!("\n\x1b[1mTime's up!\x1b[0m");
}
