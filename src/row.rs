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
}

impl Highlight {
    pub fn syntax_to_color(&self) -> Color {
        match self {
            Highlight::Normal => Color::White,
            Highlight::Number => Color::Red,
            Highlight::Match => Color::Blue,
            Highlight::String => Color::Magenta,
            Highlight::Comment => Color::Cyan,
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
}

impl Row {
    pub fn new(chars: String, flags: EditorFlags) -> Self {
        let mut result = Self {
            chars,
            render: String::new(),
            hl: Vec::new(),
            saved_highlight: Vec::new(),
        };

        result.render_row(flags);
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

    pub fn insert_char(&mut self, at: usize, c: char, flags: EditorFlags) {
        if at >= self.chars.len() {
            self.chars.push(c);
        } else {
            self.chars.insert(at, c);
        }
        self.render_row(flags);
    }

    /* returns true if row was modified, false otherwise */
    pub fn del_char(&mut self, at: usize, flags: EditorFlags) -> bool {
        if at >= self.chars.len() {
            false
        } else {
            self.chars.remove(at);
            self.render_row(flags);
            true
        }
    }

    pub fn split(&mut self, at: usize, flags: EditorFlags) -> String {
        let result = self.chars.split_off(at);
        self.render_row(flags);

        result
    }

    pub fn append_string(&mut self, s: &str, flags: EditorFlags) {
        self.chars.push_str(s);
        self.render_row(flags);
    }

    fn render_row(&mut self, flags: EditorFlags) {
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
        self.update_syntax(flags);
    }

    pub fn update_syntax(&mut self, flags: EditorFlags) {
        self.hl = vec![Highlight::Normal; self.render.len()];

        if flags == 0 {
            return;
        }

        let mut prev_sep = true;
        let mut row_iter = self.render.chars().enumerate();
        let mut in_string = None;

        while let Some((i, c)) = row_iter.next() {
            let prev_hl = if i > 0 {
                self.hl[i - 1]
            } else {
                Highlight::Normal
            };

            if in_string.is_none()
                && c == '/'
                && i < self.chars.len() - 1
                && &self.chars[i..i + 2] == "//"
            {
                for j in i..self.chars.len() {
                    self.hl[j] = Highlight::Comment;
                }
                break;
            }

            if flags & highlight::STRINGS != 0 {
                if let Some(cur) = in_string {
                    self.hl[i] = Highlight::String;
                    if c == '\\' && i + 1 < self.chars.len() {
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

            if (flags & highlight::NUMBERS) != 0
                && ((c.is_digit(10) && (prev_sep || prev_hl == Highlight::Number))
                    || (c == '.' && prev_hl == Highlight::Number))
            {
                self.hl[i] = Highlight::Number;
                prev_sep = false;
                continue;
            }

            prev_sep = c.is_separator();
        }
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
