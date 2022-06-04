use anyhow::Context;
use aurora_alert_backend::{config, db, email, serve, tasks, templates};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // A reasonable amount of set up is required for running the application. Specifically, we need to
    // carry out the following steps:
    //     // Steps 1-3 need to be carried out in this order
    //     1. Load the environement variables
    //     2. Initialise logging
    //     3. Parse the rest of the application configuration from the environment
    //     // Steps 4 - 6 can be carried out in any order
    //     4. Initialise the database pool
    //     5. Initialise the template engine
    //     6. Initialise the the email engine
    //     // Starting off the background tasks comes after app configuration and before the app gets served
    //     7. Commence tasks
    //     // The final step
    //     8. Build and serve the application
    //
    // If any of steps 1 - 6 fail, then the application will exit with an error message.

    // Steps 1 + 2 - Load the environment variables and initialise logging
    dotenv::from_filename("./backend/.env").ok();
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                eprintln!("warning - RUST_LOG environment variable not found; setting default log level to 'debug'");
                "debug".into()
            }),
        ))
        .with(fmt::layer())
        .init();

    // Step 3 - Parse the application configuration from the environment
    let config = config::Config::init()?;

    // Step 4 - Initialise the database pool
    let db = db::init(&config.database_url)
        .await
        .context("could not connect to the database")?;

    // Step 5 - Initialise the template engine
    let template_engine =
        templates::init(&config.templates_dir).context("could not create the template engine")?;

    // Step 6 - Initialise the the email engine
    let email_transport = email::init(&config.email_username, &config.email_password)
        .context("could not create the email transport")?;

    // Step 7 - Commence long-running tasks
    tasks::init(&db, &template_engine, &email_transport, &config);

    // Step 8 - Build and serve the applicatio
    serve(config, db, template_engine, email_transport).await?;

    Ok(())
}
