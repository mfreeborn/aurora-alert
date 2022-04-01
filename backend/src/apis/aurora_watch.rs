use anyhow;
use quick_xml;

use crate::types;

pub async fn get_alert_level() -> anyhow::Result<types::CurrentStatus> {
    let status_url = "https://aurorawatch-api.lancs.ac.uk/0.2/status/current-status.xml";
    let xml_response = reqwest::get(status_url).await?.text().await?;
    let status: types::CurrentStatus = quick_xml::de::from_str(&xml_response)?;
    Ok(status)
}
