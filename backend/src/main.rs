mod apis;
mod config;
mod db;
mod errors;
mod helpers;
mod mail;
mod routes;
mod tasks;
mod templates;
mod types;

use std::error::Error;

use actix_cors::Cors;
use actix_web::{error, middleware, rt as actix_rt, web, App, HttpResponse, HttpServer};
use once_cell::sync::Lazy;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // initialise our global config, database pool and template engine structs
    pub static CONFIG: Lazy<config::Config> = Lazy::new(|| match config::Config::new() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    });

    // forces initialisation of the config, loading the environment variables to set up logging
    let _ = &*CONFIG;
    env_logger::init();

    pub static POOL: Lazy<db::Pool> = Lazy::new(|| {
        let pool = match db::init_pool(&CONFIG.database_url) {
            Ok(pool) => pool,
            Err(e) => {
                log::warn!("Error initialising database connection pool: {}", e);
                std::process::exit(1);
            }
        };
        pool
    });

    pub static TEMPLATES: Lazy<templates::Tera> = Lazy::new(|| {
        let template_engine = match templates::build_template_engine("src/templates/*.html") {
            Ok(eng) => eng,
            Err(e) => {
                log::warn!("Error compiling templates: {}", e);
                std::process::exit(1);
            }
        };
        template_engine
    });

    pub static MAILER: Lazy<mail::Transport> =
        Lazy::new(
            || match mail::build_mailer(&CONFIG.email_username, &CONFIG.email_password) {
                Ok(mailer) => mailer,
                Err(e) => {
                    eprintln!("Error building mailer: {}", e);
                    std::process::exit(1);
                }
            },
        );

    let pool = &*POOL;
    let template_engine = &*TEMPLATES;
    let mailer = &*MAILER;

    // start the never-ending alert task, which regularly checks for the latest aurora status and sends out
    // email alerts as necessary
    actix_rt::spawn(tasks::alert_task(pool, template_engine, mailer));

    // start the never-ending db maintenance task which deletes all unverified users every midnight
    actix_rt::spawn(tasks::clear_unverified_users_task(pool));

    HttpServer::new(move || {
        App::new()
            // middlewares
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allowed_origin("http://localhost"))
            // error handlers
            .app_data(web::QueryConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    err.source()
                        .map(|e| e.to_string())
                        .unwrap_or_else(|| String::from("")),
                    HttpResponse::BadRequest().json(errors::JsonErrorResponse {
                        error: String::from("Error parsing query parameters"),
                        context: err.to_string(),
                    }),
                )
                .into()
            }))
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    err.source()
                        .map(|e| e.to_string())
                        .unwrap_or_else(|| String::from("")),
                    HttpResponse::BadRequest().json(errors::JsonErrorResponse {
                        error: String::from("Error parsing JSON data"),
                        context: err.to_string(),
                    }),
                )
                .into()
            }))
            .app_data(web::PathConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    err.source()
                        .map(|e| e.to_string())
                        .unwrap_or_else(|| String::from("")),
                    HttpResponse::BadRequest().json(errors::JsonErrorResponse {
                        error: String::from("Error parsing path parameters"),
                        context: err.to_string(),
                    }),
                )
                .into()
            }))
            // application state
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(template_engine.clone()))
            .app_data(web::Data::new(mailer.clone()))
            // routes
            .service(routes::verify)
            .service(routes::register)
            .service(routes::unsubscribe)
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await?;

    Ok(())
}
