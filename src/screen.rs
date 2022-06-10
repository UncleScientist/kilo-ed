use std::fmt::Display;
use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor,
    style::{Color, Colors, Print, ResetColor, SetColors, SetForegroundColor},
    terminal, QueueableCommand, Result,
};

use crate::row::*;
use kilo_ed::*;

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
            height: rows - 2,
            stdout: stdout(),
        })
    }

    pub fn draw_rows(&mut self, rows: &[Row], rowoff: u16, coloff: u16) -> Result<()> {
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        for row in 0..self.height {
            let filerow = (row + rowoff) as usize;
            if filerow >= rows.len() {
                if rows.is_empty() && row == self.height / 3 {
                    let mut welcome = format!("Kilo editor -- version {VERSION}");
                    welcome.truncate(self.width as usize);
                    if welcome.len() < self.width as usize {
                        let leftmost = ((self.width as usize - welcome.len()) / 2) as u16;
                        self.stdout
                            .queue(cursor::MoveTo(0, row))?
                            .queue(Print("~".to_string()))?
                            .queue(cursor::MoveTo(leftmost, row))?
                            .queue(Print(welcome))?;
                    } else {
                        self.stdout
                            .queue(cursor::MoveTo(0, row))?
                            .queue(Print(welcome))?;
                    }
                } else {
                    self.stdout
                        .queue(cursor::MoveTo(0, row))?
                        .queue(Print("~".to_string()))?;
                }
            } else {
                let mut len = rows[filerow].render_len();
                if len < coloff as usize {
                    continue;
                }
                len -= coloff as usize;
                let start = coloff as usize;
                let end = start
                    + if len >= self.width as usize {
                        self.width as usize
                    } else {
                        len
                    };

                self.stdout.queue(cursor::MoveTo(0, row))?;
                let mut hl_iter = rows[filerow].iter_highlight(start, end);
                let mut hl = hl_iter.next();
                let mut current_color = Color::Reset;
                for c in rows[filerow].render[start..end].to_string().chars() {
                    let highlight = *hl.unwrap();
                    if highlight.is_normal() {
                        if current_color != Color::Reset {
                            self.stdout.queue(SetForegroundColor(Color::Reset))?;
                            current_color = Color::Reset;
                        }
                    } else {
                        let color = highlight.syntax_to_color();
                        if color != current_color {
                            self.stdout.queue(SetForegroundColor(color))?;
                            current_color = color;
                        }
                    }
                    self.stdout.queue(Print(c))?;
                    hl = hl_iter.next();
                }
                self.stdout.queue(SetForegroundColor(Color::Reset))?;
            }
        }
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

    pub fn move_to(
        &mut self,
        pos: &Position,
        render_x: u16,
        rowoff: u16,
        coloff: u16,
    ) -> Result<()> {
        self.stdout
            .queue(cursor::MoveTo(render_x - coloff, pos.y - rowoff))?;
        Ok(())
    }

    pub fn bounds(&self) -> Position {
        Position {
            x: self.width,
            y: self.height,
        }
    }

    pub fn draw_status_bar<T: Into<String>, U: Into<String>>(
        &mut self,
        left: T,
        right: U,
        help: impl Display,
    ) -> Result<()> {
        let left = left.into();
        let right = right.into();

        let left_width = left.len();
        let right_width = right.len();
        let screen_width = self.width as usize;

        let status = format!("{left:0$}", left_width.min(screen_width));
        let mut rstatus = String::new();
        if status.len() < screen_width - right_width {
            let mut len = status.len();
            while len < screen_width {
                if screen_width - len == right_width {
                    rstatus.push_str(right.as_str());
                    break;
                } else {
                    rstatus.push(' ');
                    len += 1;
                }
            }
        }

        self.stdout
            .queue(cursor::MoveTo(0, self.height))?
            .queue(SetColors(Colors::new(Color::Black, Color::White)))?
            .queue(Print(format!("{status}{rstatus}")))?
            .queue(cursor::MoveTo(0, self.height + 1))?
            .queue(Print(format!("{help:0$}", screen_width)))?
            .queue(ResetColor)?;
        Ok(())
    }
}
