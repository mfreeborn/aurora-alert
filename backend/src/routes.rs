use actix_web::{delete, get, post, web, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::db;
use crate::errors;
use crate::mail;
use crate::templates;
use crate::types;

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

#[derive(Debug, Deserialize)]
struct LocationQueryParams {
    search: types::SanitisedLikeString,
}

#[get("/locations")]
async fn locations(
    search_query: web::Query<LocationQueryParams>,
    pool: db::Extractor,
) -> Result<impl Responder> {
    let search_query = &search_query.search;
    log::info!("{search_query:?}");
    // short circuit if the search string is blank. We could allow it to proceed, and that would
    // mean we return the first LIMIT locations in this case, rather than none
    if search_query.is_empty() {
        return Ok(web::Json(vec![]));
    }

    let locations = db::get_locations(search_query, pool.get_ref())
        .await
        .map_err(|e| errors::ApiError::Database {
            context: format!("Error getting locations: {}", e),
        })?;
    Ok(web::Json(locations))
}

#[derive(Deserialize)]
struct UnsubscribeUserParams {
    user_id: String,
    email: String,
}

#[delete("/users")]
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
            "User with user_id = {} deleted succesfully",
            user_id
        )))
    } else {
        Err(errors::ApiError::Database {
            context: format!(
                "No such user exists with user_id = {} and email = {}",
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

#[post("/users")]
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

    mail::Email::new_verify_user(&user.email)
        .add_context(&user)?
        .render_body(template_engine.get_ref())?
        .build_email()?
        .send(mailer.get_ref().clone());

    Ok(JsonResponse::new("User registered succesfully"))
}

#[get("/activity")]
async fn activity() -> Result<impl Responder> {
    let activity = crate::apis::aurora_watch::get_activity_data()
        .await
        .map_err(|e| errors::ApiError::Api {
            context: format!("Error fetching activity data: {e}"),
        })?;

    Ok(actix_web::web::Json(activity))
}
