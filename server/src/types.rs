use std::ops::Deref;

use derive_more::Display;
use serde::{de::Deserializer, Deserialize, Serialize};

pub type DateTimeUtc = chrono::DateTime<chrono::Utc>;

/// Strip leading whitespace and wildcards from user-provided strings.
///
/// The resulting string can then be used safely in SQL `LIKE` clauses without the user being able to
/// interrogate the database for more/different information than they intend.
#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct SanitisedString(String);

impl SanitisedString {
    fn new(string: &str) -> Self {
        let string = string.trim_start().replace('%', "").replace('_', "");
        Self(string)
    }
}

impl Deref for SanitisedString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SanitisedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(Self::new(&s))
    }
}

/// An enumeration of the four different aurora alert levels.
#[derive(
    Clone, Copy, Debug, Display, Serialize, Deserialize, PartialOrd, PartialEq, sqlx::Type,
)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "alert_level_enum", rename_all = "lowercase")]
pub enum AlertLevel {
    Green,
    Yellow,
    Amber,
    Red,
}
