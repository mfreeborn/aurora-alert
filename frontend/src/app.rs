use yew::prelude::*;
use yew_router::BrowserRouter;

use crate::components::Header;
use crate::routes;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <BrowserRouter>
                <Header />
                <div class={classes!("page-content")}>
                    <routes::Switch />
                </div>
            </BrowserRouter>
        </div>
    }
}
