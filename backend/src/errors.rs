use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "Database Error")]
    Database { context: String },

    #[display(fmt = "Template Error")]
    Template { context: String },

    #[display(fmt = "Email Error")]
    Email { context: String },

    #[display(fmt = "Api Error")]
    Api { context: String },
}

impl ApiError {
    fn get_context(&self) -> Option<String> {
        match self {
            ApiError::Database { context }
            | ApiError::Template { context }
            | ApiError::Email { context }
            | ApiError::Api { context } => Some(context.clone()),
        }
    }
}

#[derive(Serialize)]
pub struct JsonErrorResponse {
    pub error: String,
    pub context: String,
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(JsonErrorResponse {
            error: self.to_string(),
            context: self.get_context().unwrap_or_else(|| String::from("")),
        })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::Database { .. }
            | ApiError::Template { .. }
            | ApiError::Email { .. }
            | ApiError::Api { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
