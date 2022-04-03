use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UnsubscribeUserWrapper {
    pub payload: String,
}

#[derive(Clone, Deserialize)]
pub struct UnsubscribeParams {
    pub user_id: String,
    pub email: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserRegisterInfo {
    pub email: Option<String>,
    pub alert_threshold: String,
    pub locations: std::collections::HashMap<String, i64>,
}

impl Default for UserRegisterInfo {
    fn default() -> Self {
        Self {
            email: None,
            alert_threshold: "yellow".to_string(),
            locations: HashMap::new(),
        }
    }
}

impl UserRegisterInfo {
    pub fn is_valid(&self) -> bool {
        self.email.is_some() && !self.locations.is_empty()
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UserSubscribeWrapper {
    payload: String,
}

#[derive(Debug, Serialize)]
pub struct UserRegisterPostBody {
    pub email: String,
    pub alert_threshold: String,
    pub locations: Vec<i64>,
}

impl From<UserRegisterInfo> for UserRegisterPostBody {
    fn from(other: UserRegisterInfo) -> Self {
        Self {
            email: other.email.unwrap(),
            alert_threshold: other.alert_threshold.clone(),
            locations: other.locations.values().cloned().collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct VerifyUserWrapper {
    pub payload: String,
}

#[derive(Clone, Deserialize)]
pub struct VerifyUserParams {
    pub user_id: String,
    pub email: String,
}
