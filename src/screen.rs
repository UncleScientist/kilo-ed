use std::cmp::Ordering;
use std::fmt::Display;
use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
    terminal, QueueableCommand, Result,
};

use crate::options::*;
use crate::row::*;
use kilo_ed::*;

pub struct Screen {
    stdout: Stdout,
    width: u16,
    height: u16,
    gaps: Vec<u16>,
    options: Options,
    ln_shift: u16,
}

const LNO_SHIFT: u16 = 7;

impl Screen {
    pub fn new(options: Options) -> Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            width: columns,
            height: rows - 2,
            stdout: stdout(),
            gaps: Vec::new(),
            options,
            ln_shift: if options.lines == LineNumbers::Off {
                0
            } else {
                LNO_SHIFT
            },
        })
    }

    pub fn resize(&mut self, columns: u16, rows: u16) {
        self.width = columns;
        self.height = rows - 2;
    }

    pub fn draw_rows(&mut self, rows: &[Row], rowoff: u16, coloff: u16, crow: u16) -> Result<()> {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        self.gaps.clear();
        self.gaps.push(0);
        let mut gaps = 0;

        for row in 0..self.height {
            if self.options.soft_wrap() && row + gaps >= self.height {
                break;
            }
            let filerow = (row + rowoff) as usize;
            if filerow >= rows.len() {
                if rows.is_empty() && row == self.height / 3 {
                    let mut welcome = format!("Kilo editor -- version {VERSION}");
                    welcome.truncate(self.width as usize);
                    if welcome.len() < self.width as usize {
                        let leftmost = ((self.width as usize - welcome.len()) / 2) as u16;
                        self.stdout
                            .queue(cursor::MoveTo(self.ln_shift, row))?
                            .queue(Print("~".to_string()))?
                            .queue(cursor::MoveTo(leftmost + self.ln_shift, row))?
                            .queue(Print(welcome))?;
                    } else {
                        self.stdout
                            .queue(cursor::MoveTo(self.ln_shift, row))?
                            .queue(Print(welcome))?;
                    }
                } else {
                    self.stdout
                        .queue(cursor::MoveTo(self.ln_shift, row))?
                        .queue(Print("~".to_string()))?;
                }
            } else {
                // Display line number on the left
                let order = crow.cmp(&(filerow as u16));
                let gutter_num = if self.options.lines == LineNumbers::Relative {
                    match order {
                        Ordering::Less => filerow as u16 - crow,
                        Ordering::Equal => (filerow + 1) as u16,
                        Ordering::Greater => crow - filerow as u16,
                    }
                } else {
                    (filerow + 1) as u16
                };

                if matches!(
                    self.options.lines,
                    LineNumbers::Absolute | LineNumbers::Relative
                ) {
                    self.stdout
                        .queue(SetAttribute(Attribute::Reset))?
                        .queue(cursor::MoveTo(0, row + gaps))?
                        .queue(
                            if order == Ordering::Equal
                                && self.options.lines == LineNumbers::Relative
                            {
                                Print(format!("{gutter_num:<5}"))
                            } else {
                                Print(format!("{gutter_num:5}"))
                            },
                        )?;
                }

                let start = if self.options.soft_wrap() {
                    0
                } else {
                    coloff as usize
                };
                let end = if !self.options.soft_wrap() {
                    let mut len = rows[filerow].render_len();
                    if len < coloff as usize {
                        continue;
                    }
                    len -= coloff as usize;
                    start
                        + if len >= (self.width - self.ln_shift) as usize {
                            (self.width - self.ln_shift) as usize
                        } else {
                            len
                        }
                } else {
                    (self.width - self.ln_shift) as usize
                };

                let mut hl_iter = rows[filerow].iter_highlight(start);
                let mut hl = hl_iter.next();
                let mut current_color = Color::Reset;

                // Draw row in remaining columns
                let mut screen_row_count = 0;
                if end > start {
                    for s in rows[filerow].render[start..]
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(end - start)
                        .collect::<Vec<&[char]>>()
                    {
                        self.stdout
                            .queue(cursor::MoveTo(self.ln_shift, row + gaps + screen_row_count))?;
                        for c in s {
                            if c.is_ascii_control() {
                                let sym = (*c as u8 + b'@') as char;
                                self.stdout
                                    .queue(SetAttribute(Attribute::Reverse))?
                                    .queue(Print(sym))?
                                    .queue(SetAttribute(Attribute::Reset))?;
                                if current_color != Color::Reset {
                                    self.stdout.queue(SetForegroundColor(current_color))?;
                                }
                            } else {
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
                        }
                        if !self.options.soft_wrap() {
                            break;
                        }
                        screen_row_count += 1;
                    }
                }
                if screen_row_count > 1 {
                    gaps += screen_row_count - 1;
                }
                self.stdout.queue(SetForegroundColor(Color::Reset))?;
            }
            self.gaps.push(gaps);
        }
        Ok(())
    }

    pub fn clear(&mut self, rows: &[Row], rowoff: u16) -> Result<u16> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
        Ok(if self.options.soft_wrap() {
            let mut count = 0;
            let mut display_height = 0u16;
            for r in &rows[rowoff as usize..] {
                count += 1;
                display_height += (r.len() / ((self.width - self.ln_shift) as usize) + 1) as u16;
                if display_height >= self.height {
                    break;
                }
            }
            count
        } else {
            self.height
        })
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
        let display_width = self.width - self.ln_shift;
        let shift_y = pos.x / display_width;

        let pos_x = if self.options.soft_wrap() {
            pos.x % display_width + self.ln_shift
        } else {
            render_x - coloff + self.ln_shift
        };

        let pos_y = if self.options.soft_wrap() {
            pos.y - rowoff + shift_y + self.gaps[(pos.y - rowoff) as usize]
        } else {
            pos.y - rowoff
        };

        self.stdout.queue(cursor::MoveTo(pos_x, pos_y))?;

        Ok(())
    }

    pub fn bounds(&self) -> Position {
        Position {
            x: self.width - self.ln_shift,
            y: self.height - self.gaps.last().unwrap_or(&0),
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
        if right_width < screen_width && status.len() < screen_width - right_width {
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
            // .queue(SetColors(Colors::new(Color::Black, Color::White)))?
            .queue(SetAttribute(Attribute::Reverse))?
            .queue(Print(format!("{status}{rstatus}")))?
            .queue(cursor::MoveTo(0, self.height + 1))?
            .queue(Print(format!("{help:0$}", screen_width)))?
            .queue(SetAttribute(Attribute::Reset))?;
        Ok(())
    }
}
