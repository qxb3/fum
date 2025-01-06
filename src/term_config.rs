use crate::config::Config;

pub struct TermConfig {
    pub width: u16,
    pub height: u16
}

impl TermConfig {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height
        }
    }

    pub fn from_config(config: &Config) -> Self {
        match config.layout.as_str() {
            "top-to-bottom" | "bottom-to-top" => Self::new(20, 15),
            "left-to-right" | "right-to-left" => Self::new(40, 10),
            _ => unreachable!()
        }
    }
}
