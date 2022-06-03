use serde::Deserialize;

type Result<T> = std::result::Result<T, OpenWeatherError>;

#[derive(Debug, thiserror::Error)]
pub enum OpenWeatherError {
    #[error("error")]
    Request(#[from] reqwest::Error),
}

#[derive(Debug, Deserialize)]
struct Clouds {
    all: u8,
}

#[derive(Debug, Deserialize)]
struct WeatherDetail {
    description: String,
}

#[derive(Debug, Deserialize)]
struct WeatherBody {
    // Cloud cover is reported as an integer percentage.
    clouds: Clouds,
    // There is always at least 1 primary weather field, plus optional extras.
    weather: Vec<WeatherDetail>,
    // What OpenWeather calls the city_id is an unsigned integer which fits within a u32.
    id: u32,
}

pub struct Weather {
    pub location_id: u32,
    pub description: String,
    pub cloud_cover: u8,
}

impl std::convert::From<WeatherBody> for Weather {
    fn from(weather_body: WeatherBody) -> Self {
        Self {
            location_id: weather_body.id,
            // OpenWeather returns a weather field which contains 1 or more entries, with the
            // first entry being the primary weather conditions. There should always, therefore,
            // be a `.first()` entry.
            description: weather_body.weather.first().unwrap().description.clone(),
            cloud_cover: weather_body.clouds.all,
        }
    }
}

pub async fn get_weather(location_id: u32, api_key: &str) -> Result<Weather> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?id={}&appid={}",
        location_id, api_key
    );

    let current_weather: Weather = reqwest::get(url).await?.json::<WeatherBody>().await?.into();
    Ok(current_weather)
}
