use std::fmt::Display;

#[derive(PartialEq, Clone)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Light => write!(f, "light"),
            Self::Dark => write!(f, "dark"),
        }
    }
}

pub const GREEN: &'static str = "#00cc00";
pub const YELLOW: &'static str = "#ffff57";
pub const AMBER: &'static str = "#ffbf00";
pub const RED: &'static str = "#eb0000";
