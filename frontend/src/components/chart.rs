mod controls;
mod plot;

use yew::prelude::*;
use yew_hooks::use_async;
use yew_hooks::use_interval;

use self::controls::Controls;
use self::plot::ActivityPlot;
use crate::services::charts::{self, ActivityData};
type DateTimeUtc = chrono::DateTime<chrono::Utc>;

#[function_component(ActivityChart)]
pub fn activity_chart() -> Html {
    log::debug!("render activity chart");

    // None == the latest hour
    let selected_hour_handle = use_state(|| None::<DateTimeUtc>);
    let chart_data_handle = use_state_eq(|| None::<ActivityData>);

    let fetch_chart_data = {
        let selected_hour = *selected_hour_handle;
        use_async(async move {
            log::debug!("fetching activity data; end = {:?}", selected_hour.clone());
            charts::get_activity_data(selected_hour).await
        })
    };

    if let Some(data) = &fetch_chart_data.data {
        let data = data.clone();
        chart_data_handle.set(Some(data))
    };

    if let Some(data) = &fetch_chart_data.error {
        let data = data.clone();
        log::debug!("{data:#?}");
    }

    {
        let fetch_chart_data = fetch_chart_data.clone();
        let selected_hour_handle = selected_hour_handle.clone();
        use_effect_with_deps(
            move |_| {
                fetch_chart_data.run();
                || ()
            },
            selected_hour_handle,
        );
    }

    {
        let fetch_chart_data = fetch_chart_data;
        let selected_hour = *selected_hour_handle;
        use_interval(
            move || {
                // only fetch the data on an interval when the user is viewing the latest data
                if selected_hour.is_none() {
                    fetch_chart_data.run();
                }
            },
            5000,
        );
    }

    html! {
        <div id="activity-chart">
            {
                if let Some(data) = (*chart_data_handle).clone() {
                    html! {
                        <>
                            <ActivityPlot {data} />
                            <Controls selected_hour_handle={selected_hour_handle.clone()} chart_data_handle={chart_data_handle.clone()} />
                        </>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
