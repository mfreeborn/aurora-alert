use std::collections::HashMap;
use std::ops::Deref;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

use super::Form;
use crate::services::locations::get_locations;
use crate::services::user::register;
use crate::types::user::{UserRegisterPostBody, UserSubscribeWrapper};

#[function_component(RegistrationForm)]
pub fn registration_form() -> Html {
    let email_handler = use_state(String::new);
    let alert_threshold_handler = use_state(|| "yellow".to_string());
    let locations_handler = use_state(HashMap::<String, i64>::new);

    let registration_info = UserRegisterPostBody {
        email: email_handler.deref().clone(),
        alert_threshold: alert_threshold_handler.deref().clone(),
        locations: locations_handler
            .deref()
            .clone()
            .values()
            .cloned()
            .collect::<Vec<_>>(),
    };

    let user_register = {
        let registration_info = registration_info.clone();
        use_async(async move { register::<UserSubscribeWrapper>(registration_info).await })
    };

    let onsubmit = {
        let user_register = user_register;
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            user_register.run();
        })
    };

    let valid_form = registration_info.is_valid();
    html! {
        <Form {onsubmit}>
            <EmailField handler={email_handler} />
            <AlertThresholdField handler={alert_threshold_handler} />
            <LocationsField handler={locations_handler} />
            <button disabled={!valid_form} type="submit" class={classes!("btn", "btn-primary")}>{"Register"}</button>
        </Form>
    }
}

#[derive(Properties, PartialEq)]
struct EmailFieldProps {
    handler: UseStateHandle<String>,
}

#[function_component(EmailField)]
fn email_field(props: &EmailFieldProps) -> Html {
    let oninput = {
        let handler = props.handler.clone();
        Callback::from(move |e: InputEvent| {
            let el: HtmlInputElement = e.target_unchecked_into();
            handler.set(el.value());
        })
    };
    html! {
        <div class={classes!("mb-3")}>
            <label for="user-email" class={classes!("form-label")}>{"Email address"}</label>
            <input {oninput} id="user-email" type="email" class={classes!("form-control")} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct AlertThresholdFieldProps {
    handler: UseStateHandle<String>,
}

#[function_component(AlertThresholdField)]
fn alert_threshold_field(props: &AlertThresholdFieldProps) -> Html {
    let oninput = {
        let handler = props.handler.clone();
        Callback::from(move |e: InputEvent| {
            let el: HtmlInputElement = e.target_unchecked_into();
            handler.set(el.value());
        })
    };
    html! {
        <div class={classes!("mb-3")}>
            <label for="user-alert-threshold" class={classes!("form-label")}>{"Alert threshold"}</label>
            <select {oninput} id="user-alert-threshold" class={classes!("form-select")}>
                <option value="yellow" selected=true>{"Yellow"}</option>
                <option value="amber">{"Amber"}</option>
                <option value="red">{"Red"}</option>
            </select>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LocationsFieldProps {
    handler: UseStateHandle<HashMap<String, i64>>,
}

#[function_component(LocationsField)]
fn locations_field(props: &LocationsFieldProps) -> Html {
    let chosen_locations = props.handler.clone();

    let location = use_state(String::new);

    let datalist_locations = {
        let location = location.clone();
        use_async(async move { get_locations(location.deref().clone()).await })
    };

    let oninput_location = {
        let location = location.clone();
        let datalist_locations = datalist_locations.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val = input.value();
            location.set(val);

            datalist_locations.run();
        })
    };

    let onclick = {
        let chosen_locations = chosen_locations.clone();
        let datalist_locations = datalist_locations.clone();
        let location = location.clone();
        Callback::from(move |_: MouseEvent| {
            let (name, location_id) = datalist_locations
                .data
                .as_ref()
                .unwrap()
                .get_key_value(&*location)
                .unwrap();

            let mut new_chosen_locations = chosen_locations.deref().clone();
            new_chosen_locations.insert(name.to_string(), *location_id);
            chosen_locations.set(new_chosen_locations);
        })
    };

    let valid_location = if let Some(locations) = &datalist_locations.data {
        locations.contains_key(&*location)
    } else {
        false
    };

    html! {
        <>
            <label for="location-list" class="form-label">{"Locations"}</label>
            <div class={classes!("mb-3", "input-group")}>
                // we could easily add a country filter later on if we want to
                // <select class="form-select">
                //     <option value="GB" selected={true} style="max-width: max-content;">{"GB"}</option>
                // </select>
                <input oninput={oninput_location} id="location-list" class="form-control" list="location-list-options" Placeholder="Search for a location..." />
                <button {onclick} class="btn btn-primary" style="border-top-right-radius: 0.25rem; border-bottom-right-radius: 0.25rem;" type="button" disabled={!valid_location}>{"Select"}</button>
                <datalist id="location-list-options">
                    {
                        datalist_locations.data
                            .iter()
                            .flatten()
                            .map(|(name, id)| html! {<option data-value={id.to_string()} value={name.clone()} />})
                            .collect::<Html>()
                    }
                </datalist>
            </div>
            <ChosenLocations locations={chosen_locations} />
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ChosenLocationProps {
    name: String,
    location_to_remove: UseStateHandle<Option<String>>,
}

#[function_component(ChosenLocation)]
fn chosen_location(props: &ChosenLocationProps) -> Html {
    let name = props.name.clone();
    let onclick = {
        let name = props.name.clone();
        let handle = props.location_to_remove.clone();
        Callback::from(move |_: MouseEvent| {
            handle.set(Some(name.clone()));
        })
    };

    html! {
        <li class={classes!("list-group-item", "d-flex", "justify-content-between", "align-items-center")}>
            <span>{name}</span>
            <svg {onclick} xmlns="http://www.w3.org/2000/svg"  class={classes!("bi", "bi-x-circle", "icon-button")} viewBox="0 0 16 16" style="border-radius: 50%;">
                <path  d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
                <path  d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
            </svg>
        </li>
    }
}

#[derive(Properties, PartialEq)]
struct ChosenLocationsProps {
    locations: UseStateHandle<HashMap<String, i64>>,
}

#[function_component(ChosenLocations)]
fn chosen_locations(props: &ChosenLocationsProps) -> Html {
    let location_to_remove = use_state(|| None::<String>);

    if let Some(location) = location_to_remove.deref().clone() {
        let mut new_locations = props.locations.deref().clone();
        new_locations.remove(&location);
        props.locations.set(new_locations);
        location_to_remove.set(None);
    }

    html! {
        <ul class={classes!("list-group")} style="user-select: none;">
            {
                props.locations.iter().map(|(name, _)| html! {
                    <ChosenLocation name={name.clone()} location_to_remove={location_to_remove.clone()} />
                }).collect::<Html>()
            }
        </ul>
    }
}
