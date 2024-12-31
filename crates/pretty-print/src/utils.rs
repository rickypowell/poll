use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

use std::io::{self};

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
