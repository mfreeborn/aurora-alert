use yew::prelude::*;

use crate::components::forms::RegistrationForm;

#[function_component(Home)]
pub fn home() -> Html {
    log::info!("render home page");

    html! {
        <>
            <h1>{ "Home" }</h1>
            <RegistrationForm />
        </>
    }
}
