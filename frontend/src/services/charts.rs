use chrono::Timelike;
use ordered_float::NotNan;
use plotly::{
    common::{Marker, Title},
    layout::Axis,
    themes::{PLOTLY_DARK, PLOTLY_WHITE},
};
use serde::Deserialize;

use super::requests;
use crate::error::Error;
use crate::theme::{ThemeMode, AMBER, GREEN, RED, YELLOW};

type DateTimeUtc = chrono::DateTime<chrono::Utc>;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ActivityDataPoint {
    pub datetime: DateTimeUtc,
    pub value: NotNan<f32>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ActivityDataResponse {
    pub updated_at: DateTimeUtc,
    pub activities: Vec<ActivityDataPoint>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ActivityData {
    pub updated_at: DateTimeUtc,
    pub activities: [ActivityDataPoint; 24],
}

impl ActivityData {
    fn from_response(resp: ActivityDataResponse, end: Option<DateTimeUtc>) -> Self {
        if resp.activities.len() == 24 {
            ActivityData {
                updated_at: resp.updated_at,
                // this unwrap won't fail because we already checked that 24 elements are present
                activities: resp.activities.try_into().unwrap(),
            }
        } else if end.is_none() {
            // there should never be an occassion where both the end time is Some<_> and the response
            // has fewer than 24 entries
            unreachable!()
        } else {
            // we need to pad out the activities
            let end = end.unwrap().date().and_hms(end.unwrap().hour(), 0, 0);
            let mut activities = resp.activities;
            for i in (activities.len() as i64)..24 {
                activities.push(ActivityDataPoint {
                    datetime: end - chrono::Duration::hours(i),
                    // safe to unwrap because 0. is not NaN
                    value: NotNan::new(0.).unwrap(),
                });
            }
            ActivityData {
                updated_at: resp.updated_at,
                // safe to unwrap because we've just ensured that the activites vec is indeed 24
                // elements long
                activities: activities.try_into().unwrap(),
            }
        }
    }

    pub fn to_plot(&self, theme_mode: ThemeMode) -> plotly::Plot {
        let mut plot = plotly::Plot::new();

        let trace = plotly::Bar::new(
            self.activities
                .iter()
                .map(|a| a.datetime.with_timezone(&chrono::Local).to_rfc3339())
                .collect(),
            self.activities.iter().map(|a| a.value).collect(),
        )
        .marker(
            Marker::new().color_array(
                self.activities
                    .iter()
                    .map(|a| {
                        if a.value < NotNan::new(50.).unwrap() {
                            GREEN
                        } else if a.value < NotNan::new(100.).unwrap() {
                            YELLOW
                        } else if a.value < NotNan::new(200.).unwrap() {
                            AMBER
                        } else {
                            RED
                        }
                    })
                    .collect(),
            ),
        );
        plot.add_trace(trace);

        let layout = plotly::Layout::new()
            .template(if theme_mode == ThemeMode::Dark {
                &*PLOTLY_DARK
            } else {
                &*PLOTLY_WHITE
            })
            .title(
                Title::new(
                    format!(
                        "<b>Lastest Geomagnetic Activity</b><br><sub>Last updated {}</sub>",
                        self.updated_at
                            .with_timezone(&chrono::Local)
                            .format("%-d %b %y %H:%M %Z")
                    )
                    .as_str(),
                )
                .x(0.5),
            )
            .x_axis(
                Axis::new().title(
                    format!(
                        "Time ({})",
                        self.updated_at.with_timezone(&chrono::Local).format("%Z")
                    )
                    .as_str()
                    .into(),
                ),
            )
            .y_axis(Axis::new().title("Activity (nT)".into()));

        plot.set_layout(layout);

        let config = plotly::Configuration::new().display_mode_bar(false);
        plot.set_configuration(config);

        plot
    }
}

pub async fn get_activity_data(end: Option<DateTimeUtc>) -> Result<ActivityData, Error> {
    let params = if let Some(end) = end {
        format!("?end={end}")
    } else {
        "".to_string()
    };
    let data = requests::get::<ActivityDataResponse>(format!("/activity{params}")).await?;

    Ok(ActivityData::from_response(data, end))
}
