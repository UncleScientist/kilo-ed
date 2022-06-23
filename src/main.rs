use std::path::Path;

use config::{Config, ConfigError, File, FileFormat};
use crossterm::Result;
use xdg::BaseDirectories;

mod editor_syntax;
mod keyboard;
mod row;
mod screen;

mod editor;
use editor::*;

fn main() -> Result<()> {
    let mut args = std::env::args();
    let config_file = BaseDirectories::with_prefix("kilo-ed")?.find_config_file("init");

    let config = if let Some(config_file) = config_file {
        match load_config(&config_file) {
            Ok(config) => config,
            Err(_) => {
                eprintln!("unable to load config file, using defaults");
                default_config()
            }
        }
    } else {
        default_config()
    };

    let mut editor = if args.len() >= 2 {
        Editor::with_file(config, args.nth(1).unwrap())?
    } else {
        Editor::new(config)?
    };

    editor.start()?;

    Ok(())
}

fn load_config(path: &Path) -> core::result::Result<Config, ConfigError> {
    Config::builder()
        .set_default("line_numbers", "relative")?
        .add_source(File::new(path.to_str().unwrap(), FileFormat::Ini))
        .build()
}

fn default_config() -> Config {
    Config::builder()
        .set_default("line_numbers", "relative")
        .expect("oops")
        .build()
        .expect("oops again")
}
