use crossterm::event::{read, Event::*, KeyEvent};

use kilo_ed::*;

pub struct Keyboard;

impl Keyboard {
    pub fn read(&self) -> EditorResult<KeyEvent, ResultCode> {
        loop {
            if let Ok(event) = read() {
                if let Key(key_event) = event {
                    return Ok(key_event);
                }
            } else {
                return Err(ResultCode::KeyReadFail);
            }
        }
    }
}
