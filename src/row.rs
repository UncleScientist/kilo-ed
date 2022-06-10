use crossterm::style::Color;

const KILO_TAB_STOP: usize = 8;

#[derive(Copy, Clone, PartialEq)]
pub enum Highlight {
    Normal,
    Number,
}

impl Highlight {
    pub fn syntax_to_color(&self) -> Color {
        match self {
            Highlight::Normal => Color::White,
            Highlight::Number => Color::Red,
        }
    }
}

pub struct Row {
    pub chars: String,
    pub render: String,
    pub hl: Vec<Highlight>,
}

impl Row {
    pub fn new(chars: String) -> Self {
        let mut result = Self {
            chars,
            render: String::new(),
            hl: Vec::new(),
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
        self.update_syntax();
    }

    fn update_syntax(&mut self) {
        self.hl = vec![Highlight::Normal; self.render.len()];

        for (i, c) in self.render.chars().enumerate() {
            if c.is_digit(10) {
                self.hl[i] = Highlight::Number;
            }
        }
    }
}
