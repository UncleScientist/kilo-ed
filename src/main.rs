use crossterm::Result;

mod keyboard;
mod screen;

mod editor;
use editor::*;

fn main() -> Result<()> {
    let mut args = std::env::args();

    let mut editor = if args.len() >= 2 {
        Editor::with_file(args.nth(1).unwrap())?
    } else {
        Editor::new()?
    };

    editor.start()?;

    Ok(())
}
