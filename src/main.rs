use crossterm::{terminal, Result};

mod editor;
use editor::*;

fn main() -> Result<()> {
    let editor = Editor::new()?;
    println!("editor stats: {editor:?}");
    terminal::enable_raw_mode()?;

    loop {
        if editor.refresh_screen().is_err() {
            editor.die("unable to refresh screen");
        }
        if editor.process_keypress() {
            break;
        }
    }
    terminal::disable_raw_mode()?;

    Ok(())
}
