pub mod countdown {
    use std::time::Duration;
    use std::io::{self, Write};
    use std::thread::sleep;

    pub fn countdown() {
        println!("hello world from countdown");
    }

    /// Countdown timer
    ///
    /// This function takes a number of seconds to count down from and a callback function to call.
    /// This is a blocking call.
    pub fn callback_countdown(seconds: i64, callback: fn(seconds: i64) -> ()) {
        for remaining in (0..=seconds).rev() {
            callback(remaining);
            sleep(Duration::from_secs(1));
        }
    }

    /// Countdown timer
    ///
    /// This function takes a number of seconds to count down from and prints the countdown.
    /// This is a blocking call.
    ///
    /// # Deprecated
    ///
    /// This function is deprecated. Use [`callback_countdown`] instead.
    #[deprecated(since = "0.1.1", note = "Use callback_countdown instead")]
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
