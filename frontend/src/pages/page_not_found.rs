use yew::prelude::*;

use crate::routes::LinkHome;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html! {
        <>
            <h2 style="display: inline;">
                { "We can't seem find the page are looking for... " }
            </h2>
            <h3 style="display: inline;">
                <LinkHome text={"return to the homepage"} />
            </h3>
        </>
    }
}
