use common::ApiResponse;

use crate::error::Error;
use crate::requests;
use crate::types::user::UserRegisterBody;

pub async fn register(user_info: UserRegisterBody) -> Result<ApiResponse, Error> {
    requests::post::<ApiResponse, _>("/api/users".to_string(), user_info).await
}

pub async fn verify(user_id: String, email: String) -> Result<ApiResponse, Error> {
    requests::get::<ApiResponse>(format!("/api/verify?user_id={}&email={}", user_id, email)).await
}

pub async fn unsubscribe(user_id: String, email: String) -> Result<ApiResponse, Error> {
    requests::delete::<ApiResponse>(format!("/api/users?user_id={}&email={}", user_id, email)).await
}
