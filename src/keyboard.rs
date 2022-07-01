use crossterm::event::{Event::*, KeyEvent, MouseEventKind};

use kilo_ed::*;

pub struct Keyboard;

pub enum InputEvent {
    Key(KeyEvent),
    Resize(u16, u16),
    ScrollUp,
    ScrollDown,
}

impl Keyboard {
    pub fn read(&self) -> EditorResult<InputEvent, ResultCode> {
        loop {
            if let Ok(event) = crossterm::event::read() {
                match event {
                    Key(key_event) => return Ok(InputEvent::Key(key_event)),
                    Resize(col, row) => return Ok(InputEvent::Resize(col, row)),
                    Mouse(me) => match me.kind {
                        MouseEventKind::ScrollUp => return Ok(InputEvent::ScrollUp),
                        MouseEventKind::ScrollDown => return Ok(InputEvent::ScrollDown),
                        _ => {}
                    },
                }
            } else {
                return Err(ResultCode::KeyReadFail);
            }
        }
    }
}
