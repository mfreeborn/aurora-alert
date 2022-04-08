use chrono::Timelike;
use ordered_float::NotNan;
use serde::Deserialize;

use super::requests;
use crate::error::Error;

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
                log::debug!("{i}");
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
