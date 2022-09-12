pub mod apis;
pub mod configuration;
pub mod db;
pub mod email;
pub mod error;
pub mod helpers;
pub mod routes;
pub mod startup;
pub mod tasks;
pub mod telemetry;
pub mod templates;
pub mod types;

// pub async fn serve(
//     config: Config,
//     db: DbPool,
//     template_engine: TemplateEngine,
//     email_transport: EmailTransport,
// ) -> anyhow::Result<()> {
//     let addr = SocketAddr::from(([0, 0, 0, 0], 9090));

//     tracing::info!("listening on http://{}", addr);
//     Server::bind(&addr)
//         .serve(app(config, db, template_engine, email_transport).into_make_service())
//         .await
//         .context("error running HTTP server")
// }

// fn app(
//     config: Config,
//     db: DbPool,
//     template_engine: TemplateEngine,
//     email_transport: EmailTransport,
// ) -> Router {
//     api_router()
//         // Add middleware to all routes
//         .layer(
//             ServiceBuilder::new()
//                 .layer(TraceLayer::new_for_http())
//                 .layer(CorsLayer::permissive())
//                 .layer(Extension(db))
//                 .layer(Extension(template_engine))
//                 .layer(Extension(email_transport))
//                 .layer(Extension(config)),
//         )
// }

// fn api_router() -> Router {
//     routes::users::router().merge(routes::core::router())
// }
