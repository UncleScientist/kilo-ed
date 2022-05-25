use crossterm::Result;

mod keyboard;
mod screen;

mod editor;
use editor::*;

fn main() -> Result<()> {
    let mut editor = Editor::new("input.txt")?;

    editor.start()?;

    Ok(())
}
