use chrono::Timelike;
use yew::prelude::*;

type DateTimeUtc = chrono::DateTime<chrono::Utc>;

use crate::services::charts::ActivityData;

#[derive(Properties, PartialEq)]
pub struct ControlsProps {
    pub selected_hour_handle: UseStateHandle<Option<DateTimeUtc>>,
    pub chart_data_handle: UseStateHandle<Option<ActivityData>>,
}

#[function_component(Controls)]
pub fn controls(props: &ControlsProps) -> Html {
    log::debug!("render controls");
    let ControlsProps {
        selected_hour_handle,
        chart_data_handle,
    } = props;

    let onclick_back_hour = {
        let selected_hour_handle = selected_hour_handle.clone();
        let chart_data = (*chart_data_handle).clone();
        Callback::from(move |_| {
            let current_selected_hour = (*selected_hour_handle).unwrap_or_else(|| {
                let updated_at = chart_data.as_ref().unwrap().updated_at;
                updated_at.date().and_hms(updated_at.hour(), 0, 0)
            });
            let new_selected_hour = current_selected_hour - chrono::Duration::hours(1);
            selected_hour_handle.set(Some(new_selected_hour));
        })
    };

    let onclick_back_day = {
        let selected_hour_handle = selected_hour_handle.clone();
        let chart_data = (*chart_data_handle).clone();
        Callback::from(move |_| {
            let current_selected_hour = (*selected_hour_handle).unwrap_or_else(|| {
                let updated_at = chart_data.as_ref().unwrap().updated_at;
                updated_at.date().and_hms(updated_at.hour(), 0, 0)
            });
            let new_selected_hour = current_selected_hour - chrono::Duration::days(1);
            selected_hour_handle.set(Some(new_selected_hour));
        })
    };
    let onclick_forward_hour = {
        let selected_hour_handle = selected_hour_handle.clone();
        let chart_data = (*chart_data_handle).clone();
        Callback::from(move |_| {
            if let Some(current_selected_hour) = *selected_hour_handle {
                let new_selected_hour = current_selected_hour + chrono::Duration::hours(1);
                let now = {
                    let updated_at = chart_data.as_ref().unwrap().updated_at;
                    updated_at.date().and_hms(updated_at.hour(), 0, 0)
                };
                let current_hour = now.date().and_hms(now.hour(), 0, 0);

                let new_selected_hour = if new_selected_hour < current_hour {
                    Some(new_selected_hour)
                } else {
                    None
                };
                selected_hour_handle.set(new_selected_hour);
            };
        })
    };

    let onclick_forward_day = {
        let selected_hour_handle = selected_hour_handle.clone();
        let chart_data = (*chart_data_handle).clone();
        Callback::from(move |_| {
            if let Some(current_selected_hour) = *selected_hour_handle {
                let new_selected_hour = current_selected_hour + chrono::Duration::days(1);
                let now = {
                    let updated_at = chart_data.as_ref().unwrap().updated_at;
                    updated_at.date().and_hms(updated_at.hour(), 0, 0)
                };
                let current_hour = now.date().and_hms(now.hour(), 0, 0);

                let new_selected_hour = if new_selected_hour < current_hour {
                    Some(new_selected_hour)
                } else {
                    None
                };
                selected_hour_handle.set(new_selected_hour);
            };
        })
    };

    let onclick_jump_latest = {
        let selected_hour_handle = selected_hour_handle.clone();
        Callback::from(move |_| {
            selected_hour_handle.set(None);
        })
    };

    html! {
        <div id="activity-chart-controls" class="row justify-content-center gx-2">
            <div class="col-auto">
                <button onclick={onclick_back_day} type="button" class="btn btn-primary">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-chevron-double-left" viewBox="0 0 16 16">
                        <path fill-rule="evenodd" d="M8.354 1.646a.5.5 0 0 1 0 .708L2.707 8l5.647 5.646a.5.5 0 0 1-.708.708l-6-6a.5.5 0 0 1 0-.708l6-6a.5.5 0 0 1 .708 0z"/>
                        <path fill-rule="evenodd" d="M12.354 1.646a.5.5 0 0 1 0 .708L6.707 8l5.647 5.646a.5.5 0 0 1-.708.708l-6-6a.5.5 0 0 1 0-.708l6-6a.5.5 0 0 1 .708 0z"/>
                    </svg>
                </button>
            </div>
            <div class="col-auto">
                <button onclick={onclick_back_hour} type="button" class="btn btn-primary">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-chevron-left" viewBox="0 0 16 16">
                        <path fill-rule="evenodd" d="M11.354 1.646a.5.5 0 0 1 0 .708L5.707 8l5.647 5.646a.5.5 0 0 1-.708.708l-6-6a.5.5 0 0 1 0-.708l6-6a.5.5 0 0 1 .708 0z"></path>
                    </svg>
                </button>
            </div>
            <div class="col-auto">
                <button onclick={onclick_jump_latest} disabled={(*selected_hour_handle).clone().is_none()} type="button" class="btn btn-primary">{"Latest"}</button>
            </div>
            <div class="col-auto">
                <button onclick={onclick_forward_hour} disabled={(*selected_hour_handle).clone().is_none()} type="button" class="btn btn-primary">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-chevron-right" viewBox="0 0 16 16">
                        <path fill-rule="evenodd" d="M4.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L10.293 8 4.646 2.354a.5.5 0 0 1 0-.708z"/>
                    </svg>
                </button>
            </div>
            <div class="col-auto">
                <button onclick={onclick_forward_day} disabled={(*selected_hour_handle).clone().is_none()} type="button" class="btn btn-primary">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-chevron-double-right" viewBox="0 0 16 16">
                        <path fill-rule="evenodd" d="M3.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L9.293 8 3.646 2.354a.5.5 0 0 1 0-.708z"/>
                        <path fill-rule="evenodd" d="M7.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L13.293 8 7.646 2.354a.5.5 0 0 1 0-.708z"/>
                    </svg>
                </button>
            </div>
        </div>
    }
}
