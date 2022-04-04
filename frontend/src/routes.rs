use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{About, Home, InternalServerError, PageNotFound, Register, Unsubscribe, Verify};

#[derive(Clone, PartialEq, Routable, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/register")]
    Register,
    #[at("/about")]
    About,
    #[at("/verify")]
    Verify,
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
        Route::Register => html! { <Register /> },
        Route::About => html! { <About /> },
        Route::Verify => html! { <Verify /> },
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
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(LinkHome)]
pub fn link_home(props: &LinkHomeProps) -> Html {
    let LinkHomeProps { text, classes } = props;
    html! {
        <Link<Route> to={Route::Home}  classes={classes.clone()}>{text}</Link<Route>>
    }
}

#[derive(Properties, PartialEq)]
pub struct LinkRegisterProps {
    pub text: String,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(LinkRegister)]
pub fn link_register(props: &LinkRegisterProps) -> Html {
    let LinkRegisterProps { text, classes } = props;
    html! {
        <Link<Route> to={Route::Register}  classes={classes.clone()}>{text}</Link<Route>>
    }
}

#[derive(Properties, PartialEq)]
pub struct LinkAboutProps {
    pub text: String,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(LinkAbout)]
pub fn link_about(props: &LinkAboutProps) -> Html {
    let LinkAboutProps { text, classes } = props;
    html! {
        <Link<Route> to={Route::Register}  classes={classes.clone()}>{text}</Link<Route>>
    }
}
