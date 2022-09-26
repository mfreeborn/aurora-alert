use chrono::TimeZone;
use serde::{Deserialize, Serialize};

use crate::common::{ActivityData, ActivityDataPoint, AlertLevel};
use crate::types::DateTimeUtc;

type Result<T> = std::result::Result<T, AuroraWatchError>;

#[derive(thiserror::Error, Debug)]
pub enum AuroraWatchError {
    #[error("{0}")]
    Request(#[from] reqwest::Error),
    #[error("{0}")]
    Parse(String),
}

impl std::convert::From<chrono::ParseError> for AuroraWatchError {
    fn from(e: chrono::ParseError) -> Self {
        AuroraWatchError::Parse(format!("error parsing datetime: {e}"))
    }
}

impl std::convert::From<std::num::ParseFloatError> for AuroraWatchError {
    fn from(e: std::num::ParseFloatError) -> Self {
        AuroraWatchError::Parse(format!("error parsing activity value: {e}"))
    }
}

impl std::convert::From<quick_xml::DeError> for AuroraWatchError {
    fn from(e: quick_xml::DeError) -> Self {
        AuroraWatchError::Parse(format!("error parsing XML: {e}"))
    }
}

pub trait ActivityDataExt
where
    Self: Sized,
{
    fn from_text(text: &str) -> Result<Self>;
}

impl ActivityDataExt for ActivityData {
    fn from_text(text: &str) -> Result<Self> {
        // Example text: https://aurorawatch.lancs.ac.uk/api/0.1/activity.txt
        let lines = text.lines();
        let (_station, rest) = parse_single_value(lines)?;
        let (_start_time, rest) = parse_single_value(rest)?;
        let (_end_time, rest) = parse_single_value(rest)?;
        let (updated_at, rest) = parse_single_value(rest).and_then(|(updated_at, rest)| {
            let updated_at = chrono::Utc.datetime_from_str(updated_at, "%FT%T%#z")?;
            Ok((updated_at, rest))
        })?;

        // Skip the 4 THRESHOLD definitions.
        let rest = rest.skip(4);

        let activities = parse_activities(rest)?;

        Ok(Self {
            updated_at,
            // The api always returns 24 results. Missing entries have a value of nan, which we convert to 0.0
            activities: activities.try_into().unwrap(),
        })
    }
}

fn parse_single_value(mut lines: std::str::Lines) -> Result<(&str, std::str::Lines)> {
    // viz. STATION AWN/LAN1 -> AWN/LAN1,
    //      START_TIME 2022-04-04T14:00:00+00 -> 2022-04-04T14:00:00+00,
    //      END_TIME 2022-04-05T14:00:00+00 -> 2022-04-05T14:00:00+00,
    //      CREATION_TIME 2022-04-05T13:12:31+00 -> 2022-04-05T13:12:31+00

    let value = lines
        .next()
        .and_then(|line| line.split_once(' '))
        .map(|(_label, value)| value)
        .ok_or_else(|| AuroraWatchError::Parse("Error parsing single value".to_string()))?;
    Ok((value, lines))
}

fn parse_activity(line: &str) -> Result<ActivityDataPoint> {
    // e.g. ACTIVITY 2022-04-04T14:00:00+00 24.0 green
    // Skip the ACTIVITY tag.
    let mut parts = line.splitn(4, ' ').skip(1);
    let datetime = parts
        .next()
        .ok_or_else(|| {
            AuroraWatchError::Parse(
                "Error parsing activity datapoint: 'datetime' field not found".to_string(),
            )
        })
        .and_then(|datetime| Ok(chrono::Utc.datetime_from_str(datetime, "%FT%T%#z")?))?;

    let value = parts.next().ok_or_else(|| {
        AuroraWatchError::Parse(
            "Error parsing activity datapoint: 'value' field not found".to_string(),
        )
    })?;

    // Helpfully convert nan to 0.0, so that it plays better with downstream data display.
    let value = value
        .parse::<f32>()
        .map(|val| if val.is_nan() { 0.0 } else { val })?;

    // Implicitly skip the 4th item in the activity line, which is the alert level.

    Ok(ActivityDataPoint {
        timestamp: datetime,
        value,
    })
}

fn parse_activities(lines: std::iter::Skip<std::str::Lines>) -> Result<Vec<ActivityDataPoint>> {
    lines.map(parse_activity).collect()
}

/// Retrieve the latest activity data from the AuroraWatch API.
pub async fn get_activity_data() -> Result<ActivityData> {
    let activity_data_url = "https://aurorawatch.lancs.ac.uk/api/0.1/activity.txt";
    let response = reqwest::get(activity_data_url).await?.text().await?;
    let activity_data = ActivityData::from_text(&response)?;

    Ok(activity_data)
}

#[derive(Debug, Deserialize)]
struct DateTime {
    #[serde(rename = "$value")]
    value: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
struct Updated {
    datetime: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct SiteStatus {
    status_id: AlertLevel,
    site_id: String,
}

#[derive(Debug, Deserialize)]
struct CurrentStatus {
    // api_version: String,
    updated: Updated,
    site_status: SiteStatus,
}

/// The current alert level derived from the AuroraWatch API.
#[derive(Debug, Deserialize)]
pub struct CurrentAlertLevel {
    pub level: AlertLevel,
    pub updated_at: DateTimeUtc,
    pub site_id: String,
}

impl From<CurrentStatus> for CurrentAlertLevel {
    fn from(current_status: CurrentStatus) -> Self {
        Self {
            level: current_status.site_status.status_id,
            updated_at: current_status.updated.datetime.value,
            site_id: current_status.site_status.site_id,
        }
    }
}

/// Retrieve the current alert level from the AuroraWatch API.
pub async fn get_alert_level() -> Result<CurrentAlertLevel> {
    let status_url = "https://aurorawatch-api.lancs.ac.uk/0.2/status/current-status.xml";
    let xml_response = reqwest::get(status_url).await?.text().await?;
    let status: CurrentAlertLevel = quick_xml::de::from_str::<CurrentStatus>(&xml_response)?.into();
    Ok(status)
}
