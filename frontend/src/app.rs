use yew::prelude::*;
use yew_router::BrowserRouter;

use crate::components::Header;
use crate::routes;

#[derive(PartialEq, Clone)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GlobalThemeOpts {
    pub background_colour: String,
    pub colour: String,
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

#[function_component(App)]
pub fn app() -> Html {
    let light: Theme = Theme {
        global: GlobalThemeOpts {
            background_colour: "#f8f9fa".to_string(),
            colour: "#212529".to_string(),
        },
        components: ComponentThemeOpts {
            header: HeaderThemeOpts {
                background_colour: "#0d6efd".to_string(),
                colour: "#ffffff".to_string(),
                theme_toggle_hover_background_colour: "theme-toggle-light".to_string(),
            },
        },
    };

    let dark: Theme = Theme {
        global: GlobalThemeOpts {
            background_colour: "#121212".to_string(),
            colour: "#ffffff".to_string(),
        },
        components: ComponentThemeOpts {
            header: HeaderThemeOpts {
                background_colour: "#242526".to_string(),
                colour: "#ffffff".to_string(),
                theme_toggle_hover_background_colour: "theme-toggle-dark".to_string(),
            },
        },
    };

    let selected_theme_handle = use_state(|| ThemeMode::Light);
    let theme = match (*selected_theme_handle).clone() {
        ThemeMode::Light => light,
        ThemeMode::Dark => dark,
    };

    log::info!("{:?}", theme);

    html! {
        <BrowserRouter>
            <ContextProvider<Theme> context={theme.clone()}>
                <div id="content" style={
                        format!(
                            "font-family: roboto; min-height: 100%; min-width: 100%; background-color: {}; color: {};",
                            theme.global.background_colour, theme.global.colour
                        )}>
                    <Header {selected_theme_handle} />
                    <div id="page-content" class={classes!("container")}>
                        <routes::PageSwitch />
                    </div>
                </div>
            </ContextProvider<Theme>>
        </BrowserRouter>
    }
}
