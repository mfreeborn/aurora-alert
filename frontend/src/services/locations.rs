use crate::error::Error;
use crate::requests;
use crate::types::locations::Location;

pub async fn get_locations(
    search_string: String,
) -> Result<std::collections::HashMap<String, i64>, Error> {
    Ok(
        requests::get::<Vec<Location>>(format!("/locations?search={search_string}"))
            .await?
            .iter()
            .map(|loc| (loc.name.clone(), loc.location_id))
            .collect::<std::collections::HashMap<String, i64>>(),
    )
}
