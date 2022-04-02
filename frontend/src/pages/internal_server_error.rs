use yew::prelude::*;

use crate::routes::LinkHome;

#[function_component(InternalServerError)]
pub fn internal_server_error() -> Html {
    html! {
        <>
            <h2 style="display: inline;">
                { "Internal server error... " }
            </h2>
            <h3 style="display: inline;">
                <LinkHome text={"return to the homepage"} />
            </h3>
        </>
    }
}
