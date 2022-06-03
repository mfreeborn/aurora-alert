use anyhow::{self, Context};
use dotenv::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub templates_dir: String,
    pub open_weather_api_key: String,
    pub email_username: String,
    pub email_password: String,
}

impl Config {
    pub fn init() -> anyhow::Result<Self> {
        let err_context = "is a required variable";

        // necessary for composing DATABASE_URL
        let workspace_root = var("CARGO_WORKSPACE_DIRECTORY")
            .with_context(|| format!("{} {}", "CARGO_WORKSPACE_DIRECTORY", err_context))?;
        let database_url =
            var("DATABASE_URL").with_context(|| format!("{} {}", "DATABASE_URL", err_context))?;
        let templates_dir = var("TEMPLATES_DIR")
            .unwrap_or_else(|_| format!("{workspace_root}backend/src/templates"));
        let open_weather_api_key = var("OPEN_WEATHER_API_KEY")
            .with_context(|| format!("{} {}", "OPEN_WEATHER_API_KEY", err_context))?;
        let email_username = var("EMAIL_USERNAME")
            .with_context(|| format!("{} {}", "EMAIL_USERNAME", err_context))?;
        let email_password = var("EMAIL_PASSWORD")
            .with_context(|| format!("{} {}", "EMAIL_PASSWORD", err_context))?;

        Ok(Self {
            database_url,
            templates_dir,
            open_weather_api_key,
            email_username,
            email_password,
        })
    }
}
