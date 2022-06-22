use crate::editor_syntax::*;
use crossterm::style::Color;

const KILO_TAB_STOP: usize = 8;

#[derive(Copy, Clone, PartialEq)]
pub enum Highlight {
    Normal,
    Number,
    Match,
    String,
    Comment,
    Keyword1,
    Keyword2,
    MultilineComment,
}

impl Highlight {
    pub fn syntax_to_color(&self) -> Color {
        match self {
            Highlight::Normal => Color::White,
            Highlight::Number => Color::Red,
            Highlight::Match => Color::Blue,
            Highlight::String => Color::Magenta,
            Highlight::Comment => Color::Cyan,
            Highlight::Keyword1 => Color::Yellow,
            Highlight::Keyword2 => Color::Green,
            Highlight::MultilineComment => Color::Cyan,
        }
    }

    pub fn is_normal(&self) -> bool {
        self == &Highlight::Normal
    }
}

pub struct Row {
    pub chars: String,
    pub render: String,
    hl: Vec<Highlight>,
    saved_highlight: Vec<Highlight>,
    pub open_comment: bool,
}

impl Row {
    pub fn new(chars: String) -> Self {
        let mut result = Self {
            chars,
            render: String::new(),
            hl: Vec::new(),
            saved_highlight: Vec::new(),
            open_comment: false,
        };

        result.render_row();
        result
    }

    pub fn render_len(&self) -> usize {
        self.render.len()
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn cx_to_rx(&self, cx: u16) -> u16 {
        let mut rx = 0;
        for c in self.chars.chars().take(cx as usize) {
            if c == '\t' {
                rx += (KILO_TAB_STOP - 1) - (rx % KILO_TAB_STOP);
            }
            rx += 1;
        }
        rx as u16
    }

    pub fn rx_to_cx(&self, rx: usize) -> u16 {
        let mut cur_rx = 0;
        for (cx, c) in self.chars.chars().enumerate() {
            if c == '\t' {
                cur_rx += (KILO_TAB_STOP - 1) - (cur_rx % KILO_TAB_STOP);
            }
            cur_rx += 1;
            if cur_rx > rx {
                return cx as u16;
            }
        }
        self.chars.len() as u16
    }

    pub fn insert_char(&mut self, at: usize, c: char) {
        if at >= self.chars.len() {
            self.chars.push(c);
        } else {
            self.chars.insert(at, c);
        }
        self.render_row();
    }

    /* returns true if row was modified, false otherwise */
    pub fn del_char(&mut self, at: usize) -> bool {
        if at >= self.chars.len() {
            false
        } else {
            self.chars.remove(at);
            self.render_row();
            true
        }
    }

    pub fn split(&mut self, at: usize) -> String {
        let result = self.chars.split_off(at);
        self.render_row();

        result
    }

    pub fn append_string(&mut self, s: &str) {
        self.chars.push_str(s);
        self.render_row();
    }

    fn render_row(&mut self) {
        let mut render = String::new();
        let mut idx = 0;
        for c in self.chars.chars() {
            match c {
                '\t' => {
                    render.push(' ');
                    idx += 1;
                    while idx % KILO_TAB_STOP != 0 {
                        render.push(' ');
                        idx += 1;
                    }
                }
                _ => {
                    render.push(c);
                    idx += 1;
                }
            }
        }

        self.render = render;
    }

    // returns true if we're in the middle of a multi-line comment
    pub fn update_syntax(&mut self, ml_comment: bool, syntax: &Option<EditorSyntax>) -> bool {
        self.hl = vec![Highlight::Normal; self.render.len()];

        let syntax = if let Some(syntax) = syntax {
            syntax
        } else {
            return ml_comment;
        };

        let mut prev_sep = true;
        let mut row_iter = self.render.chars().enumerate();
        let mut in_string = None;
        let mut in_comment = ml_comment;
        let scs = syntax.singleline_comment_start.as_ref();
        let mcs = syntax.multiline_comment_start.as_ref();
        let mce = syntax.multiline_comment_end.as_ref();

        'outer: while let Some((i, c)) = row_iter.next() {
            let prev_hl = if i > 0 {
                self.hl[i - 1]
            } else {
                Highlight::Normal
            };

            if in_string.is_none() && scs.is_some() && !in_comment {
                if let Some(scs) = scs {
                    let len = scs.len();

                    if self.render.len() - i >= len && &self.render[i..i + len] == scs {
                        self.hl[i..self.render.len()].fill(Highlight::Comment);
                        break;
                    }
                }
            }

            if mcs.is_some() && mce.is_some() && in_string.is_none() {
                if in_comment {
                    self.hl[i] = Highlight::MultilineComment;
                    if let Some(mce) = mce {
                        let len = mce.len();
                        if self.render.len() - i >= len && &self.render[i..i + len] == mce {
                            self.hl[i..i + len].fill(Highlight::MultilineComment);
                            row_iter.nth(len - 2);
                            in_comment = false;
                            prev_sep = true;
                        }
                        continue;
                    }
                } else if let Some(mcs) = mcs {
                    let len = mcs.len();
                    if self.render.len() - i >= len && &self.render[i..i + len] == mcs {
                        self.hl[i..i + len].fill(Highlight::MultilineComment);
                        row_iter.nth(len - 2);
                        in_comment = true;
                        continue;
                    }
                }
            }

            if syntax.flags & highlight::STRINGS != 0 {
                if let Some(cur) = in_string {
                    self.hl[i] = Highlight::String;
                    if c == '\\' && i + 1 < self.render.len() {
                        self.hl[i + 1] = Highlight::String;
                        row_iter.nth(1); // skip 1
                        continue;
                    }
                    if c == cur {
                        in_string = None;
                    }
                    prev_sep = true;
                    continue;
                } else if c == '"' || c == '\'' {
                    in_string = Some(c);
                    self.hl[i] = Highlight::String;
                    continue;
                }
            }

            if (syntax.flags & highlight::NUMBERS) != 0
                && ((c.is_digit(10) && (prev_sep || prev_hl == Highlight::Number))
                    || (c == '.' && prev_hl == Highlight::Number))
            {
                self.hl[i] = Highlight::Number;
                prev_sep = false;
                continue;
            }

            if prev_sep {
                for keyword in &syntax.keywords {
                    let (key, is_type_1) = match keyword {
                        Keyword::Basic(x) => (x, true),
                        Keyword::Type(x) => (x, false),
                    };

                    let klen = key.len();
                    let last_is_sep = if let Some(ch) = self.render.chars().nth(i + klen) {
                        ch.is_separator()
                    } else {
                        true
                    };

                    if self.render.len() - i >= klen
                        && &self.render[i..i + klen] == key
                        && last_is_sep
                    {
                        self.hl[i..i + klen].fill(if is_type_1 {
                            Highlight::Keyword1
                        } else {
                            Highlight::Keyword2
                        });
                        row_iter.nth(klen - 2); // skip keyword
                        prev_sep = false;
                        continue 'outer;
                    }
                }
            }

            prev_sep = c.is_separator();
        }

        let changed = self.open_comment != in_comment;
        self.open_comment = in_comment;

        changed
    }

    pub fn iter_highlight(&self, start: usize, end: usize) -> std::slice::Iter<Highlight> {
        self.hl[start..end].iter()
    }

    pub fn highlight_match(&mut self, start: usize, len: usize) {
        self.saved_highlight = self.hl.clone();
        for c in self.hl[start..start + len].iter_mut() {
            *c = Highlight::Match;
        }
    }

    pub fn reset_match(&mut self) {
        self.hl = self.saved_highlight.clone();
        self.saved_highlight.clear();
    }
}

// -----

trait Separator {
    fn is_separator(&self) -> bool;
}

impl Separator for char {
    fn is_separator(&self) -> bool {
        matches!(
            self,
            ' ' | ','
                | '.'
                | '('
                | ')'
                | '+'
                | '-'
                | '/'
                | '*'
                | '='
                | '~'
                | '%'
                | '<'
                | '>'
                | '['
                | ']'
                | ';'
        )
    }
}
