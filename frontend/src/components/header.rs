use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{LinkHome, Route};
use crate::theme::ThemeMode;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub selected_theme_handle: UseStateHandle<ThemeMode>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let HeaderProps {
        selected_theme_handle,
    } = props;

    let current_route: Route = use_route().unwrap_or_default();

    let onclick_toggle_theme = {
        let selected_theme_handle = selected_theme_handle.clone();
        Callback::from(move |_: MouseEvent| {
            let current_theme = (*selected_theme_handle).clone();
            let new_theme = match current_theme {
                ThemeMode::Dark => ThemeMode::Light,
                ThemeMode::Light => ThemeMode::Dark,
            };
            selected_theme_handle.set(new_theme);
        })
    };

    // let theme = use_context::<Theme>().expect("No theme found");

    html! {
        // <nav class="navbar navbar-expand-lg navbar-dark mb-5" style={format!("background-color: {}", theme.components.header.background_colour)}>
        <nav class="navbar navbar-expand-lg mb-5">
            <div class="container-fluid">
                <LinkHome classes="navbar-brand" text="Aurora Alert" />
                <div class="d-flex order-lg-last col justify-content-end">
                    <button onclick={onclick_toggle_theme} data-bs-toggle="tooltip" data-bs-placement="bottom" title="Toggle between light and dark theme" class={classes!("btn", "btn-outline", "d-flex", "align-items-center", "me-2", "p-1", "rounded-circle", "border-0", "shadow-none", "theme-toggler")}>
                        <span class="theme-toggler-img"></span>
                    </button>
                </div>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbar-nav-collapse">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbar-nav-collapse">
                    <div class="navbar-nav">
                        <NavLink to={Route::Register} text={"Register"} current_route={current_route.clone()} />
                        <NavLink to={Route::About} text={"About"} current_route={current_route.clone()} />
                    </div>
                </div>
            </div>
        </nav>
    }
}

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
