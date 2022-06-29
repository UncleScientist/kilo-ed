#[derive(PartialEq, Copy, Clone)]
pub enum LineNumbers {
    Off,
    Absolute,
    Relative,
}

#[derive(Copy, Clone)]
pub struct Options {
    pub lines: LineNumbers,
    pub soft_wrap: bool,
}
