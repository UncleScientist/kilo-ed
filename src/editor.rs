use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::{terminal, Result};
use errno::errno;

use kilo_ed::*;

use crate::keyboard::*;
use crate::screen::*;

#[derive(Copy, Clone)]
enum EditorKey {
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
}

pub struct Editor {
    screen: Screen,
    keyboard: Keyboard,
    cursor: Position,
    keymap: HashMap<char, EditorKey>,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let mut keymap = HashMap::new();
        keymap.insert('w', EditorKey::ArrowUp);
        keymap.insert('s', EditorKey::ArrowDown);
        keymap.insert('a', EditorKey::ArrowLeft);
        keymap.insert('d', EditorKey::ArrowRight);
        Ok(Self {
            screen: Screen::new()?,
            keyboard: Keyboard {},
            cursor: Position::default(),
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
                    KeyCode::Up => self.move_cursor(EditorKey::ArrowUp),
                    KeyCode::Down => self.move_cursor(EditorKey::ArrowDown),
                    KeyCode::Left => self.move_cursor(EditorKey::ArrowLeft),
                    KeyCode::Right => self.move_cursor(EditorKey::ArrowRight),
                    KeyCode::PageUp | KeyCode::PageDown => {
                        let bounds = self.screen.bounds();
                        for _ in 0..bounds.y {
                            self.move_cursor(if code == KeyCode::PageUp {
                                EditorKey::ArrowUp
                            } else {
                                EditorKey::ArrowDown
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
        self.screen.draw_rows()
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
            ArrowLeft => {
                self.cursor.x = self.cursor.x.saturating_sub(1);
            }
            ArrowRight if self.cursor.x <= bounds.x => self.cursor.x += 1,
            ArrowUp => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            ArrowDown if self.cursor.y <= bounds.y => self.cursor.y += 1,
            _ => {}
        }
    }
}