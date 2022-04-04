use yew::prelude::*;

use crate::routes::{LinkHome, LinkRegister};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-light bg-light">
            <div class="container-fluid">
                <LinkHome classes="navbar-brand" text="Aurora Alert" />
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbar-nav-collapse">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbar-nav-collapse">
                    <div class="navbar-nav">
                        <LinkRegister classes="nav-link" text="Register" />
                    </div>
                </div>
            </div>
        </nav>
    }
}
