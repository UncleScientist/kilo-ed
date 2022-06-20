pub type EditorFlags = u32;

pub mod highlight {
    use super::*;

    pub const NUMBERS: EditorFlags = 0x0000_0001;
}

pub struct EditorSyntax {
    pub filetype: String,
    pub filematch: Vec<String>,
    pub flags: EditorFlags,
}

impl EditorSyntax {
    pub fn new() -> Vec<Self> {
        vec![EditorSyntax {
            filetype: "c".to_string(),
            filematch: vec!["c".to_string(), "h".to_string(), "cpp".to_string()],
            flags: highlight::NUMBERS,
        }]
    }
}
