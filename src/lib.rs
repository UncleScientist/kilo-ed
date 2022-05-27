pub type EditorResult<T, E> = std::result::Result<T, E>;

pub enum ResultCode {
    KeyReadFail,
}

#[derive(Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn above(&self, row: usize) -> bool {
        self.y < row as u16
    }

    pub fn left_of(&self, col: usize) -> bool {
        self.x < col as u16
    }

    pub fn row(&self) -> usize {
        self.y as usize
    }
}
