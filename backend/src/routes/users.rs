use axum::routing::{patch, post};
use axum::Extension;
use axum::Json;
use axum::Router;

use serde::Deserialize;

use crate::email;
use crate::templates;
use crate::Result;
use crate::{db, Error};

pub fn router() -> Router {
    Router::new()
        .route("/users", post(register).delete(unsubscribe))
        .route("/users/verify", patch(verify))
}

#[derive(Deserialize)]
pub struct VerifyUser {
    user_id: String,
    email: String,
}

#[derive(Deserialize)]
struct UnsubscribeUser {
    user_id: String,
    email: String,
}

/// Register a new user to the Aurora Alert service.
async fn register(
    Json(user_details): Json<db::RegisterUser>,
    Extension(db): db::Extension,
    Extension(template_engine): templates::Extension,
    Extension(mailer): email::Extension,
) -> Result<()> {
    // Two cases to handle:
    //     1. The user already exists
    //     2. The user does not exist
    //
    // If the user already exists, the frontend should act as if it is a successful registration when the
    // registration form is submitted, but they should be emailed an existing user email, rather than a
    // new user email.
    let user = db::get_user_by_email(&user_details, &db).await?;

    if let Some(user) = user {
        email::Email::new_user_already_registered(&user.email)
            .add_context(&user)?
            .render_body(&template_engine)?
            .build_email()?
            .send(mailer.clone());
    } else {
        let user = db::insert_user(&user_details, &db).await?;

        email::Email::new_verify_user(&user.email)
            .add_context(&user)?
            .render_body(&template_engine)?
            .build_email()?
            .send(mailer.clone());
    };

    Ok(())
}

/// Set the user with the given identifiers as verified.
pub async fn verify(
    Json(user_params): Json<VerifyUser>,
    Extension(db): db::Extension,
) -> Result<()> {
    let user_id = db::set_user_verified(&user_params.user_id, &user_params.email, &db).await?;

    if let Some(user_id) = user_id {
        tracing::debug!("user with user_id {user_id:?} verified successfully");
        Ok(())
    } else {
        tracing::debug!(
            "failed to verify user with user_id {:?}",
            user_params.user_id
        );
        Err(Error::NotFound)
    }
}

/// Unsubscribe the user with the given identifiers from the Aurora Alert service.
async fn unsubscribe(
    Json(user): Json<UnsubscribeUser>,
    Extension(db): db::Extension,
) -> Result<()> {
    let user_id = db::delete_user(&user.user_id, &user.email, &db).await?;

    if let Some(user_id) = user_id {
        tracing::debug!("user with user_id {user_id:?} deleted succesfully");
        Ok(())
    } else {
        tracing::debug!("failed to delete user with user_id {user_id:?}");
        Err(Error::NotFound)
    }
}
