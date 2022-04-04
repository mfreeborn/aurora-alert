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
    pub email: String,
    pub alert_threshold: String,
    pub locations: std::collections::HashMap<String, i64>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UserSubscribeWrapper {
    pub payload: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserRegisterPostBody {
    pub email: String,
    pub alert_threshold: String,
    pub locations: Vec<i64>,
}

impl UserRegisterPostBody {
    pub fn is_valid(&self) -> bool {
        !self.email.is_empty() && !self.locations.is_empty()
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
