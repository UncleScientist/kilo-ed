use std::collections::HashMap;
use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::{terminal, Result};
use errno::errno;

use kilo_ed::*;

use crate::keyboard::*;
use crate::screen::*;

#[derive(Copy, Clone)]
enum EditorKey {
    Left,
    Right,
    Up,
    Down,
}

pub struct Editor {
    screen: Screen,
    keyboard: Keyboard,
    cursor: Position,
    keymap: HashMap<char, EditorKey>,
    rows: Vec<String>,
}

impl Editor {
    pub fn with_file<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let lines = std::fs::read_to_string(filename)
            .expect("Unable to open file")
            .split('\n')
            .map(|x| x.into())
            .collect::<Vec<String>>();
        Editor::build(&lines)
    }

    pub fn new() -> Result<Self> {
        Editor::build(&[])
    }

    fn build(data: &[String]) -> Result<Self> {
        let mut keymap = HashMap::new();
        keymap.insert('w', EditorKey::Up);
        keymap.insert('s', EditorKey::Down);
        keymap.insert('a', EditorKey::Left);
        keymap.insert('d', EditorKey::Right);
        Ok(Self {
            screen: Screen::new()?,
            keyboard: Keyboard {},
            cursor: Position::default(),
            rows: if data.is_empty() {
                Vec::new()
            } else {
                Vec::from(data)
            },
            keymap,
        })
    }

    pub fn process_keypress(&mut self) -> Result<bool> {
        if let Ok(c) = self.keyboard.read() {
            match c {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                } => return Ok(true),
                KeyEvent {
                    code: KeyCode::Char(key),
                    ..
                } => match key {
                    'w' | 'a' | 's' | 'd' => {
                        let c = *self.keymap.get(&key).unwrap();
                        self.move_cursor(c);
                    }
                    _ => {}
                },
                KeyEvent { code, .. } => match code {
                    KeyCode::Home => self.cursor.x = 0,
                    KeyCode::End => self.cursor.x = self.screen.bounds().x - 1,
                    KeyCode::Up => self.move_cursor(EditorKey::Up),
                    KeyCode::Down => self.move_cursor(EditorKey::Down),
                    KeyCode::Left => self.move_cursor(EditorKey::Left),
                    KeyCode::Right => self.move_cursor(EditorKey::Right),
                    KeyCode::PageUp | KeyCode::PageDown => {
                        let bounds = self.screen.bounds();
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
        Ok(false)
    }

    pub fn start(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;

        loop {
            if self.refresh_screen().is_err() {
                self.die("unable to refresh screen");
            }
            self.screen.move_to(&self.cursor)?;
            self.screen.flush()?;
            if self.process_keypress()? {
                break;
            }
        }
        terminal::disable_raw_mode()
    }

    pub fn refresh_screen(&mut self) -> Result<()> {
        self.screen.clear()?;
        self.screen.draw_rows(&self.rows)
    }

    pub fn die<S: Into<String>>(&mut self, message: S) {
        let _ = self.screen.clear();
        let _ = terminal::disable_raw_mode();
        eprintln!("{}: {}", message.into(), errno());
        std::process::exit(1);
    }

    fn move_cursor(&mut self, key: EditorKey) {
        use EditorKey::*;

        let bounds = self.screen.bounds();
        match key {
            Left => {
                self.cursor.x = self.cursor.x.saturating_sub(1);
            }
            Right if self.cursor.x <= bounds.x => self.cursor.x += 1,
            Up => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            Down if self.cursor.y <= bounds.y => self.cursor.y += 1,
            _ => {}
        }
    }
}
