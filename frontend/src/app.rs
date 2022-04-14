use yew::prelude::*;
use yew_router::BrowserRouter;

use crate::components::Header;
use crate::routes;
use crate::themes::{Theme, ThemeMode, DARK, LIGHT};

#[function_component(App)]
pub fn app() -> Html {
    let selected_theme_handle = use_state(|| ThemeMode::Light);
    let theme = match (*selected_theme_handle).clone() {
        ThemeMode::Light => &*LIGHT,
        ThemeMode::Dark => &*DARK,
    };

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
