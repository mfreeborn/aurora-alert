use derive_more::Display;
use serde::{de::Deserializer, Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct SanitisedLikeString(String);

impl SanitisedLikeString {
    fn new(string: String) -> Self {
        let string = string.replace("%", "");
        let string = string.replace("_", "");
        Self(string)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'de> Deserialize<'de> for SanitisedLikeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(Self::new(s))
    }
}

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
