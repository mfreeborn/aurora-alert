use yew::prelude::*;

use crate::components::forms::RegistrationForm;

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <>
            <div class="row mb-5 justify-content-center">
                <div class="col-lg-10">
                    <p>{"Fill in the form to receive notifications when the aurora alert level reaches a chosen threshold. Select up to 5 locations near to you to receive a brief weather status, including cloud cover, with each alert. This should help give you an idea where the best place is to go and try and see the aurora, or whether there is any chance at all if the cloud cover is too extensive."}</p>
                    <p>{"If you have registered in the past and wish to change your settings, first unsubscribe before re-registering."}</p>
                </div>
            </div>
            <div class={classes!("row", "justify-content-center")}>
                <div class="col-lg-6">
                    <RegistrationForm />
                </div>
            </div>
        </>
    }
}
