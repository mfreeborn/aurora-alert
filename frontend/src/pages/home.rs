use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    log::info!("render home page");

    html! {
        <>
            <h1>{ "Home" }</h1>
        </>
    }
}
