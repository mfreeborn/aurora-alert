use actix_web::{delete, get, post, web, Responder, Result};
use chrono::Timelike;
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
    user_details: web::Json<db::RegisterUser>,
    pool: db::Extractor,
    template_engine: templates::Extractor,
    mailer: mail::Extractor,
) -> Result<impl Responder> {
    // Two cases to handle:
    //     1. The user already exists
    //     2. The user does not exist
    //
    // If the user already exists, the frontend should act as if it is a succesful registration when the
    // registration form is submitted, but they should be emailed an existing user email, rather than a
    // new user email

    let user = db::get_user_by_email(&user_details, pool.get_ref())
        .await
        .map_err(|e| errors::ApiError::Database {
            context: format!("Checking user exists failed: {}", e),
        })?;

    if let Some(user) = user {
        mail::Email::new_user_already_registered(&user.email)
            .add_context(&user)?
            .render_body(template_engine.get_ref())?
            .build_email()?
            .send(mailer.get_ref().clone());
    } else {
        let user = db::insert_user(&user_details, pool.get_ref())
            .await
            .map_err(|e| errors::ApiError::Database {
                context: format!("User insert failed: {}", e),
            })?;

        mail::Email::new_verify_user(&user.email)
            .add_context(&user)?
            .render_body(template_engine.get_ref())?
            .build_email()?
            .send(mailer.get_ref().clone());
    };

    Ok(JsonResponse::new("User registration initiated succesfully"))
}

#[derive(Deserialize)]
struct ActivityRouteParams {
    end: Option<chrono::DateTime<chrono::Utc>>,
}

#[get("/activity")]
async fn activity(
    params: web::Query<ActivityRouteParams>,
    pool: db::Extractor,
) -> Result<impl Responder> {
    let end = params.end.unwrap_or_else(|| {
        let now = chrono::Utc::now();
        now.date().and_hms(now.time().hour(), 0, 0)
    });
    let activity_data = db::get_activity_data(end, pool.get_ref())
        .await
        .map_err(|e| errors::ApiError::Api {
            context: format!("Error retrieving aurora activity data: {e}"),
        })?;

    Ok(actix_web::web::Json(activity_data))
}
