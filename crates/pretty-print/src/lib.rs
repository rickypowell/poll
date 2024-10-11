use crossterm::{
    cursor::MoveTo,
    execute,
    style::{PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self};

/// prints the give `s`.
///
/// The style is reset so the bold does not continue in the stdout.
pub fn print_bold<Input: ToString>(s: Input) -> io::Result<()> {
    io::stdout().execute(PrintStyledContent(s.to_string().bold()))?;

    Ok(())
}

/// prints the give `s` followed by a new line.
///
/// The style is reset so the bold does not continue in the stdout.
pub fn println_bold(s: &str) -> io::Result<()> {
    let with_new_line = format!("{}{}", s, "\n");
    io::stdout().execute(PrintStyledContent(with_new_line.bold()))?;

    Ok(())
}

/// prints the give `s`.
///
/// The style is reset so the italic does not continue in the stdout.
pub fn print_italic(s: &str) -> io::Result<()> {
    io::stdout().execute(PrintStyledContent(s.italic()))?;

    Ok(())
}

/// prints the give `s` followed by a new line.
///
/// The style is reset so the italic does not continue in the stdout.
pub fn println_italic(s: &str) -> io::Result<()> {
    let with_new_line = format!("{}{}", s, "\n");
    io::stdout().execute(PrintStyledContent(with_new_line.italic()))?;

    Ok(())
}

/// prints the given `s` followed by a new line in red color.
///
/// The style is reset so the red color does not continue in the stdout.
pub fn println_red(s: &str) -> io::Result<()> {
    let with_new_line = format!("{}{}", s, "\n");
    io::stdout().execute(PrintStyledContent(with_new_line.red()))?;

    Ok(())
}

/// prints the given `s` in red color.
///
/// The style is reset so the red color does not continue in the stdout.
pub fn print_red(s: &str) -> io::Result<()> {
    let with_new_line = format!("{}", s);
    io::stdout().execute(PrintStyledContent(with_new_line.red()))?;

    Ok(())
}

fn println_config_line_default<V: PartialEq + ToString>(
    label: &str,
    value: V,
    default_value: V,
) -> std::io::Result<()> {
    print!("│ ");
    print!("{}: ", label);
    print_bold(&format!("{}", value.to_string()))?;
    if value == default_value {
        print_italic(" default")?;
    }
    println!();

    Ok(())
}

// Overloaded function with default value for `default_value` parameter
fn println_config_line<V: PartialEq + ToString>(label: &str, value: V) -> std::io::Result<()> {
    print!("│ ");
    print!("{}: ", label);
    print_bold(&format!("{}", value.to_string()))?;
    println!();

    Ok(())
}

fn println_group_start(label: &str) {
    println!("╭ {}", label);
    println!("│ ");
}

fn println_group_end() {
    println!("╰");
}

pub struct GroupItem<'a, Value> {
    label: &'a str,
    value: Value,
    default_value: Option<Value>,
}

impl<'a, Value: ToString + PartialEq> GroupItem<'a, Value> {
    pub fn new(label: &'a str, value: Value, default_value: Option<Value>) -> Self {
        GroupItem {
            label,
            value,
            default_value,
        }
    }
}

pub fn println_group<T: ToString + PartialEq + Clone>(
    label: &str,
    items: Vec<GroupItem<T>>,
) -> std::io::Result<()> {
    if items.len() == 0 {
        return Ok(());
    }

    println_group_start(label);

    let max_label = items
        .iter()
        .max_by(|lhs, rhs| lhs.label.len().cmp(&rhs.label.len()))
        .expect("should have at least one");

    for item in &items {
        // build the padding for the label layout
        let diff = max_label.label.len() - item.label.len();
        let label = format!("{}{}", " ".repeat(diff), item.label.to_string());
        match &item.default_value {
            Some(default_value) => {
                println_config_line_default(
                    label.as_str(),
                    &item.value.to_string(),
                    &default_value.to_string(),
                )?;
            }
            None => println_config_line(label.as_str(), item.value.to_string())?,
        }
    }

    println_group_end();

    Ok(())
}

/// clears the terminal display of the output
pub fn clear() -> std::io::Result<()> {
    // Clear the terminal screen
    execute!(io::stdout(), Clear(ClearType::All))?;

    Ok(())
}

/// moves the cursor to the top left of the terminal
pub fn move_cursor_top_left() -> std::io::Result<()> {
    // Move the cursor to the top-left corner
    execute!(io::stdout(), MoveTo(0, 0))?;

    Ok(())
}
