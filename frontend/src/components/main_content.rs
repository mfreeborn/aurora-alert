use gloo_console as console;
use yew::prelude::*;
use yew_hooks::use_async;

use crate::services::subscribe;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    let state = use_async(async move {
        let level = subscribe().await;
        if let Ok(lev) = &level {
            console::log!("ok");
            console::log!(lev.clone());
        } else {
            console::log!("Fail");
        };
        level
    });

    let onclick = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // You can trigger to run in callback or use_effect.
            state.run();
        })
    };

    html! {
        <div class="row">
            <div class="col">
                <form>
                    <label for="email">{ "Email" }</label>
                    <input type="text" />
                    <button {onclick} class="btn btn-primary">{ "Submit" }</button>
                </form>
            </div>
            <div class="col">
                <p>{state.loading}</p>
                {
                    if let Some(data) = &state.data {
                        html!{<p>{data}</p>}
                    } else {
                        html!{}
                    }
                }
            </div>
        </div>
    }
}
