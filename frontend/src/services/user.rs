use crate::error::Error;
use crate::requests;
use crate::types::user::UserRegisterBody;

pub async fn register(user_info: UserRegisterBody) -> Result<(), Error> {
    requests::post::<(), _>("/api/users".to_string(), user_info).await
}

pub async fn verify(user_id: String, email: String) -> Result<(), Error> {
    requests::get::<()>(format!("/api/verify?user_id={}&email={}", user_id, email)).await
}

pub async fn unsubscribe(user_id: String, email: String) -> Result<(), Error> {
    requests::delete::<()>(format!("/api/users?user_id={}&email={}", user_id, email)).await
}
