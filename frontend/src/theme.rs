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
