use crossterm::event::{read, Event::*, KeyEvent};

use kilo_ed::*;

pub struct Keyboard;

pub enum InputEvent {
    Key(KeyEvent),
    Resize(u16, u16),
}

impl Keyboard {
    pub fn read(&self) -> EditorResult<InputEvent, ResultCode> {
        loop {
            if let Ok(event) = read() {
                match event {
                    Key(key_event) => return Ok(InputEvent::Key(key_event)),
                    Resize(col, row) => return Ok(InputEvent::Resize(col, row)),
                    _ => return Err(ResultCode::KeyReadFail),
                }
            }
        }
    }
}
