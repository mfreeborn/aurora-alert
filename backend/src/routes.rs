use crate::db;
use crate::errors;
use crate::mail;
use crate::templates;

use actix_web::{get, post, web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct JsonResponse {
    pub payload: String,
}

impl JsonResponse {
    pub fn new(payload: &str) -> web::Json<Self> {
        web::Json(Self {
            payload: payload.to_string(),
        })
    }
}

#[derive(Deserialize)]
struct UnsubscribeUserParams {
    user_id: String,
    email: String,
}

#[get("/unsubscribe")]
async fn unsubscribe(
    user_params: web::Query<UnsubscribeUserParams>,
    pool: db::Extractor,
) -> Result<impl Responder> {
    let user_id = db::delete_user(&user_params.user_id, &user_params.email, pool.get_ref())
        .await
        .map_err(|e| errors::ApiError::Database {
            context: format!("Error deleting user: {}", e),
        })?;

    if let Some(user_id) = user_id {
        Ok(JsonResponse::new(&format!(
            "User with user_id = {:?} deleted succesfully",
            user_id
        )))
    } else {
        Err(errors::ApiError::Database {
            context: format!(
                "No such user exists with user_id = {:?} and email = {:?}",
                user_params.user_id, user_params.email
            ),
        }
        .into())
    }
}

#[derive(Deserialize)]
struct VerifiedUserParams {
    user_id: String,
    email: String,
}

#[get("/verify")]
async fn verify(
    user_params: web::Query<VerifiedUserParams>,
    pool: db::Extractor,
) -> Result<impl Responder> {
    let user_id = db::set_user_verified(&user_params.user_id, &user_params.email, pool.get_ref())
        .await
        .map_err(|e| errors::ApiError::Database {
            context: format!("Error updating user verified status: {}", e),
        })?;

    if let Some(user_id) = user_id {
        Ok(JsonResponse::new(&format!(
            "User wither user_id = {:?} verified succesfully",
            user_id
        )))
    } else {
        Err(errors::ApiError::Database {
            context: format!(
                "No such user exists with user_id = {:?} and email = {:?}",
                user_params.user_id, user_params.email
            ),
        }
        .into())
    }
}

#[post("/register")]
async fn register(
    user: web::Json<db::RegisterUser>,
    pool: db::Extractor,
    template_engine: templates::Extractor,
    mailer: mail::Extractor,
) -> Result<impl Responder> {
    let user =
        db::insert_user(&user, pool.get_ref())
            .await
            .map_err(|e| errors::ApiError::Database {
                context: format!("User insert failed: {}", e),
            })?;

    mail::Email::build_verify(&user, template_engine.get_ref())?.send(mailer.get_ref().clone());

    Ok(JsonResponse::new("User registered succesfully"))
}
