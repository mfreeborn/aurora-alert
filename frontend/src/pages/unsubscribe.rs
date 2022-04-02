use yew::prelude::*;

use crate::hooks::use_query_params;
use crate::routes::{LinkHome, RedirectInternalServerError, RedirectNotFound};
use crate::services::user::unsubscribe;
use crate::types::user::{UnsubscribeParams, UnsubscribeUserWrapper};

#[function_component(Unsubscribe)]
pub fn unsubscribe() -> Html {
    let params: UnsubscribeParams = match use_query_params() {
        Ok(params) => params,
        Err(e) => {
            log::warn!("Required parameters not provided: {e}");
            return html! { <RedirectNotFound />};
        }
    };

    let state = {
        //let params = params.clone();
        yew_hooks::use_async_with_options(
            async move { unsubscribe::<UnsubscribeUserWrapper>(params.user_id, params.email).await },
            yew_hooks::UseAsyncOptions::enable_auto(),
        )
    };

    {
        let state = state.clone();
        if let Some(e) = &state.error {
            log::warn!("Error within unsubscribe callback: {e}");
            return html! { <RedirectInternalServerError /> };
        }
    }
    html! {
        {
            if state.data.is_some() {
                html! {
                <div>
                    <p>{"Thank you for using Aurora Alert, you have now been unsubscribed and will no longer receive email alerts"}</p>
                    <LinkHome text={"Return to the homepage"} />
                </div>
                }
            } else {
                html! {
                    <div></div>
                }
            }
        }
    }
}
