use web_sys::{HtmlInputElement, SvgElement};
use yew::prelude::*;
use yew_hooks::use_async;

use crate::services::locations::get_locations;
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

            register::<UserSubscribeWrapper>(user_info.into()).await
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
            info.email = Some(input.value());
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

    let get_locations = {
        let location = location.clone();
        use_async(async move { get_locations((*location).clone()).await })
    };

    let oninput_location = {
        let location = location.clone();
        let get_locations = get_locations.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val = input.value();
            location.set(val.clone());

            get_locations.run();
        })
    };

    let onclick_location = {
        let location = location.clone();
        let register_info = register_info.clone();
        let get_locations = get_locations.clone();
        Callback::from(move |_: MouseEvent| {
            log::info!("{}", *location);
            let location_id = get_locations
                .data
                .as_ref()
                .unwrap()
                .get(&*location)
                .unwrap()
                .to_owned();
            let mut info = (*register_info).clone();
            info.locations.insert((&location).to_string(), location_id);
            register_info.set(info);
        })
    };

    if let Some(loc) = &get_locations.data {
        log::info!("{loc:?}");
    }

    let valid_location = {
        if let Some(locs) = &get_locations.data {
            locs.contains_key(&*location)
        } else {
            false
        }
    };

    let onclick_remove_location = {
        let register_info = register_info.clone();
        Callback::from(move |e: MouseEvent| {
            let el: SvgElement = e.target_unchecked_into();
            let location_name = el.id().strip_prefix("delete-").unwrap().to_string();
            let mut info = (*register_info).clone();
            info.locations.remove(&location_name);
            register_info.set(info);
        })
    };

    let valid_form = { register_info.is_valid() };

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
                // we could easily add a country filter later on if we want to
                // <select class="form-select">
                //     <option value="GB" selected={true} style="max-width: max-content;">{"GB"}</option>
                // </select>
                <input oninput={oninput_location} id="location-list" class="form-control" list="location-list-options" Placeholder="Search for a location..." />
                <button onclick={onclick_location} class="btn btn-primary" style="border-top-right-radius: 0.25rem; border-bottom-right-radius: 0.25rem;" type="button" disabled={!valid_location}>{"Select"}</button>
                <datalist id="location-list-options">
                    {
                        get_locations.data
                            .iter()
                            .flatten()
                            .map(|(name, id)| html! {<option data-value={id.to_string()} value={name.clone()} />})
                            .collect::<Html>()
                    }
                </datalist>
                </div>

            <ul class={classes!("list-group")} style="user-select: none;">
                {
                    register_info.locations.iter().map(|(name, _)| html! {
                        <li class={classes!("list-group-item", "d-flex", "justify-content-between", "align-items-center")}>
                        {name.clone()}
                            // really ugly way of tracking which location we are on, so that we can later delete it, but
                            // I simply couldn't get the data-* API to work. Also, currentTarget _always_ seems to return
                            // HtmlBodyElement, so we have to make sure the id is on all child elements, too.
                            <svg id={format!("delete-{}", name)} onclick={onclick_remove_location.clone()} xmlns="http://www.w3.org/2000/svg"  class={classes!("bi", "bi-x-circle", "icon-button")} viewBox="0 0 16 16" style="border-radius: 50%;">
                                <path id={format!("delete-{}", name)} d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
                                <path id={format!("delete-{}", name)} d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                            </svg>
                        </li>
                    }).collect::<Html>()
                }
            </ul>
            <button disabled={!valid_form} type="submit" class={classes!("btn", "btn-primary")}>{"Register"}</button>
            </form>
        </>
    }
}
