use chrono::{Duration, Local};
use clap;
use countdown::countdown::print_countdown;

fn main() {
    let matches = clap::Command::new("Add Seconds")
        .version("1.0")
        .author("Ricky")
        .about("Add seconds")
        .arg(
            clap::Arg::new("number")
                .short('n')
                .long("number")
                .value_name("NUMBER")
                .help("set the multiplier of seconds")
                .required(true)
                .value_parser(clap::value_parser!(i64)),
        )
        .get_matches();

    // default value var1
    let var1 = 10;

    // get the value of var2 from command line arg
    let var2: i64 = *matches.get_one("number").expect("Invalid number");

    let total_seconds = var1 * var2;

    let current_date = Local::now();

    let new_date = current_date + Duration::seconds(total_seconds);

    // Display the new date and time
    println!("Current date and time: {}", current_date);
    println!("Date and time after adding {} seconds: {}", total_seconds, new_date);

    // countdown timer
    print_countdown(total_seconds);
}
