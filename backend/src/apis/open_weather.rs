use crate::types;

use anyhow;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentWeather {
    pub coord: Coord,
    pub clouds: Clouds,
    pub weather: Vec<Weather>,
}

pub async fn get_weather(location: types::Location) -> anyhow::Result<CurrentWeather> {
    let api_key = std::env::var("OPEN_WEATHER_API_KEY")
        .expect("Open Weather API key is required to be set as an environment variable");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?id={}&appid={}",
        location.city_id(),
        api_key
    );
    let current_weather: CurrentWeather = reqwest::get(url).await?.json().await?;
    Ok(current_weather)
}
