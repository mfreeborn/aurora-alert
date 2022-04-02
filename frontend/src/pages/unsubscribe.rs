use yew::prelude::*;

use crate::hooks::use_query_params;
use crate::routes::{LinkHome, RedirectNotFound};
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
        let params = params.clone();
        yew_hooks::use_async_with_options(
            async move { unsubscribe::<UnsubscribeUserWrapper>(params.user_id, params.email).await },
            yew_hooks::UseAsyncOptions::enable_auto(),
        )
    };

    html! {
        <>
            {
                if state.loading {
                    html! {<div>{"Loading..."}</div>}
                } else {
                    html! { <div></div> }
                }
            }
            {
                if let Some(_) = &state.data {
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
            {
                if let Some(_) = &state.error {
                    html! {
                        <RedirectNotFound />
                    }
                } else {
                    html! {
                        <div></div>
                    }
                }
            }
        </>
    }
}
