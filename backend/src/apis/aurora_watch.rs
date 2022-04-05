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

#[derive(Debug, Serialize)]
pub struct ActivityDataPoint {
    pub datetime: DT,
    pub value: f64,
}

#[derive(Debug, Serialize)]
pub struct ActivityData {
    pub station: String,
    pub start_time: DT,
    pub end_time: DT,
    pub creation_time: DT,
    pub activities: Vec<ActivityDataPoint>,
}

fn parse_single_value(mut lines: std::str::Lines) -> Result<(&str, std::str::Lines)> {
    // e.g. "STATION AWN/LAN1" -> "AWN/LAN1"
    // "0345 301 0574"
    let value = lines
        .next()
        .and_then(|line| line.split_once(' '))
        .and_then(|(_label, value)| Some(value))
        .ok_or(AuroraWatchError::Parse(
            "Error parsing single value".to_string(),
        ))?;
    Ok((value, lines))
}

fn parse_activity(line: &str) -> Result<ActivityDataPoint> {
    let mut split = line.splitn(4, ' ').skip(1);
    let datetime = split.next().ok_or(AuroraWatchError::Parse(
        "Error parsing activity datapoint: 'datetime' field not found".to_string(),
    ))?;
    let value = split.next().ok_or(AuroraWatchError::Parse(
        "Error parsing activity datapoint: 'value' field not found".to_string(),
    ))?;

    Ok(ActivityDataPoint {
        datetime: chrono::Utc.datetime_from_str(datetime, "%FT%T%#z")?,
        value: value.parse()?,
    })
}

fn parse_activities(lines: std::iter::Skip<std::str::Lines>) -> Result<Vec<ActivityDataPoint>> {
    let activities = lines.map(|line| parse_activity(line)).collect();
    activities
}

impl ActivityData {
    pub fn from_text(text: &str) -> Result<Self> {
        // example text: https://aurorawatch.lancs.ac.uk/api/0.1/activity.txt
        let lines = text.lines();
        let (station, rest) = parse_single_value(lines)?;
        let (start_time, rest) = parse_single_value(rest)?;
        let (end_time, rest) = parse_single_value(rest)?;
        let (creation_time, rest) = parse_single_value(rest)?;

        let rest = rest.skip(4);

        let activities = parse_activities(rest)?;

        Ok(Self {
            station: station.to_string(),
            start_time: chrono::Utc.datetime_from_str(start_time, "%FT%T%#z")?,
            end_time: chrono::Utc.datetime_from_str(end_time, "%FT%T%#z")?,
            creation_time: chrono::Utc.datetime_from_str(creation_time, "%FT%T%#z")?,
            activities,
        })
    }
}

pub async fn get_activity_data() -> Result<ActivityData> {
    let activity_data_url = "https://aurorawatch.lancs.ac.uk/api/0.1/activity.txt";
    let response = reqwest::get(activity_data_url).await?.text().await?;
    let activity_data = ActivityData::from_text(&response)?;

    Ok(activity_data)
}
