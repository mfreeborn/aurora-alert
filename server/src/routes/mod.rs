use axum::Router;

use crate::startup::AppState;

mod core;
mod users;

pub fn api_router(app_state: AppState) -> Router {
    Router::new()
        .route("/ping", axum::routing::get(|| async { "pong" }))
        .merge(core::router(app_state.clone()))
        .merge(users::router(app_state.clone()))
}
