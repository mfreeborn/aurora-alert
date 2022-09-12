use config::ConfigError;

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment.
    // Default to `dev` if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    let environment_filename = format!("{}.toml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.toml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(&environment_filename),
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub email: EmailSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub open_weather_api_key: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailSettings {
    pub username: String,
    pub password: String,
    pub templates_dir: String,
}

/// The possible runtime environment for our application.
pub enum Environment {
    Dev,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Dev => "dev",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `dev` or `production`.",
                other
            )),
        }
    }
}
