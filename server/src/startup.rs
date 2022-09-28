use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};

use axum::{extract::FromRef, Router, Server};
use axum_extra::routing::SpaRouter;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use tower_http::trace::TraceLayer;

use crate::{
    configuration::{DatabaseSettings, EmailSettings, Settings},
    email::EmailClient,
    routes::api_router,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: DbState,
    pub email: EmailState,
}

#[derive(Clone, Debug)]
pub struct DbState {
    pub pool: PgPool,
}

impl FromRef<AppState> for DbState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.database.clone()
    }
}

#[derive(Clone, Debug)]
pub struct EmailState {
    pub email_client: EmailClient,
}

impl FromRef<AppState> for EmailState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.email.clone()
    }
}

pub struct Application {
    app: Router,
    pub sock_addr: SocketAddr,
}

impl Application {
    pub fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let pool = get_connection_pool(&config.database);
        let email_client = get_email_client(&config.email);

        let app_state = AppState {
            database: DbState { pool },
            email: EmailState { email_client },
        };

        let app = Router::new()
            // attach the api endpoints
            .nest("/api", api_router(app_state))
            // attach the frontend SPA
            .merge(SpaRouter::new("/assets", "./").index_file("index.html"))
            // attach tracing
            .layer(TraceLayer::new_for_http());

        let sock_addr = SocketAddr::from((
            IpAddr::from_str(&config.application.host).unwrap(),
            config.application.port,
        ));

        Ok(Self { app, sock_addr })
    }

    pub async fn run_until_stopped(self) -> Result<(), hyper::Error> {
        Server::bind(&self.sock_addr)
            .serve(self.app.into_make_service_with_connect_info::<SocketAddr>())
            .await
    }
}

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_lazy_with({
            PgConnectOptions::new()
                .host(&config.host)
                .username(&config.username)
                .password(&config.password)
                .port(config.port)
                .database(&config.database_name)
        })
}

pub fn get_email_client(config: &EmailSettings) -> EmailClient {
    EmailClient::new(&config)
}
