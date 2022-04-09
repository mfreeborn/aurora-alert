use std::collections::HashMap;
use std::ops::Deref;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

use super::Form;
use crate::routes::LinkHome;
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
        let user_register = user_register.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            user_register.run();
        })
    };

    let valid_form = !user_register.loading && registration_info.is_valid();
    html! {
        <>
            {
                if let Some(_data) = &user_register.data {
                    html ! {
                        <div class="row">
                            <div class="col">
                                <div class="alert alert-success d-flex align-items-center" role="alert">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" class="bi bi-check-circle-fill flex-shrink-0 me-3" viewBox="0 0 16 16" role="img" aria-label="Success:">
                                        <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z"/>
                                    </svg>
                                    <div>
                                        <p class="mb-2" style="font-weight: 500;">{"Thanks for registering!"}</p>
                                        <p class="mb-0">{"Check your emails to verify your account so that you can start to receive aurora alerts. Head back to the "}
                                        <LinkHome text="homepage" />
                                        {" to see the latest aurora activity."}</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <>
                            {
                                if let Some(_err) = &user_register.error {
                                    html! {
                                        <div class="row">
                                            <div class="col">
                                                <div class="alert alert-danger d-flex align-items-center" role="alert">
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" class="bi bi-exclamation-triangle-fill flex-shrink-0 me-3" viewBox="0 0 16 16" role="img" aria-label="Warning:">
                                                        <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/>
                                                    </svg>
                                                    <div>
                                                        {"There seems to have been an error - check that you have entered your details correctly and consider trying again later."}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                            <div class="row justify-content-center">
                                <div class="col-lg-auto">
                                    <Form {onsubmit}>
                                        <EmailField handler={email_handler} />
                                        <AlertThresholdField handler={alert_threshold_handler} />
                                        <LocationsField handler={locations_handler} />
                                        <button disabled={!valid_form} type="submit" class={classes!("btn", "btn-primary", "mb-3")}>{"Register"}</button>
                                    </Form>
                                </div>
                            </div>
                        </>
                    }
                }
            }
        </>
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
        <div class={classes!("form-floating", "mb-3")}>
            <input {oninput} placeholder="Email address" id="user-email" type="email" class={classes!("form-control")} />
            <label for="user-email" class={classes!("form-label")} style="color: #5c5c5c;">{"Email address"}</label>
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
        <div class={classes!("form-floating", "mb-3")}>
        <select {oninput} id="user-alert-threshold" class={classes!("form-select")}>
            <option value="yellow" selected=true>{"Yellow"}</option>
            <option value="amber">{"Amber"}</option>
            <option value="red">{"Red"}</option>
        </select>
        <label for="user-alert-threshold" class={classes!("form-label")}>{"Alert threshold"}</label>
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

    let locations_input_ref = use_node_ref();

    let onclick = {
        let chosen_locations = chosen_locations.clone();
        let datalist_locations = datalist_locations.clone();
        let location = location.clone();
        let locations_input_ref = locations_input_ref.clone();
        Callback::from(move |_: MouseEvent| {
            // add the location to the list below the input box
            let (name, location_id) = datalist_locations
                .data
                .as_ref()
                .unwrap()
                .get_key_value(&*location)
                .unwrap();

            let mut new_chosen_locations = chosen_locations.deref().clone();
            new_chosen_locations.insert(name.to_string(), *location_id);
            chosen_locations.set(new_chosen_locations);

            // clear the input box
            // safe to unwrap because we know that we have insert the ref into the DOM below
            let input = locations_input_ref.cast::<HtmlInputElement>().unwrap();
            input.set_value("");
        })
    };

    let valid_location = if let Some(locations) = &datalist_locations.data {
        locations.contains_key(&*location) && chosen_locations.deref().len() <= 5
    } else {
        false
    };

    html! {
        <>
            <div class={classes!("input-group", "mb-3")}>
                // we could easily add a country filter later on if we want to
                // <select class="form-select">
                //     <option value="GB" selected={true} style="max-width: max-content;">{"GB"}</option>
                // </select>
                <div class="form-floating flex-grow-1">
                    <input oninput={oninput_location} ref={locations_input_ref} id="location-list" class="form-control" list="location-list-options" Placeholder="Search for a location..." />
                    <label for="location-list" class="form-label" style="color: #5c5c5c;">{"Search for a location..."}</label>
                </div>
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
        <ul class={classes!("list-group", "mb-3", "user-select-none")}>
            {
                props.locations.iter().map(|(name, _)| html! {
                    <ChosenLocation name={name.clone()} location_to_remove={location_to_remove.clone()} />
                }).collect::<Html>()
            }
        </ul>
    }
}
