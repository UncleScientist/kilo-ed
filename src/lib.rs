pub type EditorResult<T, E> = std::result::Result<T, E>;

pub enum ResultCode {
    KeyReadFail,
}

#[derive(Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}
