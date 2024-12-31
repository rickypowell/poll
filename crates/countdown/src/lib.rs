pub mod countdown {
    use std::time::Duration;
    use std::io::{self, Write};
    use std::thread::sleep;

    pub fn countdown() {
        println!("hello world from countdown");
    }

    pub fn print_countdown(seconds: i64) {
        // countdown timer
        println!("Countdown timer: ");
        let escape = "\x1b";
        let erase_current_line = "[2K";
        for remaining in (0..=seconds).rev() {
            print!("\r{}{}{} seconds remaining", escape, erase_current_line, remaining);
            io::stdout().flush().unwrap();
            sleep(Duration::from_secs(1));
        }

        println!("\nTime's up!");
    }
}
