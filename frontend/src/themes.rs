use once_cell::sync::Lazy;

pub static LIGHT: Lazy<Theme> = Lazy::new(|| Theme {
    global: GlobalThemeOpts {
        background_colour: "#f8f9fa".to_string(),
        colour: "#212529".to_string(),
        table_class: "".to_string(),
    },
    components: ComponentThemeOpts {
        header: HeaderThemeOpts {
            background_colour: "#0d6efd".to_string(),
            colour: "#ffffff".to_string(),
            theme_toggle_hover_background_colour: "theme-toggle-light".to_string(),
        },
    },
});

pub static DARK: Lazy<Theme> = Lazy::new(|| Theme {
    global: GlobalThemeOpts {
        background_colour: "#121212".to_string(),
        colour: "#ffffff".to_string(),
        table_class: "table-dark".to_string(),
    },
    components: ComponentThemeOpts {
        header: HeaderThemeOpts {
            background_colour: "#242526".to_string(),
            colour: "#ffffff".to_string(),
            theme_toggle_hover_background_colour: "theme-toggle-dark".to_string(),
        },
    },
});

#[derive(PartialEq, Clone)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GlobalThemeOpts {
    pub background_colour: String,
    pub colour: String,
    pub table_class: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentThemeOpts {
    pub header: HeaderThemeOpts,
}

#[derive(Clone, PartialEq, Debug)]
pub struct HeaderThemeOpts {
    pub background_colour: String,
    pub colour: String,
    pub theme_toggle_hover_background_colour: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Theme {
    pub global: GlobalThemeOpts,
    pub components: ComponentThemeOpts,
}
