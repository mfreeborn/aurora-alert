use crate::error::Error;
use crate::requests;
use crate::types::locations::LocationsBody;

pub async fn get_locations(
    search_string: String,
) -> Result<std::collections::HashMap<String, i64>, Error> {
    Ok(
        requests::get::<LocationsBody>(format!("/api/locations?search_str={search_string}"))
            .await?
            .locations
            .iter()
            .map(|loc| (loc.name.clone(), loc.location_id))
            .collect::<std::collections::HashMap<String, i64>>(),
    )
}
