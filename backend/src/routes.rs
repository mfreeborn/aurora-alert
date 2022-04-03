use std::fmt;

use actix_web::{delete, get, post, web, Responder, Result};
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    Deserialize, Serialize,
};

use crate::db;
use crate::errors;
use crate::mail;
use crate::templates;

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

#[derive(Debug)]
struct StrippedString {
    search: String,
}

#[derive(Debug)]
struct LocationQueryParams {
    search: String,
}

impl LocationQueryParams {
    fn new(search: String) -> Self {
        Self { search }
    }
}

impl<'de> Deserialize<'de> for LocationQueryParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Search,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`search`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "search" => Ok(Field::Search),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct LocationQueryParamsVisitor;

        impl<'de> Visitor<'de> for LocationQueryParamsVisitor {
            type Value = LocationQueryParams;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct LocationQueryParams")
            }

            fn visit_map<V>(self, mut map: V) -> Result<LocationQueryParams, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut search = None::<String>;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Search => {
                            if search.is_some() {
                                return Err(de::Error::duplicate_field("search"));
                            }
                            search = Some(map.next_value()?);
                        }
                    }
                }
                let search = search.ok_or_else(|| de::Error::missing_field("search"))?;
                let search = search.trim();
                let search = search.replace("%", "");
                let search = search.replace("_", "");
                Ok(LocationQueryParams::new(search))
            }
        }

        const FIELDS: &'static [&'static str; 1] = &["search"];
        deserializer.deserialize_struct("LocationQueryParams", FIELDS, LocationQueryParamsVisitor)
    }
}

#[get("/locations")]
async fn locations(
    search_query: web::Query<LocationQueryParams>,
    pool: db::Extractor,
) -> Result<impl Responder> {
    log::info!("{search_query:?}");
    // short circuit if the search string is blank. We could allow it to proceed, and that would
    // mean we return the first LIMIT locations in this case, rather than none
    if search_query.search.is_empty() {
        return Ok(web::Json(vec![]));
    }

    let locations = db::get_locations(&search_query.search, pool.get_ref())
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
