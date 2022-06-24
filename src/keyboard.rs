use crossterm::event::{read, Event::*, KeyEvent};

use kilo_ed::*;

pub struct Keyboard;

impl Keyboard {
    pub fn read(&self) -> EditorResult<KeyEvent, ResultCode> {
        loop {
            if let Ok(event) = read() {
                match event {
                    Key(key_event) => return Ok(key_event),
                    Resize(col, row) => return Err(ResultCode::Resized(col, row)),
                    _ => return Err(ResultCode::KeyReadFail),
                }
            }
        }
    }
}
