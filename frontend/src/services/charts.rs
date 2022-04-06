use ordered_float::NotNan;
use serde::Deserialize;

use super::requests;
use crate::error::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct ActivityDataPoint {
    pub datetime: chrono::DateTime<chrono::Utc>,
    pub value: NotNan<f32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ActivityData {
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub activities: Vec<ActivityDataPoint>,
}

pub async fn get_activity_data() -> Result<ActivityData, Error> {
    requests::get::<ActivityData>("/activity".to_string()).await
}
