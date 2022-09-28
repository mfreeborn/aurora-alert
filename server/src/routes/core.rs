use axum::{
    extract::{Query, State},
    Json,
};
use axum::{routing::get, Router};
use chrono::Timelike;
use serde::Deserialize;
use serde::Serialize;

use crate::common::ActivityData;
use crate::error::Error;
use crate::startup::AppState;
use crate::types::{DateTimeUtc, SanitisedString};
use crate::{db, startup::DbState};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::with_state(app_state)
        .route("/activity", get(activity))
        .route("/locations", get(locations))
}

#[derive(Deserialize)]
struct LocationQuery {
    search_str: SanitisedString,
}

#[derive(Serialize)]
struct LocationsBody {
    locations: Vec<db::LocationName>,
}

/// Given a search string, return an alphabetical list of the locations which
/// begin with said string.
///
/// If the search string is empty, then an empty list is returned, otherwise, up
/// to a maximum of 5 locations will be returned.
#[tracing::instrument(name = "Fetch locations")]
async fn locations(
    Query(LocationQuery { search_str }): Query<LocationQuery>,
    State(db): State<DbState>,
) -> Result<Json<LocationsBody>, Error> {
    // Short circuit if the search string is blank. Alternatively, we could allow it
    // to proceed, and that would mean we return the first LIMIT locations,
    // rather than none.
    if search_str.is_empty() {
        return Ok(Json(LocationsBody { locations: vec![] }));
    }

    let locations = db::get_locations(&search_str, &db.pool).await?;
    Ok(Json(LocationsBody { locations }))
}

#[derive(Deserialize)]
struct ActivityQuery {
    end: Option<DateTimeUtc>,
}

#[derive(Serialize)]
struct ActivityBody {
    #[serde(flatten)]
    activity_data: ActivityData,
}

/// Return the hourly activity data for the 24 hours ending at the given `end`.
///
/// If `end` is None, then the data for the latest 24 hours is returned.
#[tracing::instrument(name = "Fetch activity data")]
async fn activity(
    Query(ActivityQuery { end }): Query<ActivityQuery>,
    State(db): State<DbState>,
) -> Result<Json<ActivityBody>, Error> {
    let end = end.unwrap_or_else(|| {
        let now = chrono::Utc::now();
        now.date().and_hms(now.time().hour(), 0, 0)
    });

    let activity_data = db::get_activity_data(&end, &db.pool).await?;

    Ok(Json(ActivityBody { activity_data }))
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::app;

    #[tokio::test]
    async fn test_get_activity_exists() {
        // let app = app();
    }

    #[tokio::test]
    async fn test_get_locations_exists() {
        // let app = app();
    }
}
