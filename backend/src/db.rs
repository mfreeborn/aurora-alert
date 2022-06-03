use std::collections::HashMap;

use axum::Extension as AxumExtension;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::apis;
use crate::types::{self, DateTimeUtc};
use crate::Result;

pub type DbPool = SqlitePool;
pub type Extension = AxumExtension<DbPool>;

/// Create a new connection pool for the SQLite database at the given url.
///
/// During initialisation, the migrations are applied to ensure that the app
/// is always running against an up to date schema.
pub async fn init(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Validate the database structure +/- apply any new changes.
    sqlx::migrate!().run(&db).await?;

    Ok(db)
}

/// Given an email address, return the associated user information.
pub async fn get_user_by_email(
    user: &RegisterUser,
    db: &DbPool,
) -> Result<Option<UserWithLocations>> {
    let user_locations = sqlx::query_as!(
        UserWithLocationModel,
        r#"
            SELECT
              users.user_id,
              users.email,
              users.alert_threshold as "alert_threshold: types::AlertLevel",
              users.last_alerted_at as "last_alerted_at: DateTimeUtc",
              locations.location_id as "location_id: u32",
              locations.name,
              locations.weather_description,
              locations.cloud_cover as "cloud_cover: u8",
              locations.updated_at as "updated_at: DateTimeUtc"
            FROM 
              users, locations, user_locations
            WHERE
              users.email = ?
            AND 
              users.user_id = user_locations.user_id
            AND
              locations.location_id = user_locations.location_id
        "#,
        user.email
    )
    .fetch_all(db)
    .await?;

    if !user_locations.is_empty() {
        Ok(Some(UserWithLocations::one_from_rows(user_locations)))
    } else {
        Ok(None)
    }
}

#[derive(Serialize)]
pub struct Location {
    pub location_id: u32,
    pub name: String,
    pub weather_description: String,
    pub cloud_cover: u8,
    pub updated_at: DateTimeUtc,
}

#[derive(Serialize)]
pub struct LocationName {
    pub location_id: u32,
    pub name: String,
}

/// Return a maximum of 5 locations whose name starts with `search_str`.
///
/// The locations are sorted alphabetically.
pub async fn get_locations(
    search_str: &types::SanitisedString,
    db: &DbPool,
) -> Result<Vec<LocationName>> {
    let locations = sqlx::query_as!(
        LocationName,
        r#"
            SELECT 
              location_id as "location_id: u32", name
            FROM 
              locations
            WHERE 
              name LIKE ? || '%'
            ORDER BY 
              name DESC, location_id ASC
            LIMIT 
              5
        "#,
        search_str
    )
    .fetch_all(db)
    .await?;
    Ok(locations)
}

/// Mark the user with the given identifiers as verified.
///
/// The successful value is `Some(user_id)` if the user was found and updated, otherwise `None` not found.
pub async fn set_user_verified(user_id: &str, email: &str, db: &DbPool) -> Result<Option<String>> {
    let user_id = sqlx::query_scalar!(
        "
            UPDATE 
              users
            SET 
              verified = 1
            WHERE 
              user_id = ? AND email = ?
            RETURNING 
              user_id
        ",
        user_id,
        email
    )
    .fetch_optional(db)
    .await?;

    Ok(user_id)
}

/// Permanently remove the user with the given identifiers from the database.
///
/// The successful value is `Some(user_id)` if a user was found and deleted, otherwise it is `None`.
pub async fn delete_user(user_id: &str, email: &str, db: &DbPool) -> Result<Option<String>> {
    let user_id = sqlx::query_scalar!(
        "
            DELETE FROM 
              users
            WHERE 
              user_id = ? AND email = ?
            RETURNING 
              user_id
        ",
        user_id,
        email
    )
    .fetch_optional(db)
    .await?;

    Ok(user_id)
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub alert_threshold: types::AlertLevel,
    pub locations: Vec<u32>,
}

/// Create a new user record in the database, including associated locations.
pub async fn insert_user(user: &RegisterUser, db: &DbPool) -> Result<UserWithLocations> {
    let mut tx = db.begin().await?;

    let user_id = sqlx::query_scalar!(
        "
            INSERT INTO users 
              (email, alert_threshold)
            VALUES 
              (?, ?)
            RETURNING
              user_id
        ",
        user.email,
        user.alert_threshold
    )
    .fetch_one(&mut tx)
    .await?;

    for location in &user.locations {
        sqlx::query!(
            "
                INSERT INTO user_locations
                  (user_id, location_id)
                VALUES 
                  (?, ?)
            ",
            user_id,
            location
        )
        .execute(&mut tx)
        .await?;
    }

    let user_locations = sqlx::query_as!(
        UserWithLocationModel,
        r#"
            SELECT
              users.user_id,
              users.email,
              users.alert_threshold as "alert_threshold: types::AlertLevel",
              users.last_alerted_at as "last_alerted_at: DateTimeUtc",
              locations.location_id as "location_id: u32",
              locations.name,
              locations.weather_description,
              locations.cloud_cover as "cloud_cover: u8",
              locations.updated_at as "updated_at: DateTimeUtc"
            FROM 
              users, locations, user_locations
            WHERE
              users.user_id = ?
            AND 
              users.user_id = user_locations.user_id
            AND
              locations.location_id = user_locations.location_id
        "#,
        user_id
    )
    .fetch_all(&mut tx)
    .await?;

    let user = UserWithLocations::one_from_rows(user_locations);

    tx.commit().await?;

    Ok(user)
}

#[derive(Debug)]
pub struct AlertLevelModel {
    pub alert_level: types::AlertLevel,
    pub updated_at: DateTimeUtc,
}

/// Retrieve the stored alert level from the database.
pub async fn get_alert_level(db: &DbPool) -> Result<AlertLevelModel> {
    let alert_level = sqlx::query_as!(
        AlertLevelModel,
        r#"
            SELECT
              alert_level as "alert_level: types::AlertLevel",
              updated_at as "updated_at: DateTimeUtc"
            FROM
              alert_level
            WHERE
              alert_level_id = 1
        "#
    )
    .fetch_one(db)
    .await?;

    Ok(alert_level)
}

/// Update the existing alert level stored in the database with a new alert level.
pub async fn update_alert_level(
    new_alert_level: &apis::aurora_watch::CurrentAlertLevel,
    db: &DbPool,
) -> Result<()> {
    sqlx::query!(
        "
            UPDATE
              alert_level
            SET
              alert_level = ?,
              updated_at = ?
            WHERE
              alert_level_id = 1
        ",
        new_alert_level.level,
        new_alert_level.updated_at
    )
    .execute(db)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize, Clone)]
pub struct LocationUpdatedAt {
    pub location_id: u32,
    pub updated_at: DateTimeUtc,
}

/// Retrieve a set of all locations which are associated with at least one user.
pub async fn get_unique_user_locations(db: &DbPool) -> Result<Vec<LocationUpdatedAt>> {
    let locations = sqlx::query_as!(
        LocationUpdatedAt,
        r#"
            SELECT DISTINCT
              locations.location_id as "location_id: u32",
              locations.updated_at as "updated_at: DateTimeUtc"
            FROM
              user_locations INNER JOIN locations USING (location_id)
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(locations)
}

/// Update the weather report at the given location.
pub async fn update_weather(
    weather: apis::open_weather::Weather,
    location_id: u32,
    db: &DbPool,
) -> Result<()> {
    sqlx::query!(
        "
            UPDATE
              locations
            SET
              weather_description = ?,
              cloud_cover = ?
            WHERE
              location_id = ?
        ",
        weather.description,
        weather.cloud_cover,
        location_id
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Set the `last_updated_at` field for the given user to now.
pub async fn update_user_last_alerted_at(user_id: &str, db: &DbPool) -> Result<()> {
    let now = chrono::Utc::now();
    sqlx::query!(
        "
            UPDATE
              users
            SET
              last_alerted_at = ?
            WHERE 
              user_id = ?
        ",
        now,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

#[derive(Serialize)]
pub struct UserWithLocations {
    pub user_id: String,
    pub email: String,
    pub alert_threshold: types::AlertLevel,
    pub last_alerted_at: Option<DateTimeUtc>,
    pub locations: Vec<Location>,
}

impl UserWithLocations {
    fn one_from_rows(user_rows: Vec<UserWithLocationModel>) -> Self {
        let mut locations = vec![];
        for user_row in &user_rows {
            let location = Location {
                location_id: user_row.location_id,
                name: user_row.name.clone(),
                weather_description: user_row.weather_description.clone(),
                cloud_cover: user_row.cloud_cover,
                updated_at: user_row.updated_at,
            };
            locations.push(location);
        }
        let row = &user_rows[0];

        UserWithLocations {
            user_id: row.user_id.clone(),
            email: row.email.clone(),
            alert_threshold: row.alert_threshold.clone(),
            last_alerted_at: row.last_alerted_at,
            locations,
        }
    }

    fn many_from_rows(users: Vec<UserWithLocationModel>) -> Vec<Self> {
        let mut user_map = HashMap::new();
        for user in &users {
            let user_entry = user_map.entry(&user.user_id).or_insert(UserWithLocations {
                user_id: user.user_id.clone(),
                email: user.email.clone(),
                alert_threshold: user.alert_threshold.clone(),
                last_alerted_at: user.last_alerted_at,
                locations: vec![],
            });
            let location = Location {
                location_id: user.location_id,
                name: user.name.clone(),
                weather_description: user.weather_description.clone(),
                cloud_cover: user.cloud_cover,
                updated_at: user.updated_at,
            };
            user_entry.locations.push(location);
        }
        user_map
            .into_iter()
            .map(|(_user_id, user_location)| user_location)
            .collect()
    }
}

struct UserWithLocationModel {
    user_id: String,
    email: String,
    alert_threshold: types::AlertLevel,
    last_alerted_at: Option<DateTimeUtc>,
    location_id: u32,
    name: String,
    weather_description: String,
    cloud_cover: u8,
    updated_at: DateTimeUtc,
}

/// Return a list of all verified users.
pub async fn get_verified_users(db: &DbPool) -> Result<Vec<UserWithLocations>> {
    let users = sqlx::query_as!(
        UserWithLocationModel,
        r#"
            SELECT
              users.user_id,
              users.email,
              users.alert_threshold as "alert_threshold: types:: AlertLevel",
              users.last_alerted_at as "last_alerted_at: DateTimeUtc",
              locations.location_id as "location_id: u32",
              locations.name,
              locations.weather_description,
              locations.cloud_cover as "cloud_cover: u8",
              locations.updated_at as "updated_at: DateTimeUtc"
            FROM
              users, locations, user_locations
            WHERE 
              users.user_id = user_locations.user_id
            AND 
              locations.location_id = user_locations.location_id
            AND 
              users.verified = 1
        "#
    )
    .fetch_all(db)
    .await?;

    let users = UserWithLocations::many_from_rows(users);

    Ok(users)
}

/// Delete all unverified users from the database, returning a count of how many were deleted if successful.
pub async fn delete_unverified_users(db: &DbPool) -> Result<u64> {
    let deleted_users_count = sqlx::query!(
        r#"
            DELETE FROM 
              users
            WHERE 
              NOT verified
        "#,
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(deleted_users_count)
}

/// Store the latest aurora activity data.
pub async fn update_aurora_activity(
    activity: super::apis::aurora_watch::ActivityData,
    db: &DbPool,
) -> Result<()> {
    let mut tx = db.begin().await?;
    let acts = activity.activities;
    #[rustfmt::skip]
    sqlx::query!(
        "
            INSERT INTO activity_data
              (datetime, value)
            VALUES
              (?, ?), (?, ?),  -- 2
              (?, ?), (?, ?),  -- 4
              (?, ?), (?, ?),  -- 6
              (?, ?), (?, ?),  -- 8
              (?, ?), (?, ?),  -- 10
              (?, ?), (?, ?),  -- 12
              (?, ?), (?, ?),  -- 14
              (?, ?), (?, ?),  -- 16
              (?, ?), (?, ?),  -- 18
              (?, ?), (?, ?),  -- 20
              (?, ?), (?, ?),  -- 22
              (?, ?), (?, ?)   -- 24
        ",
        // eugh!
        acts[0].datetime,  acts[0].value,  acts[1].datetime,  acts[1].value,
        acts[2].datetime,  acts[2].value,  acts[3].datetime,  acts[3].value,
        acts[4].datetime,  acts[4].value,  acts[5].datetime,  acts[5].value,
        acts[6].datetime,  acts[6].value,  acts[7].datetime,  acts[7].value,
        acts[8].datetime,  acts[8].value,  acts[9].datetime,  acts[9].value,
        acts[10].datetime, acts[10].value, acts[11].datetime, acts[11].value,
        acts[12].datetime, acts[12].value, acts[13].datetime, acts[13].value,
        acts[14].datetime, acts[14].value, acts[15].datetime, acts[15].value,
        acts[16].datetime, acts[16].value, acts[17].datetime, acts[17].value,
        acts[18].datetime, acts[18].value, acts[19].datetime, acts[19].value,
        acts[20].datetime, acts[20].value, acts[21].datetime, acts[21].value,
        acts[22].datetime, acts[22].value, acts[23].datetime, acts[23].value,
    )
    .execute(&mut tx)
    .await?;

    sqlx::query!(
        "
            UPDATE
               activity_data_meta
            SET
              updated_at = ?
            WHERE
              activity_data_meta_id = 1
        ",
        activity.updated_at
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

/// Retrieve the activity data for the 24 hours ending at `end`.
pub async fn get_activity_data(
    end: &DateTimeUtc,
    db: &DbPool,
) -> Result<super::apis::aurora_watch::ActivityData> {
    let mut tx = db.begin().await?;

    let updated_at = sqlx::query_scalar!(
        r#"
            SELECT 
              updated_at as "updated_at: DateTimeUtc"
            FROM 
              activity_data_meta
            WHERE 
              activity_data_meta_id = 1
        "#
    )
    .fetch_one(&mut tx)
    .await?;

    let activities = sqlx::query_as::<_, super::apis::aurora_watch::ActivityDataPoint>(
        r#"
            WITH RECURSIVE datetimes(datetime) AS (
              VALUES(?)
              UNION ALL
              SELECT datetime(datetime, '-1 hour')
              FROM datetimes
              WHERE datetime > datetime(?, '-23 hour')
            )
            SELECT 
              datetimes.datetime ,
              activity_data.value
            FROM 
              datetimes
            LEFT OUTER JOIN
              activity_data ON datetimes.datetime = activity_data.datetime
            ORDER BY
              datetimes.datetime DESC
        "#,
    )
    .bind(end)
    .bind(end)
    .fetch_all(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(apis::aurora_watch::ActivityData {
        updated_at,
        // Safe to unwrap because we have used a CTE to ensure we always get 24 rows.
        activities: activities.try_into().unwrap(),
    })
}
