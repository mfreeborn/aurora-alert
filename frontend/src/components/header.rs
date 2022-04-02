use yew::prelude::*;
use yew_router::components::Link;

use crate::routes::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="row header">
            <div class="col-auto">
                <Link<Route> to={Route::Home}>{ "Aurora Alert" }</Link<Route>>
            </div>
            <div class="col-auto">
                <span>{ "2022" }</span>
            </div>
        </div>
    }
}
