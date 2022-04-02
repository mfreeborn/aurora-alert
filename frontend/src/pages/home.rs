use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

use crate::services::user::register;
use crate::types::user::{UserRegisterInfo, UserSubscribeWrapper};

#[function_component(Home)]
pub fn home() -> Html {
    let register_info = use_state(UserRegisterInfo::default);
    log::info!("{:?}", register_info.clone());

    let user_register = {
        let register_info = register_info.clone();
        use_async(async move {
            let user_info = (*register_info).clone();
            register::<UserSubscribeWrapper>(user_info).await
        })
    };

    let onsubmit = {
        let user_register = user_register.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            user_register.run();
        })
    };

    let oninput_email = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            register_info.set(info);
        })
    };

    let oninput_alert_threshold = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.alert_threshold = input.value();
            register_info.set(info);
        })
    };

    let location = use_state(String::new);
    let oninput_location = {
        let location = location.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val = input.value();
            location.set(val);
        })
    };

    let locations = vec![
        ("Fort William".to_string(), "2649169".to_string()),
        ("Spean Bridge".to_string(), "2649168".to_string()),
    ];

    let valid_location = locations.iter().any(|loc| loc.0 == *location);

    html! {
        <>
            <h1>{ "Home" }</h1>
            <form {onsubmit} autocomplete="off" style="max-width: max-content;">
            <div class={classes!("mb-3")}>
                <label for="user-email" class={classes!("form-label")}>{"Email address"}</label>
                <input id="user-email" type="email" class={classes!("form-control")}  oninput={oninput_email} />
            </div>
            <div class={classes!("mb-3")}>
                <label for="user-alert-threshold" class={classes!("form-label")}>{"Alert threshold"}</label>
                <select oninput={oninput_alert_threshold} id="user-alert-threshold" class={classes!("form-select")}>
                    <option value="yellow" selected=true>{"Yellow"}</option>
                    <option value="amber">{"Amber"}</option>
                    <option value="red">{"Red"}</option>
                </select>
            </div>
            <label for="location-list" class="form-label">{"Locations"}</label>
            <div class={classes!("mb-3", "input-group")}>
                <input oninput={oninput_location} id="location-list" class="form-control" list="location-list-options" Placeholder="Search for a location..." />
                <button class="btn btn-primary" style="border-top-right-radius: 0.25rem; border-bottom-right-radius: 0.25rem;" type="button" disabled={!valid_location}>{"Select"}</button>
                <datalist id="location-list-options">
                    {
                        locations
                            .iter()
                            .map(|loc| html!{<option data-value={loc.1.clone()} value={loc.0.clone()} />})
                            .collect::<Html>()
                    }
                </datalist>
                {(*location).clone()}
            </div>
            <button type="submit" class={classes!("btn", "btn-primary")}>{"Register"}</button>
            </form>
        </>
    }
}
