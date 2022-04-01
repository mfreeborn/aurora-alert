use aurora_alert_frontend::components::header::Header;
use aurora_alert_frontend::components::main_content::MainContent;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <Header />
            <MainContent />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
