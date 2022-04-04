use yew::prelude::*;
use yew_router::BrowserRouter;

use crate::components::Header;
use crate::routes;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <div class={classes!("container")}>
                <routes::PageSwitch />
            </div>
        </BrowserRouter>
    }
}
