use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Clone, Debug, Display, Serialize, Deserialize, PartialOrd, PartialEq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum AlertLevel {
    Green,
    Yellow,
    Amber,
    Red,
}

#[derive(Clone, Copy, Debug, Deserialize_repr, Serialize, sqlx::Type)]
#[repr(u32)]
pub enum Location {
    FortWilliam = 2649169,
    SpeanBridge = 2637248,
}

impl Location {
    pub const fn city_id(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Deserialize)]
pub struct DateTime {
    #[serde(rename = "$value")]
    pub value: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Updated {
    pub datetime: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteStatus {
    #[serde(rename = "status_id")]
    pub alert_level: AlertLevel,
    pub project_id: String,
    pub site_id: String,
    pub site_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CurrentStatus {
    pub api_version: String,
    #[serde(rename = "updated")]
    pub updated_at: Updated,
    pub site_status: SiteStatus,
}
