use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UnsubscribeUserWrapper {
    pub payload: String,
}

#[derive(Clone, Deserialize)]
pub struct UnsubscribeParams {
    pub user_id: String,
    pub email: String,
}
