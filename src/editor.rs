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
                    code: KeyCode::Up, ..
                } => self.move_cursor(EditorKey::ArrowUp),
                KeyEvent {
                    code: KeyCode::Down,
                    ..
                } => self.move_cursor(EditorKey::ArrowDown),
                KeyEvent {
                    code: KeyCode::Left,
                    ..
                } => self.move_cursor(EditorKey::ArrowLeft),
                KeyEvent {
                    code: KeyCode::Right,
                    ..
                } => self.move_cursor(EditorKey::ArrowRight),
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
                _ => {}
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

        match key {
            ArrowLeft => {
                self.cursor.x = self.cursor.x.saturating_sub(1);
            }
            ArrowRight => self.cursor.x += 1,
            ArrowUp => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            ArrowDown => self.cursor.y += 1,
        }
    }
}
