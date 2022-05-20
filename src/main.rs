use errno::errno;

use crossterm::{event::Event::*, terminal, Result};

mod keyboard;

mod input;
use input::*;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if editor_process_keypress() {
            break;
        }
    }
    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn die<S: Into<String>>(message: S) {
    let _ = terminal::disable_raw_mode();
    eprintln!("{}: {}", message.into(), errno());
    std::process::exit(1);
}
