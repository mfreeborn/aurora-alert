use anyhow::{self, Context};
use dotenv::var;

pub struct Config {
    pub database_url: String,
    pub open_weather_api_key: String,
    pub email_username: String,
    pub email_password: String,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let err_context = "is a required variable";
        let database_url =
            var("DATABASE_URL").with_context(|| format!("{} {}", "DATABASE_URL", err_context))?;
        let open_weather_api_key = var("OPEN_WEATHER_API_KEY")
            .with_context(|| format!("{} {}", "OPEN_WEATHER_API_KEY", err_context))?;
        let email_username = var("EMAIL_USERNAME")
            .with_context(|| format!("{} {}", "EMAIL_USERNAME", err_context))?;
        let email_password = var("EMAIL_PASSWORD")
            .with_context(|| format!("{} {}", "EMAIL_PASSWORD", err_context))?;

        Ok(Self {
            database_url,
            open_weather_api_key,
            email_username,
            email_password,
        })
    }
}
