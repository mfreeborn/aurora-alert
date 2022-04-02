use yew::{function_component, html, Html};
use yew_router::{Routable, Switch as YewSwitch};

use crate::pages::{Home, PageNotFound, Unsubscribe};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/unsubscribe/:user_id/:email")]
    Unsubscribe { user_id: String, email: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Unsubscribe { user_id, email } => {
            html! { <Unsubscribe user_id={user_id.to_string()} email={email.to_string()} /> }
        }
        Route::NotFound => html! { <PageNotFound /> },
    }
}

#[function_component(Switch)]
pub fn router() -> Html {
    html! {
        <YewSwitch<Route> render={YewSwitch::render(switch)} />
    }
}
