pub type EditorFlags = u32;

pub mod highlight {
    use super::*;

    pub const NUMBERS: EditorFlags = 1 << 0;
    pub const STRINGS: EditorFlags = 1 << 1;
}

#[derive(Clone)]
pub enum Keyword {
    Basic(String),
    Type(String),
}

#[derive(Clone)]
pub struct EditorSyntax {
    pub filetype: String,
    pub filematch: Vec<String>,
    pub singleline_comment_start: Option<String>,
    pub flags: EditorFlags,
    pub keywords: Vec<Keyword>,
}

impl EditorSyntax {
    pub fn new() -> Vec<Self> {
        vec![EditorSyntax {
            filetype: "c".to_string(),
            filematch: vec!["c".to_string(), "h".to_string(), "cpp".to_string()],
            singleline_comment_start: Some("//".to_string()),
            flags: highlight::NUMBERS | highlight::STRINGS,
            keywords: vec![
                Keyword::Basic("switch".to_string()),
                Keyword::Basic("if".to_string()),
                Keyword::Basic("while".to_string()),
                Keyword::Basic("for".to_string()),
                Keyword::Basic("break".to_string()),
                Keyword::Basic("continue".to_string()),
                Keyword::Basic("return".to_string()),
                Keyword::Basic("else".to_string()),
                Keyword::Basic("struct".to_string()),
                Keyword::Basic("union".to_string()),
                Keyword::Basic("typedef".to_string()),
                Keyword::Basic("static".to_string()),
                Keyword::Basic("enum".to_string()),
                Keyword::Basic("class".to_string()),
                Keyword::Basic("case".to_string()),
                Keyword::Type("int".to_string()),
                Keyword::Type("long".to_string()),
                Keyword::Type("double".to_string()),
                Keyword::Type("float".to_string()),
                Keyword::Type("char".to_string()),
                Keyword::Type("unsigned".to_string()),
                Keyword::Type("signed".to_string()),
                Keyword::Type("void".to_string()),
            ],
        }]
    }
}
