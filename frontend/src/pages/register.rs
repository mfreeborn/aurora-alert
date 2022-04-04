use yew::prelude::*;

use crate::components::forms::RegistrationForm;

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <>
            <div class="row mb-5">
                <div class="col">
                    <p>{"Fill in the form to receive notifications when the aurora alert level reaches a chosen threshold. Select up to 5 locations near to you to receive a brief weather status, including cloud cover, with each alert. This should help give you an idea where the best place is to go and try and see the aurora, or whether there is any chance at all if the cloud cover is too extensive."}</p>
                    <p>{"If you have registered in the past, then repeating the process will update your preferences to the newly selected alert threshold and locations."}</p>
                </div>
            </div>
            <div class={classes!("row", "justify-content-center")}>
                <div class="col-auto">
                    <RegistrationForm />
                </div>
            </div>
        </>
    }
}
