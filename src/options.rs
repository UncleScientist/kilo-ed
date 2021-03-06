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
            _ => LineNumbers::default(),
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
//     - Auto-Indentation -
// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Indentation {
    On,
    Off,
}

impl From<String> for Indentation {
    fn from(s: String) -> Self {
        match s.as_str() {
            "on" => Indentation::On,
            _ => Indentation::default(),
        }
    }
}

impl Default for Indentation {
    fn default() -> Self {
        Indentation::Off
    }
}

impl ConvertOptString for Indentation {}

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
            _ => LineDisplay::default(),
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
use config::Config;

#[derive(Debug, Copy, Clone, Default)]
pub struct Options {
    pub lines: LineNumbers,
    pub soft_wrap: LineDisplay,
    pub auto_indent: Indentation,
}

impl Options {
    pub fn new(config: &Config) -> Self {
        let lines = read_config_parameter::<LineNumbers>(config, "display", "line_numbers");
        let soft_wrap = read_config_parameter::<LineDisplay>(config, "display", "soft_wrap");
        let auto_indent = read_config_parameter::<Indentation>(config, "display", "auto_indent");

        Self {
            lines,
            soft_wrap,
            auto_indent,
        }
    }

    pub fn soft_wrap(&self) -> bool {
        self.soft_wrap == LineDisplay::Wrap
    }
}

pub trait ConvertOptString: From<String> + Default + core::fmt::Debug {}

fn read_config_parameter<T: ConvertOptString + From<String>>(
    config: &Config,
    table: &str,
    key: &str,
) -> T {
    let table = if let Ok(table) = config.get_table(table) {
        table
    } else {
        return T::default();
    };

    let value = if let Some(value) = table.get(key) {
        value
    } else {
        return T::default();
    };

    if let Ok(value) = value.clone().into_string() {
        value.into()
    } else {
        T::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use config::{Config, File, FileFormat};

    #[test]
    fn config_reads_empty_file() {
        let config = Config::builder()
            .add_source(File::new("tests/empty-config-file", FileFormat::Ini))
            .build()
            .expect("failed to build config");
        let options = Options::new(&config);
        assert_eq!(options.lines, LineNumbers::Off);
        assert_eq!(options.soft_wrap, LineDisplay::Scroll);
    }

    #[test]
    fn config_can_set_absolute_line_numbers() {
        let config = Config::builder()
            .add_source(File::new("tests/line-numbers-absolute", FileFormat::Ini))
            .build()
            .expect("failed to build config");
        let options = Options::new(&config);
        assert_eq!(options.lines, LineNumbers::Absolute);
        assert_eq!(options.soft_wrap, LineDisplay::Scroll);
    }

    #[test]
    fn config_can_set_auto_indent() {
        let config = Config::builder()
            .add_source(File::new("tests/auto-indent-on", FileFormat::Ini))
            .build()
            .expect("failed to build config");
        let options = Options::new(&config);
        assert_eq!(options.auto_indent, Indentation::On);
    }
}
