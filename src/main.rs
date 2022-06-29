use std::collections::HashMap;

use config::{builder::DefaultState, Config, ConfigBuilder, File, FileFormat, Value};
use crossterm::Result;
use xdg::BaseDirectories;

mod editor_syntax;
mod keyboard;
mod options;
mod row;
mod screen;

mod editor;
use editor::*;

fn main() -> Result<()> {
    let mut args = std::env::args();
    let config_file = BaseDirectories::with_prefix("kilo-ed")?.find_config_file("init");
    let config_builder = default_config();

    // If there's a file, add it to the config
    let config_builder = if let Some(config_file) = config_file {
        config_builder.add_source(File::new(config_file.to_str().unwrap(), FileFormat::Ini))
    } else {
        config_builder
    };

    // Attempt to load the config from the defaults + any file found above
    let config = if let Ok(config) = config_builder.build() {
        config
    } else {
        // Failsafe: if the file failed to read, then fall back to the defaults
        default_config().build().unwrap()
    };

    let mut editor = if args.len() >= 2 {
        Editor::with_file(config, args.nth(1).unwrap())?
    } else {
        Editor::new(config)?
    };

    editor.start()?;

    Ok(())
}

fn default_config() -> ConfigBuilder<DefaultState> {
    let display: HashMap<String, Value> = [
        ("line_numbers".to_string(), "relative".into()),
        ("soft_wrap".to_string(), "true".into()),
    ]
    .into_iter()
    .collect();
    Config::builder()
        .set_default("display", display)
        .expect("oops")
}
