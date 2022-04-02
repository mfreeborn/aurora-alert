use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::components::Link;

use crate::requests;
use crate::routes::Route;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UnsubscribeUserWrapper {
    pub message: String,
}

#[derive(Properties, PartialEq)]
pub struct UnsubscribeProps {
    pub user_id: String,
    pub email: String,
}

#[function_component(Unsubscribe)]
pub fn unsubscribe(props: &UnsubscribeProps) -> Html {
    let UnsubscribeProps { user_id, email } = props;

    let state = {
        let user_id = user_id.clone();
        let email = email.clone();

        yew_hooks::use_async(async move {
            requests::get::<UnsubscribeUserWrapper>(format!(
                "/unsubscribe?user_id={}&email={}",
                user_id, email
            ))
            .await
        })
    };

    {
        let state = state.clone();
        yew_hooks::use_effect_once(move || {
            {
                state.run()
            }
            || log::info!("hi")
        });
    }

    html! {
        <>
            {
                if state.loading {
                    html!{<div>{"hi!"}</div>}
                } else {
                    html!{ <div>{"there"}</div> }
                }
            }
        <div>
            <p>{"Thank you for using Aurora Alert, you have now been subscribed and will no longer receive email alerts"}</p>
            <Link<Route> to={Route::Home}>{"Return to the homepage"}</Link<Route>>
        </div></>
    }
}
