use anyhow;
use chrono::TimeZone;
use quick_xml;
use serde::Serialize;

use crate::types;

#[derive(thiserror::Error, Debug)]
pub enum AuroraWatchError {
    #[error("{0}")]
    Request(String),
    #[error("{0}")]
    Parse(String),
}

impl std::convert::From<chrono::ParseError> for AuroraWatchError {
    fn from(e: chrono::ParseError) -> Self {
        AuroraWatchError::Parse(format!("Error parsing datetime: {e}"))
    }
}

impl std::convert::From<std::num::ParseFloatError> for AuroraWatchError {
    fn from(e: std::num::ParseFloatError) -> Self {
        AuroraWatchError::Parse(format!("Error parsing activity value: {e}"))
    }
}

impl std::convert::From<reqwest::Error> for AuroraWatchError {
    fn from(e: reqwest::Error) -> Self {
        AuroraWatchError::Request(format!("Request error: {e}"))
    }
}

type Result<T> = std::result::Result<T, AuroraWatchError>;

pub async fn get_alert_level() -> anyhow::Result<types::CurrentStatus> {
    let status_url = "https://aurorawatch-api.lancs.ac.uk/0.2/status/current-status.xml";
    let xml_response = reqwest::get(status_url).await?.text().await?;
    let status: types::CurrentStatus = quick_xml::de::from_str(&xml_response)?;
    Ok(status)
}

type DT = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ActivityDataPoint {
    pub datetime: DT,
    pub value: f64,
}

#[derive(Debug, Serialize)]
pub struct ActivityData {
    pub updated_at: DT,
    pub activities: [ActivityDataPoint; 24],
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
    // skip the ACTIVITY tag
    let mut parts = line.splitn(4, ' ').skip(1);
    let datetime = parts.next().ok_or_else(|| {
        AuroraWatchError::Parse(
            "Error parsing activity datapoint: 'datetime' field not found".to_string(),
        )
    })?;
    let value = parts.next().ok_or_else(|| {
        AuroraWatchError::Parse(
            "Error parsing activity datapoint: 'value' field not found".to_string(),
        )
    })?;

    let mut value = value.parse::<f64>()?;
    if value.is_nan() {
        value = 0.0;
    }

    // implicitly skip the 4th item in the activity line, which is the alert level

    Ok(ActivityDataPoint {
        datetime: chrono::Utc.datetime_from_str(datetime, "%FT%T%#z")?,
        value,
    })
}

fn parse_activities(lines: std::iter::Skip<std::str::Lines>) -> Result<Vec<ActivityDataPoint>> {
    lines.map(parse_activity).collect()
}

impl ActivityData {
    pub fn from_text(text: &str) -> Result<Self> {
        // example text: https://aurorawatch.lancs.ac.uk/api/0.1/activity.txt
        let lines = text.lines();
        let (_station, rest) = parse_single_value(lines)?;
        let (_start_time, rest) = parse_single_value(rest)?;
        let (_end_time, rest) = parse_single_value(rest)?;
        let (updated_at, rest) = parse_single_value(rest)?;

        // skip the 4 THRESHOLD definitions
        let rest = rest.skip(4);

        let activities = parse_activities(rest)?;

        Ok(Self {
            updated_at: chrono::Utc.datetime_from_str(updated_at, "%FT%T%#z")?,
            // The api always returns 24 results. Missing entries have a value of nan, which we convert to 0.0
            activities: activities.try_into().unwrap(),
        })
    }
}

pub async fn get_activity_data() -> Result<ActivityData> {
    let activity_data_url = "https://aurorawatch.lancs.ac.uk/api/0.1/activity.txt";
    let response = reqwest::get(activity_data_url).await?.text().await?;
    let activity_data = ActivityData::from_text(&response)?;

    Ok(activity_data)
}
