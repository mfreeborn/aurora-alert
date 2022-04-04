use yew::prelude::*;

use crate::components::forms::RegistrationForm;

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <>
            <RegistrationForm />
        </>
    }
}
