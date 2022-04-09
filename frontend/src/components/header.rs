use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{LinkHome, Route};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub selected_theme_handle: UseStateHandle<crate::app::ThemeMode>,
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
                crate::app::ThemeMode::Dark => crate::app::ThemeMode::Light,
                crate::app::ThemeMode::Light => crate::app::ThemeMode::Dark,
            };
            selected_theme_handle.set(new_theme);
        })
    };

    let theme = use_context::<crate::app::Theme>().expect("No theme found");

    html! {
        <nav class="navbar navbar-expand-lg navbar-dark mb-5" style={format!("background-color: {}", theme.components.header.background_colour)}>
            <div class="container-fluid align-items-center">
                <LinkHome classes="navbar-brand" text="Aurora Alert" />
                <div class="d-flex order-lg-last col justify-content-end">
                    <button onclick={onclick_toggle_theme} data-bs-toggle="tooltip" data-bs-placement="bottom" title="Toggle between light and dark theme" class={classes!("btn", "btn-outline", "d-flex", "align-items-center", "me-2", "p-1", "rounded-circle", "border-0", "shadow-none", theme.components.header.theme_toggle_hover_background_colour)}>
                    {
                        match *selected_theme_handle.clone() {
                            crate::app::ThemeMode::Light => html ! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill={format!("{}", theme.components.header.colour)} class="bi bi-brightness-high" viewBox="0 0 16 16">
                                    <path d="M8 11a3 3 0 1 1 0-6 3 3 0 0 1 0 6zm0 1a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM8 0a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 0zm0 13a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 13zm8-5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2a.5.5 0 0 1 .5.5zM3 8a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2A.5.5 0 0 1 3 8zm10.657-5.657a.5.5 0 0 1 0 .707l-1.414 1.415a.5.5 0 1 1-.707-.708l1.414-1.414a.5.5 0 0 1 .707 0zm-9.193 9.193a.5.5 0 0 1 0 .707L3.05 13.657a.5.5 0 0 1-.707-.707l1.414-1.414a.5.5 0 0 1 .707 0zm9.193 2.121a.5.5 0 0 1-.707 0l-1.414-1.414a.5.5 0 0 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .707zM4.464 4.465a.5.5 0 0 1-.707 0L2.343 3.05a.5.5 0 1 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .708z"/>
                                </svg>
                            },
                            crate::app::ThemeMode::Dark => html ! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill={format!("{}", theme.components.header.colour)} class="bi bi-moon" viewBox="0 0 16 16">
                                    <path d="M6 .278a.768.768 0 0 1 .08.858 7.208 7.208 0 0 0-.878 3.46c0 4.021 3.278 7.277 7.318 7.277.527 0 1.04-.055 1.533-.16a.787.787 0 0 1 .81.316.733.733 0 0 1-.031.893A8.349 8.349 0 0 1 8.344 16C3.734 16 0 12.286 0 7.71 0 4.266 2.114 1.312 5.124.06A.752.752 0 0 1 6 .278zM4.858 1.311A7.269 7.269 0 0 0 1.025 7.71c0 4.02 3.279 7.276 7.319 7.276a7.316 7.316 0 0 0 5.205-2.162c-.337.042-.68.063-1.029.063-4.61 0-8.343-3.714-8.343-8.29 0-1.167.242-2.278.681-3.286z"/>
                                </svg>
                            }
                        }
                    }
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
