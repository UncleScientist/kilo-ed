use std::io::{stdout, Stdout, Write};

use crossterm::{cursor, terminal, QueueableCommand, Result};
use errno::errno;

pub fn clear_screen(stdout: &mut Stdout) -> Result<()> {
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()?;
    Ok(())
}

pub fn editor_refresh_screen() -> Result<()> {
    let mut stdout = stdout();

    clear_screen(&mut stdout)?;

    stdout.flush()?;
    Ok(())
}

pub fn die<S: Into<String>>(message: S) {
    let mut stdout = stdout();
    let _ = clear_screen(&mut stdout);
    let _ = terminal::disable_raw_mode();
    eprintln!("{}: {}", message.into(), errno());
    std::process::exit(1);
}
