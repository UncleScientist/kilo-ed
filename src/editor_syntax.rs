pub type EditorFlags = u32;

pub mod highlight {
    use super::*;

    pub const NUMBERS: EditorFlags = 1 << 0;
    pub const STRINGS: EditorFlags = 1 << 1;
}

pub struct EditorSyntax {
    pub filetype: String,
    pub filematch: Vec<String>,
    pub singleline_comment_start: Option<String>,
    pub flags: EditorFlags,
}

impl EditorSyntax {
    pub fn new() -> Vec<Self> {
        vec![EditorSyntax {
            filetype: "c".to_string(),
            filematch: vec!["c".to_string(), "h".to_string(), "cpp".to_string()],
            singleline_comment_start: Some("//".to_string()),
            flags: highlight::NUMBERS | highlight::STRINGS,
        }]
    }
}
