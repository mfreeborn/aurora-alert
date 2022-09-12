use std::fmt::Debug;

use gloo_net::http::{Method, Request};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;

async fn request<B, T>(method: Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = match method {
        Method::POST | Method::PUT => true,
        _ => false,
    };

    let mut builder = Request::new(&url)
        .method(method)
        .header("Content-Type", "application/json");

    if allow_body {
        builder = builder.json(&body).expect("Failed to attach json");
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.ok() {
            let data = data.json::<T>().await;
            if let Ok(data) = data {
                Ok(data)
            } else {
                Err(Error::DeserializeError)
            }
        } else {
            match data.status() {
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
    request(Method::GET, url, ()).await
}

pub async fn post<T, B>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + Debug + 'static,
    B: Serialize + Debug,
{
    request(Method::POST, url, body).await
}

pub async fn delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + Debug + 'static,
{
    request(Method::DELETE, url, ()).await
}
