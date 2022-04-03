use yew::prelude::*;

use crate::hooks::use_query_params;
use crate::routes::{LinkHome, RedirectInternalServerError, RedirectNotFound};
use crate::services::user::verify;
use crate::types::user::{VerifyUserParams, VerifyUserWrapper};

#[function_component(Verify)]
pub fn verify() -> Html {
    let params: VerifyUserParams = match use_query_params() {
        Ok(params) => params,
        Err(e) => {
            log::warn!("Required parameters not provided: {e}");
            return html! { <RedirectNotFound />};
        }
    };

    let state = {
        yew_hooks::use_async_with_options(
            async move { verify::<VerifyUserWrapper>(params.user_id, params.email).await },
            yew_hooks::UseAsyncOptions::enable_auto(),
        )
    };

    {
        let state = state.clone();
        if let Some(e) = &state.error {
            log::warn!("Error within verify callback: {e}");
            return html! { <RedirectInternalServerError /> };
        }
    }
    html! {
        {
            if state.data.is_some() {
                html! {
                <div>
                    <p>{"Thank you for using Aurora Alert, you will now receive email notifications when the alert level reaches a certain threshold."}</p>
                    <p>
                        <span>{"Head over the the "}</span>
                        <LinkHome text={"homepage"} />
                        <span>{" to see the current aurora alert level."}</span>
                    </p>
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
