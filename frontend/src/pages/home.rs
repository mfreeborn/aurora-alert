use yew::prelude::*;

use crate::components::chart::ActivityChart;

#[function_component(Home)]
pub fn home() -> Html {
    log::debug!("render home page");

    html! {
        <div class="row">
            <div class="col">
                <ActivityChart />
            </div>
        </div>
    }
}
