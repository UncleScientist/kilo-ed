use crossterm::{cursor, style::Print, terminal, QueueableCommand, Result};
use std::io::{stdout, Stdout, Write};

pub struct Screen {
    stdout: Stdout,
    width: u16,
    height: u16,
}

impl Screen {
    pub fn new() -> Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            width: columns,
            height: rows,
            stdout: stdout(),
        })
    }

    pub fn draw_rows(&mut self) -> Result<()> {
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        for row in 0..self.height {
            if row == self.height / 3 {
                let mut welcome = format!("Kilo editor -- version {VERSION}");
                welcome.truncate(self.width as usize);
                self.stdout
                    .queue(cursor::MoveTo(0, row))?
                    .queue(Print(welcome))?;
            } else {
                self.stdout
                    .queue(cursor::MoveTo(0, row))?
                    .queue(Print("~".to_string()))?;
            }
        }
        self.stdout.queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }

    pub fn cursor_position(&self) -> Result<(u16, u16)> {
        cursor::position()
    }
}
