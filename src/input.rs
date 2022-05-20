use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::keyboard::*;

pub fn editor_process_keypress() -> bool {
    let c = editor_read_key();

    match c {
        Ok(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
        }) => true,
        _ => false,
    }
}
