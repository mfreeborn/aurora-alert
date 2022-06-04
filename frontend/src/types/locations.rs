use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LocationsBody {
    pub locations: Vec<Location>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Location {
    pub location_id: i64,
    pub name: String,
}
