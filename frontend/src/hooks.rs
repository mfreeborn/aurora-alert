use serde::de::DeserializeOwned;
use yew_hooks::use_location;

pub fn use_query_params<T>() -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let location = use_location();
    let raw_params = location.search.strip_prefix('?').unwrap_or("");
    let parsed_params = serde_urlencoded::from_str(raw_params)?;

    Ok(parsed_params)
}
