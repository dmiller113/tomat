use cursive::theme::{load_theme_file, Error as ThemeError, Theme};
use dirs::config_dir;
use std::io::{Error, ErrorKind};

pub struct Config {
    pub duration: u32,
    pub theme: Theme,
}

pub fn get_configuration() -> Result<Config, ThemeError> {
    config_dir()
        .ok_or(ThemeError::Io(Error::new(
            ErrorKind::NotFound,
            "No config directory"
        )))
        .map(|path| path.as_path().join("tomat/theme.toml"))
        .and_then(|path| load_theme_file(path))
        .map(|theme| Config { duration: 15, theme:  theme})
}
