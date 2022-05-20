use crossterm::{event::Event::*, terminal, Result};

mod keyboard;

mod output;
use output::*;

mod input;
use input::*;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if editor_refresh_screen().is_err() {
            die("unable to refresh screen");
        }
        if editor_process_keypress() {
            break;
        }
    }
    terminal::disable_raw_mode()?;

    Ok(())
}
