use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;

async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("http://0.0.0.0:9090{url}");
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            let data = data.json::<T>().await;
            if let Ok(data) = data {
                Ok(data)
            } else {
                Err(Error::DeserializeError)
            }
        } else {
            match data.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}

pub async fn get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + Debug + 'static,
{
    request(reqwest::Method::GET, url, ()).await
}

pub async fn post<T, B>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + Debug + 'static,
    B: Serialize + Debug,
{
    request(reqwest::Method::POST, url, body).await
}

pub async fn delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + Debug + 'static,
{
    request(reqwest::Method::DELETE, url, ()).await
}
