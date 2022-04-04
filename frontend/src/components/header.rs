use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{LinkHome, Route};

#[derive(Properties, PartialEq, Clone)]
struct NavLinkProps {
    to: Route,
    text: String,
    current_route: Route,
}

#[function_component(NavLink)]
fn nav_link(props: &NavLinkProps) -> Html {
    let NavLinkProps {
        to,
        text,
        current_route,
    } = (*props).clone();

    let active = if to == current_route { "active" } else { "" };
    html! {
        <Link<Route> {to} classes={classes!("nav-link", active)}>{text}</Link<Route>>
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    let current_route: Route = use_route().unwrap_or_default();

    html! {
        <nav class="navbar navbar-expand-lg navbar-light bg-light mb-5">
            <div class="container-fluid">
                <LinkHome classes="navbar-brand" text="Aurora Alert" />
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbar-nav-collapse">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbar-nav-collapse">
                    <div class="navbar-nav">
                        <NavLink to={Route::Register} text={"Register".to_string()} current_route={current_route.clone()} />
                        <NavLink to={Route::About} text={"About".to_string()} current_route={current_route.clone()} />
                    </div>
                </div>
            </div>
        </nav>
    }
}
