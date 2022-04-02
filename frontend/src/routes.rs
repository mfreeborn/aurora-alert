use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home, InternalServerError, PageNotFound, Unsubscribe};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/unsubscribe")]
    Unsubscribe,
    #[at("/internal-server-error")]
    InternalServerError,
    #[not_found]
    #[at("/not-found")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Unsubscribe => {
            html! { <Unsubscribe  /> }
        }
        Route::InternalServerError => html! { <InternalServerError /> },
        Route::NotFound => html! { <PageNotFound /> },
    }
}

#[function_component(PageSwitch)]
pub fn page_switch() -> Html {
    html! {
        <Switch<Route> render={Switch::render(switch)} />
    }
}

#[function_component(RedirectNotFound)]
pub fn redirect_not_found() -> Html {
    html! {
        <Redirect<Route> to={Route::NotFound} />
    }
}

#[function_component(RedirectInternalServerError)]
pub fn redirect_internal_server_error() -> Html {
    html! {
        <Redirect<Route> to={Route::InternalServerError} />
    }
}

#[derive(Properties, PartialEq)]
pub struct LinkHomeProps {
    pub text: String,
}

#[function_component(LinkHome)]
pub fn link_home(props: &LinkHomeProps) -> Html {
    html! {
        <Link<Route> to={Route::Home}>{props.text.clone()}</Link<Route>>
    }
}
