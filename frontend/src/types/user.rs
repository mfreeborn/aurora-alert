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
    pub locations: Vec<usize>,
}

impl Default for UserRegisterInfo {
    fn default() -> Self {
        Self {
            email: String::new(),
            alert_threshold: "yellow".to_string(),
            locations: vec![],
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UserSubscribeWrapper {
    payload: String,
}
