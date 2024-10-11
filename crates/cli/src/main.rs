use chrono::{Duration as ChronoDuration, Local};
use clap::Parser;
use command::run_command;
use pretty_print::{clear, move_cursor_top_left, println_group, print_bold, GroupItem};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
struct Args {
    /// limit on how many checks will take place
    #[arg(short, long, default_value_t = 500)]
    limit: u64,

    /// amount of time until the next check is performed
    #[arg(short, long, default_value_t = 10)]
    interval: u64,

    #[arg(short, long, default_value_t = String::from("exit 0"))]
    command: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let start = Local::now();

    for i in 0..args.limit {
        // Clear the terminal screen
        clear()?;
        // Move cursor to top of terminal
        move_cursor_top_left()?;

        let now = Local::now();
        let iter_left = args.interval - i;
        let end = start + ChronoDuration::seconds(args.limit as i64 * iter_left as i64);
        let datetime_fmt = "%b %d, %H:%M:%S %Z";
        println_group(
            "Configuration",
            vec![
                GroupItem::new(
                    "polling_limit",
                    args.limit.to_string(),
                    Some(500.to_string()),
                ),
                GroupItem::new(
                    "interval",
                    args.interval.to_string(),
                    Some(10.to_string()),
                ),
                GroupItem::new(
                    "command",
                    args.command.to_string(),
                    Some(String::from("exit 0")),
                ),
                GroupItem::new("start", now.format(datetime_fmt).to_string(), None),
                GroupItem::new("end", end.format(datetime_fmt).to_string(), None),
            ],
        )?;

        // print the number left after this iteration
        print_bold(args.limit - 1 - i)?;
        print!(" checks of ");
        print_bold(args.limit)?;
        println!(" left");

        match run_command(&args.command) {
            Ok(output) => {
                io::stdout()
                    .write_all(&output.stdout)
                    .expect("some output should be here");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        // don't sleep on the last iteration
        if i <= args.limit {
            sleep(Duration::from_secs(args.interval));
        }
    }

    Ok(())
}
