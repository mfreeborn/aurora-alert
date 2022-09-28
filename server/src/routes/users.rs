use axum::debug_handler;
use axum::extract::State;
use axum::routing::{patch, post, Router};
use axum::Json;
use common::ApiResponse;
use serde::Deserialize;
use uuid::Uuid;

use crate::db;
use crate::error::Error;
use crate::startup::{AppState, DbState};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::with_state(app_state)
        .route("/users", post(register).delete(unsubscribe))
        .route("/users/verify", patch(verify))
}

#[derive(Deserialize)]
pub struct VerifyUser {
    user_id: Uuid,
    email: String,
}

#[derive(Deserialize)]
struct UnsubscribeUser {
    user_id: Uuid,
    email: String,
}

/// Register a new user to the Aurora Alert service.
#[debug_handler]
async fn register(
    State(app_state): State<AppState>,
    Json(user_details): Json<db::RegisterUser>,
) -> Result<Json<ApiResponse>, Error> {
    // Two cases to handle:
    //     1. The user already exists
    //     2. The user does not exist
    //
    // If the user already exists, the frontend should act as if it is a successful
    // registration when the registration form is submitted, but they should be
    // emailed an existing user email, rather than a new user email.
    let pool = app_state.database.pool;
    let email_client = app_state.email.email_client;

    let user = db::get_user_by_email(&user_details, &pool).await?;
    if let Some(user) = user {
        let message = email_client
            .new_user_already_registered(&user.email)
            .add_context(&user)?
            .build_email()?;

        email_client.send(message).await;
    } else {
        let user = db::insert_user(&user_details, &pool).await?;

        let message = email_client
            .new_verify_user(&user.email)
            .add_context(&user)?
            .build_email()?;

        email_client.send(message).await;
    };

    Ok(Json(ApiResponse::success()))
}

/// Set the user with the given identifiers as verified.
async fn verify(
    State(db): State<DbState>,
    Json(user_params): Json<VerifyUser>,
) -> Result<Json<ApiResponse>, Error> {
    let user_id = db::set_user_verified(&user_params.user_id, &user_params.email, &db.pool).await?;

    if let Some(user_id) = user_id {
        tracing::debug!("user with user_id {user_id:?} verified successfully");
        Ok(Json(ApiResponse::success()))
    } else {
        tracing::debug!(
            "failed to verify user with user_id {:?}",
            user_params.user_id
        );
        Err(Error::NotFound)
    }
}

/// Unsubscribe the user with the given identifiers from the Aurora Alert
/// service.
async fn unsubscribe(
    State(db): State<DbState>,
    Json(user): Json<UnsubscribeUser>,
) -> Result<Json<ApiResponse>, Error> {
    let user_id = db::delete_user(&user.user_id, &user.email, &db.pool).await?;

    if let Some(user_id) = user_id {
        tracing::debug!("user with user_id {user_id:?} deleted succesfully");
        Ok(Json(ApiResponse::success()))
    } else {
        tracing::debug!("failed to delete user with user_id {user_id:?}");
        Err(Error::NotFound)
    }
}
