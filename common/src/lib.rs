use derive_more::Display;
use serde::{Deserialize, Serialize};

type DateTimeUtc = chrono::DateTime<chrono::Utc>;

/// An enumeration of the four different aurora alert levels.
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[derive(Clone, Copy, Debug, Display, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(
    feature = "sql",
    sqlx(type_name = "alert_level_enum", rename_all = "lowercase")
)]
pub enum AlertLevel {
    Green,
    Yellow,
    Amber,
    Red,
}

/// A container for 24 contiguous hours of activity data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ActivityData {
    pub activities: [ActivityDataPoint; 24],
    pub updated_at: DateTimeUtc,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ActivityDataPoint {
    pub timestamp: DateTimeUtc,
    pub value: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse {
    success: bool,
}

impl ApiResponse {
    pub fn success() -> Self {
        Self { success: true }
    }

    pub fn failure() -> Self {
        Self { success: true }
    }
}
