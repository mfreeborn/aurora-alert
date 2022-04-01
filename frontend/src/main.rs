use aurora_alert_frontend::components::header::Header;
use aurora_alert_frontend::components::main_content::MainContent;
use aurora_alert_frontend::routes;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <Header />

        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
