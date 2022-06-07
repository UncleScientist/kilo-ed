use std::path::Path;
use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::{terminal, Result};
use errno::errno;

use kilo_ed::*;

use crate::keyboard::*;
use crate::row::*;
use crate::screen::*;

const KILO_QUIT_TIMES: usize = 3;

#[derive(Copy, Clone)]
enum EditorKey {
    Left,
    Right,
    Up,
    Down,
}

pub struct Editor {
    filename: String,
    status_msg: String,
    status_time: Instant,
    screen: Screen,
    keyboard: Keyboard,
    cursor: Position,
    render_x: u16,
    rows: Vec<Row>,
    rowoff: u16,
    coloff: u16,
    dirty: usize,
    quit_times: usize,
}

impl Editor {
    pub fn with_file<P: AsRef<Path> + ToString>(filename: P) -> Result<Self> {
        let fn_string = filename.to_string();
        let lines = std::fs::read_to_string(filename)
            .expect("Unable to open file")
            .split('\n')
            .map(|x| x.into())
            .collect::<Vec<String>>();
        Editor::build(&lines, fn_string)
    }

    pub fn new() -> Result<Self> {
        Editor::build(&[], "")
    }

    fn build<T: Into<String>>(data: &[String], filename: T) -> Result<Self> {
        Ok(Self {
            filename: filename.into(),
            status_msg: String::from("HELP: Ctrl-S = save | Ctrl-Q = quit"),
            status_time: Instant::now(),
            screen: Screen::new()?,
            keyboard: Keyboard {},
            cursor: Position::default(),
            rows: if data.is_empty() {
                Vec::new()
            } else {
                let v = Vec::from(data);
                let mut rows = Vec::new();
                for row in v {
                    rows.push(Row::new(row))
                }
                if rows.last().unwrap().len() == 0 {
                    rows.pop();
                }
                rows
            },
            rowoff: 0,
            coloff: 0,
            render_x: 0,
            dirty: 0,
            quit_times: KILO_QUIT_TIMES,
        })
    }

    pub fn process_keypress(&mut self) -> Result<bool> {
        if let Ok(c) = self.keyboard.read() {
            match c {
                /*
                 * Ctrl-Q to quit
                 */
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                } => {
                    if self.dirty > 0 && self.quit_times > 0 {
                        self.set_status_message(format!(
                            "WARNING!!! File has unsaved changes. \
                                Press Ctrl-Q {} more times to quit.",
                            self.quit_times
                        ));
                        self.quit_times -= 1;
                        return Ok(false);
                    } else {
                        return Ok(true);
                    }
                }

                /*
                 * Ctrl-S to save
                 */
                KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                } => {
                    self.save();
                }

                /*
                 * Ignore Ctrl-L and Escape keys
                 */
                KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::CONTROL,
                }
                | KeyEvent {
                    code: KeyCode::Esc, ..
                } => {}

                /*
                 * Ctrl-h or Backspace or Delete to delete characters
                 */
                KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: KeyModifiers::CONTROL,
                }
                | KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }
                | KeyEvent {
                    code: KeyCode::Delete,
                    ..
                } => {
                    if let KeyEvent {
                        code: KeyCode::Delete,
                        ..
                    } = c
                    {
                        self.move_cursor(EditorKey::Right);
                    }
                    self.del_char();
                }

                /*
                 * Any 'regular' key gets inserted
                 */
                KeyEvent {
                    code: KeyCode::Char(key),
                    modifiers: KeyModifiers::NONE,
                }
                | KeyEvent {
                    code: KeyCode::Char(key),
                    modifiers: KeyModifiers::SHIFT,
                } => self.insert_char(key),

                /*
                 * Handle all other special keycodes
                 */
                KeyEvent { code, .. } => match code {
                    KeyCode::Enter => {
                        self.insert_newline();
                    }
                    KeyCode::Home => self.cursor.x = 0,
                    KeyCode::End => self.cursor.x = self.current_row_len(),
                    KeyCode::Up => self.move_cursor(EditorKey::Up),
                    KeyCode::Down => self.move_cursor(EditorKey::Down),
                    KeyCode::Left => self.move_cursor(EditorKey::Left),
                    KeyCode::Right => self.move_cursor(EditorKey::Right),
                    KeyCode::PageUp | KeyCode::PageDown => {
                        let bounds = self.screen.bounds();

                        match code {
                            KeyCode::PageUp => self.cursor.y = self.rowoff,
                            KeyCode::PageDown => {
                                self.cursor.y =
                                    (self.rowoff + bounds.y - 1).min(self.rows.len() as u16)
                            }
                            _ => panic!("rust compiler broke"),
                        }

                        for _ in 0..bounds.y {
                            self.move_cursor(if code == KeyCode::PageUp {
                                EditorKey::Up
                            } else {
                                EditorKey::Down
                            })
                        }
                    }
                    _ => {}
                },
            }
        } else {
            self.die("Unable to read from keyboard");
        }
        self.quit_times = KILO_QUIT_TIMES;
        Ok(false)
    }

    pub fn start(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;

        loop {
            if self.refresh_screen().is_err() {
                self.die("unable to refresh screen");
            }
            self.screen
                .move_to(&self.cursor, self.render_x, self.rowoff, self.coloff)?;
            self.screen.flush()?;
            if self.process_keypress()? {
                break;
            }
        }
        terminal::disable_raw_mode()
    }

    pub fn refresh_screen(&mut self) -> Result<()> {
        self.scroll();
        self.screen.clear()?;
        self.screen
            .draw_rows(&self.rows, self.rowoff, self.coloff)?;
        if !self.status_msg.is_empty() && self.status_time.elapsed() > Duration::from_secs(5) {
            self.status_msg.clear();
        }
        self.screen.draw_status_bar(
            format!(
                "{:20} - {} lines {}",
                if self.filename.is_empty() {
                    "[No Name]"
                } else {
                    &self.filename
                },
                self.rows.len(),
                if self.dirty > 0 { "(modified)" } else { "" }
            ),
            format!("{}/{}", self.cursor.y, self.rows.len()),
            &self.status_msg,
        )
    }

    pub fn die<S: Into<String>>(&mut self, message: S) {
        let _ = self.screen.clear();
        let _ = terminal::disable_raw_mode();
        eprintln!("{}: {}", message.into(), errno());
        std::process::exit(1);
    }

    fn move_cursor(&mut self, key: EditorKey) {
        use EditorKey::*;

        match key {
            Left => {
                if self.cursor.x != 0 {
                    self.cursor.x -= 1;
                } else if self.cursor.y > 0 {
                    self.cursor.y -= 1;
                    self.cursor.x = self.rows[self.cursor.row()].len() as u16;
                }
            }
            Right => {
                if (self.cursor.y as usize) < self.rows.len() {
                    let idx = self.cursor.row();
                    if self.cursor.left_of(self.rows[idx].len()) {
                        self.cursor.x += 1;
                    } else if self.cursor.above(self.rows.len()) {
                        self.cursor.y += 1;
                        self.cursor.x = 0;
                    }
                }
            }
            Up => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            Down if self.cursor.y < self.rows.len() as u16 => self.cursor.y += 1,
            _ => {}
        }

        let rowlen = self.current_row_len();
        self.cursor.x = self.cursor.x.min(rowlen);
    }

    fn scroll(&mut self) {
        self.render_x = if self.cursor.above(self.rows.len()) {
            self.rows[self.cursor.y as usize].cx_to_rx(self.cursor.x)
        } else {
            0
        };

        let bounds = self.screen.bounds();
        if self.cursor.y < self.rowoff {
            self.rowoff = self.cursor.y;
        }
        if self.cursor.y >= self.rowoff + bounds.y {
            self.rowoff = self.cursor.y - bounds.y + 1;
        }

        if self.render_x < self.coloff {
            self.coloff = self.render_x;
        }

        if self.render_x >= self.coloff + bounds.x {
            self.coloff = self.render_x - bounds.x + 1;
        }
    }

    fn current_row_len(&self) -> u16 {
        if self.cursor.above(self.rows.len()) {
            self.rows[self.cursor.y as usize].len() as u16
        } else {
            0
        }
    }

    fn insert_char(&mut self, c: char) {
        if !self.cursor.above(self.rows.len()) {
            self.insert_row(self.rows.len(), String::new());
        }
        self.rows[self.cursor.y as usize].insert_char(self.cursor.x as usize, c);
        self.cursor.x += 1;
        self.dirty += 1;
    }

    fn del_char(&mut self) {
        if !self.cursor.above(self.rows.len()) {
            return;
        }
        if self.cursor.x == 0 && self.cursor.y == 0 {
            return;
        }

        let cur_row = self.cursor.y as usize;

        if self.cursor.x > 0 {
            if self.rows[cur_row].del_char(self.cursor.x as usize - 1) {
                self.dirty += 1;
                self.cursor.x -= 1;
            }
        } else {
            self.cursor.x = self.rows[cur_row - 1].len() as u16;
            if let Some(row) = self.del_row(cur_row) {
                self.rows[cur_row - 1].append_string(&row);
                self.cursor.y -= 1;
                self.dirty += 1;
            }
        }
    }

    fn insert_newline(&mut self) {
        let row = self.cursor.y as usize;

        if self.cursor.x == 0 {
            self.insert_row(row, String::from(""));
        } else {
            let new_row = self.rows[row].split(self.cursor.x as usize);
            self.insert_row(row + 1, new_row);
        }
        self.cursor.y += 1;
        self.cursor.x = 0;
    }

    fn insert_row(&mut self, at: usize, s: String) {
        if at > self.rows.len() {
            return;
        }

        self.rows.insert(at, Row::new(s));
        self.dirty += 1;
    }

    fn del_row(&mut self, at: usize) -> Option<String> {
        if at >= self.rows.len() {
            None
        } else {
            self.dirty += 1;
            Some(self.rows.remove(at).chars)
        }
    }

    fn rows_to_string(&self) -> String {
        let mut buf = String::new();
        for r in &self.rows {
            buf.push_str(r.chars.as_str());
            buf.push('\n');
        }

        buf
    }

    fn save(&mut self) {
        if self.filename.is_empty() {
            self.filename = self.editor_prompt("Save as");
        }

        let buf = self.rows_to_string();
        let len = buf.as_bytes().len();
        if std::fs::write(&self.filename, &buf).is_ok() {
            self.dirty = 0;
            self.set_status_message(&format!("{len} bytes written to disk"));
        } else {
            self.set_status_message(&format!("Can't save! I/O error: {}", errno()));
        }
    }

    fn editor_prompt(&mut self, prompt: &str) -> String {
        let mut buf = String::from("");

        loop {
            self.set_status_message(&format!("{}: {}", prompt, buf));
            let _ = self.refresh_screen();

            let _ = self.screen.flush();
            if let Ok(c) = self.keyboard.read() {
                match c {
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        self.set_status_message("".to_string());
                        return buf;
                    }
                    KeyEvent {
                        code: KeyCode::Char(ch),
                        modifiers: modif,
                    } => {
                        if matches!(modif, KeyModifiers::NONE | KeyModifiers::SHIFT) {
                            buf.push(ch);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn set_status_message<T: Into<String>>(&mut self, message: T) {
        self.status_time = Instant::now();
        self.status_msg = message.into();
    }
}
