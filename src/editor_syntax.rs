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
    pub multiline_comment_start: Option<String>,
    pub multiline_comment_end: Option<String>,
    pub flags: EditorFlags,
    pub keywords: Vec<Keyword>,
}

impl EditorSyntax {
    pub fn new() -> Vec<Self> {
        vec![
            // C
            EditorSyntax {
                filetype: "C".to_string(),
                filematch: vec!["c".to_string(), "h".to_string(), "cpp".to_string()],
                singleline_comment_start: Some("//".to_string()),
                multiline_comment_start: Some("/*".to_string()),
                multiline_comment_end: Some("*/".to_string()),
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
            },
            // Rust
            EditorSyntax {
                filetype: "Rust".to_string(),
                filematch: vec!["rs".to_string()],
                singleline_comment_start: Some("//".to_string()),
                multiline_comment_start: Some("/*".to_string()),
                multiline_comment_end: Some("*/".to_string()),
                flags: highlight::NUMBERS | highlight::STRINGS,
                keywords: vec![
                    Keyword::Basic("as".to_string()),
                    Keyword::Basic("async".to_string()),
                    Keyword::Basic("await".to_string()),
                    Keyword::Basic("break".to_string()),
                    Keyword::Basic("const".to_string()),
                    Keyword::Basic("continue".to_string()),
                    Keyword::Basic("crate".to_string()),
                    Keyword::Basic("dyn".to_string()),
                    Keyword::Basic("else".to_string()),
                    Keyword::Basic("enum".to_string()),
                    Keyword::Basic("extern".to_string()),
                    Keyword::Basic("false".to_string()),
                    Keyword::Basic("fn".to_string()),
                    Keyword::Basic("for".to_string()),
                    Keyword::Basic("if".to_string()),
                    Keyword::Basic("impl".to_string()),
                    Keyword::Basic("in".to_string()),
                    Keyword::Basic("let".to_string()),
                    Keyword::Basic("loop".to_string()),
                    Keyword::Basic("match".to_string()),
                    Keyword::Basic("mod".to_string()),
                    Keyword::Basic("move".to_string()),
                    Keyword::Basic("mut".to_string()),
                    Keyword::Basic("pub".to_string()),
                    Keyword::Basic("ref".to_string()),
                    Keyword::Basic("return".to_string()),
                    Keyword::Basic("self".to_string()),
                    Keyword::Basic("Self".to_string()),
                    Keyword::Basic("static".to_string()),
                    Keyword::Basic("struct".to_string()),
                    Keyword::Basic("super".to_string()),
                    Keyword::Basic("trait".to_string()),
                    Keyword::Basic("true".to_string()),
                    Keyword::Basic("type".to_string()),
                    Keyword::Basic("unsafe".to_string()),
                    Keyword::Basic("use".to_string()),
                    Keyword::Basic("where".to_string()),
                    Keyword::Basic("while".to_string()),
                    Keyword::Type("u8".to_string()),
                    Keyword::Type("i8".to_string()),
                    Keyword::Type("u16".to_string()),
                    Keyword::Type("i16".to_string()),
                    Keyword::Type("u32".to_string()),
                    Keyword::Type("i32".to_string()),
                    Keyword::Type("u64".to_string()),
                    Keyword::Type("i64".to_string()),
                    Keyword::Type("u128".to_string()),
                    Keyword::Type("i128".to_string()),
                    Keyword::Type("usize".to_string()),
                    Keyword::Type("isize".to_string()),
                    Keyword::Type("f32".to_string()),
                    Keyword::Type("f64".to_string()),
                ],
            },
        ]
    }
}
