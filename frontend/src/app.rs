use yew::prelude::*;
use yew_router::BrowserRouter;

use crate::components::Header;
use crate::routes;
use crate::theme::ThemeMode;

#[function_component(App)]
pub fn app() -> Html {
    let selected_theme_handle = use_state(|| ThemeMode::Dark);

    html! {
        <BrowserRouter>
            //<ContextProvider<Theme> context={theme.clone()}>
                <div id="content" data-theme={(*selected_theme_handle).clone().to_string()}>
                    <Header {selected_theme_handle} />
                    <div id="page-content" class={classes!("container")}>
                        <routes::PageSwitch />
                    </div>
                </div>
            //</ContextProvider<Theme>>
        </BrowserRouter>
    }
}
