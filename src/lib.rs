pub type EditorResult<T, E> = std::result::Result<T, E>;

pub enum ResultCode {
    KeyReadFail,
}

impl std::fmt::Display for ResultCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultCode::KeyReadFail => write!(f, "failed to read key"),
        }
    }
}

#[derive(Default, Clone, Copy)]
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
