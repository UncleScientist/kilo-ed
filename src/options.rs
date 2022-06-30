// -----------------------------------------------------------------------------
//     - Line Numbers -
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LineNumbers {
    Off,
    Absolute,
    Relative,
}

impl From<String> for LineNumbers {
    fn from(s: String) -> LineNumbers {
        match s.as_str() {
            "absolute" => LineNumbers::Absolute,
            "relative" => LineNumbers::Relative,
            _ => LineNumbers::Off,
        }
    }
}

impl Default for LineNumbers {
    fn default() -> Self {
        LineNumbers::Off
    }
}

impl ConvertOptString for LineNumbers {}

// -----------------------------------------------------------------------------
//     - Line Display-
// -----------------------------------------------------------------------------

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LineDisplay {
    Wrap,
    Scroll,
}

impl From<String> for LineDisplay {
    fn from(s: String) -> LineDisplay {
        match s.as_str() {
            "wrap" => LineDisplay::Wrap,
            _ => LineDisplay::Scroll,
        }
    }
}

impl Default for LineDisplay {
    fn default() -> Self {
        LineDisplay::Scroll
    }
}

impl ConvertOptString for LineDisplay {}

// -----------------------------------------------------------------------------
//     - Options Infrastructure-
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Options {
    pub lines: LineNumbers,
    pub soft_wrap: LineDisplay,
}

impl Options {
    pub fn soft_wrap(&self) -> bool {
        self.soft_wrap == LineDisplay::Wrap
    }
}

pub trait ConvertOptString: From<String> + Default + core::fmt::Debug {}
