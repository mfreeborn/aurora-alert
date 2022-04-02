use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::requests;

pub async fn unsubscribe<T>(user_id: String, email: String) -> Result<T, Error>
where
    T: DeserializeOwned + std::fmt::Debug + 'static,
{
    requests::delete::<T>(format!("/users?user_id={}&email={}", user_id, email)).await
}
