use ordered_float::NotNan;
use plotly::{
    common::{Marker, Title},
    configuration::DisplayModeBar,
    layout::{
        themes::{PLOTLY_DARK, PLOTLY_WHITE},
        Axis,
    },
};
use serde::Deserialize;

use super::requests;
use crate::error::Error;
use crate::theme::{ThemeMode, AMBER, GREEN, RED, YELLOW};

type DateTimeUtc = chrono::DateTime<chrono::Utc>;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ActivityDataPoint {
    pub timestamp: DateTimeUtc,
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
    fn from_response(resp: ActivityDataResponse) -> Self {
        ActivityData {
            updated_at: resp.updated_at,
            // This unwrap won't fail because we already ensured that 24 elements are present on the server side.
            activities: resp.activities.try_into().unwrap(),
        }
    }

    pub fn to_plot(&self, theme_mode: ThemeMode) -> plotly::Plot {
        let mut plot = plotly::Plot::new();

        let trace = plotly::Bar::new(
            self.activities
                .iter()
                .map(|a| a.timestamp.with_timezone(&chrono::Local).to_rfc3339())
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
        )
        .hover_template(r#"<b>%{x|%-d %b %y %H:%M}</b><br>Activity: %{y:.1f}nT<extra></extra>"#);
        plot.add_trace(trace);

        let layout = plotly::Layout::new()
            .template(match theme_mode {
                ThemeMode::Dark => &*PLOTLY_DARK,
                ThemeMode::Light => &*PLOTLY_WHITE,
            })
            .title(
                Title::new(&format!(
                    "<b>Lastest Geomagnetic Activity</b><br><sub>Last updated {}</sub>",
                    self.updated_at
                        .with_timezone(&chrono::Local)
                        .format("%-d %b %y %H:%M %Z")
                ))
                .x(0.5),
            )
            .x_axis(
                Axis::new()
                    .title(
                        format!(
                            "Time ({})",
                            self.updated_at.with_timezone(&chrono::Local).format("%Z")
                        )
                        .as_str()
                        .into(),
                    )
                    .fixed_range(true),
            )
            .y_axis(Axis::new().title("Activity (nT)".into()).fixed_range(true));

        plot.set_layout(layout);

        let config = plotly::Configuration::new()
            .display_mode_bar(DisplayModeBar::False)
            .scroll_zoom(false)
            .show_axis_drag_handles(false);
        plot.set_configuration(config);

        plot
    }
}

pub async fn get_activity_data(end: DateTimeUtc) -> Result<ActivityData, Error> {
    let params = format!("?end={end:?}");

    let data = requests::get::<ActivityDataResponse>(format!("api/activity{params}")).await?;

    Ok(ActivityData::from_response(data))
}
