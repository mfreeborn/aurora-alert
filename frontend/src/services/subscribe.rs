use std::fmt;

use gloo_console as console;
use reqwest;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

pub async fn subscribe() -> Result<SiteStatus, MyError> {
    let response = reqwest::Client::new()
        .get("http://0.0.0.0:9090/alert-level")
        .send()
        .await;
    println!("{:#?}", response);
    if let Ok(data) = response {
        console::log!("hi");
        let d = data.json::<SiteStatus>().await.unwrap();
        Ok(d)
    } else {
        Err(MyError::RequestError)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MyError {
    RequestError,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SiteStatus {
    #[serde(rename = "status_id")]
    alert_level: AlertLevel,
    project_id: String,
    site_id: String,
    site_url: String,
}

impl fmt::Display for SiteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SiteStatus(alert_level: {:?}, project_id: {}, site_id: {}, site_url: {})",
            self.alert_level, self.project_id, self.site_id, self.site_url
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AlertLevel {
    Green,
    Yellow,
    Amber,
    Red,
}
