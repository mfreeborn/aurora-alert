use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// An enumeration of all possible error variants.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Return `404 Not Found`.
    #[error("request path not found")]
    NotFound,

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("email error: {0}")]
    Email(#[from] crate::email::EmailError),

    #[error("template error: {0}")]
    Template(#[from] tera::Error),

    #[error("internal server error: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    /// Associate a HTTP status code with each error variant.
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Provide a generic error description for each error variant.
    ///
    /// The purpose here is to prevent internal error details from leaking out to the client.
    fn to_generic_string(&self) -> String {
        match self {
            Self::Database(_) => "an error occurred with the database".to_string(),

            Self::Email(_) => "an error occurred with the email system".to_string(),

            Self::Template(_) => "an error occurred with the template system".to_string(),

            Self::Anyhow(_) => "an internal server error occurred".to_string(),

            _ => self.to_string(),
        }
    }
}

impl IntoResponse for Error {
    /// Convert our custom error into something that can be returned directly from a route.
    ///
    /// Note that we only provide generic error messages here, which get relayed to the client. The
    /// details of the error will get logged (see `IntoResponse` impl below) on the server, thus
    /// preventing internal details leaking to the client.
    fn into_response(self) -> Response {
        match self {
            Self::Database(ref e) => {
                tracing::error!("database error: {:?}", e);
            }

            Self::Email(ref e) => {
                tracing::error!("email error: {:?}", e);
            }

            Self::Template(ref e) => {
                tracing::error!("tempalte error: {:?}", e);
            }

            Self::Anyhow(ref e) => {
                tracing::error!("error: {:?}", e);
            }

            // Other errors get mapped normally.
            _ => (),
        }

        (self.status_code(), self.to_generic_string()).into_response()
    }
}
