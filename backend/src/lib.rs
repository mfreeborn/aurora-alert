use std::net::SocketAddr;

use anyhow::Context;
use axum::{Extension, Router, Server};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{config::Config, db::DbPool, email::EmailTransport, templates::TemplateEngine};

mod apis;
pub mod config;
pub mod db;
pub mod email;
mod error;
mod helpers;
mod routes;
pub mod tasks;
pub mod templates;
mod types;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn serve(
    config: Config,
    db: DbPool,
    template_engine: TemplateEngine,
    email_transport: EmailTransport,
) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 9090));

    tracing::info!("listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app(config, db, template_engine, email_transport).into_make_service())
        .await
        .context("error running HTTP server")
}

fn app(
    config: Config,
    db: DbPool,
    template_engine: TemplateEngine,
    email_transport: EmailTransport,
) -> Router {
    api_router()
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(Extension(db))
                .layer(Extension(template_engine))
                .layer(Extension(email_transport))
                .layer(Extension(config)),
        )
}

fn api_router() -> Router {
    routes::users::router().merge(routes::core::router())
}
