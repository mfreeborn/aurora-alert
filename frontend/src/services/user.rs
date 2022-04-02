use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::requests;
use crate::types::user::UserRegisterInfo;

pub async fn unsubscribe<T>(user_id: String, email: String) -> Result<T, Error>
where
    T: DeserializeOwned + std::fmt::Debug + 'static,
{
    requests::delete::<T>(format!("/users?user_id={}&email={}", user_id, email)).await
}

pub async fn register<T>(user_info: UserRegisterInfo) -> Result<T, Error>
where
    T: DeserializeOwned + std::fmt::Debug + 'static,
{
    requests::post::<T, _>("/users".to_string(), user_info).await
}
