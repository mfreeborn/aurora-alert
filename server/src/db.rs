use std::collections::HashMap;

use axum::extract::State as AxumState;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::apis;
use crate::apis::aurora_watch::{ActivityData, ActivityDataPoint};
use crate::types::{self, DateTimeUtc};

pub type DbPool = PgPool;
pub type State = AxumState<DbPool>;

/// Given an email address, return the associated user information.
pub async fn get_user_by_email(
    user: &RegisterUser,
    db: &DbPool,
) -> Result<Option<UserWithLocations>, anyhow::Error> {
    let user_locations = sqlx::query_as!(
        UserWithLocationModel,
        r#"
            SELECT
              users.user_id,
              users.email::TEXT as "email!",
              users.alert_threshold as "alert_threshold: types::AlertLevel",
              users.last_alerted_at as "last_alerted_at: DateTimeUtc",
              locations.location_id,
              locations.name::TEXT as "name!",
              locations.weather_description as "weather_description!",
              locations.cloud_cover,
              locations.updated_at as "updated_at: DateTimeUtc"
            FROM 
              users
            JOIN user_locations USING (user_id)
            JOIN locations USING (location_id)
            WHERE
              users.email = $1::TEXT::CITEXT
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
    pub location_id: i32,
    pub name: String,
    pub weather_description: String,
    pub cloud_cover: i16,
    pub updated_at: DateTimeUtc,
}

#[derive(Serialize)]
pub struct LocationName {
    pub location_id: i32,
    pub name: String,
}

/// Return a maximum of 5 locations whose name starts with `search_str`.
///
/// The locations are sorted alphabetically.
pub async fn get_locations(
    search_str: &types::SanitisedString,
    db: &DbPool,
) -> Result<Vec<LocationName>, anyhow::Error> {
    let locations = sqlx::query_as!(
        LocationName,
        r#"
            SELECT 
              location_id, name::TEXT as "name!"
            FROM 
              locations
            WHERE 
              name LIKE $1 || '%'
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
pub async fn set_user_verified(
    user_id: &Uuid,
    email: &str,
    db: &DbPool,
) -> Result<Option<Uuid>, anyhow::Error> {
    let user_id = sqlx::query_scalar!(
        r#"
            UPDATE 
              users
            SET 
              verified = true
            WHERE 
              user_id = $1 AND email = $2::TEXT::CITEXT
            RETURNING 
              user_id
        "#,
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
pub async fn delete_user(
    user_id: &Uuid,
    email: &str,
    db: &DbPool,
) -> Result<Option<Uuid>, anyhow::Error> {
    let user_id = sqlx::query_scalar!(
        "
            DELETE FROM 
              users
            WHERE 
              user_id = $1 AND email = $2::TEXT::CITEXT
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
    pub locations: Vec<i32>,
}

/// Create a new user record in the database, including associated locations.
pub async fn insert_user(
    user: &RegisterUser,
    db: &DbPool,
) -> Result<UserWithLocations, anyhow::Error> {
    let mut tx = db.begin().await?;

    let user_id = sqlx::query_scalar!(
        r#"
            INSERT INTO users 
              (email, alert_threshold)
            VALUES 
              ($1::TEXT::CITEXT, $2)
            RETURNING
              user_id
        "#,
        user.email,
        user.alert_threshold as types::AlertLevel
    )
    .fetch_one(&mut tx)
    .await?;

    for location in &user.locations {
        sqlx::query!(
            "
                INSERT INTO user_locations
                  (user_id, location_id)
                VALUES 
                  ($1, $2)
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
            users.email::TEXT as "email!",
            users.alert_threshold as "alert_threshold: types::AlertLevel",
            users.last_alerted_at as "last_alerted_at: DateTimeUtc",
            locations.location_id,
            locations.name::TEXT as "name!",
            locations.weather_description as "weather_description!",
            locations.cloud_cover,
            locations.updated_at as "updated_at: DateTimeUtc"
          FROM 
            users
          JOIN user_locations USING (user_id)
          JOIN locations USING (location_id)
          WHERE
            users.user_id = $1
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
pub async fn get_alert_level(pool: &DbPool) -> Result<AlertLevelModel, anyhow::Error> {
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
    .fetch_one(pool)
    .await?;

    Ok(alert_level)
}

/// Update the existing alert level stored in the database with a new alert level.
pub async fn update_alert_level(
    new_alert_level: &apis::aurora_watch::CurrentAlertLevel,
    pool: &DbPool,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        "
            UPDATE
              alert_level
            SET
              alert_level = $1,
              updated_at = $2
            WHERE
              alert_level_id = 1
        ",
        new_alert_level.level as types::AlertLevel,
        new_alert_level.updated_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize, Clone)]
pub struct LocationUpdatedAt {
    pub location_id: i32,
    pub updated_at: DateTimeUtc,
}

/// Retrieve a set of all locations which are associated with at least one user.
pub async fn get_unique_user_locations(
    pool: &DbPool,
) -> Result<Vec<LocationUpdatedAt>, anyhow::Error> {
    let locations = sqlx::query_as!(
        LocationUpdatedAt,
        r#"
            SELECT DISTINCT
              locations.location_id,
              locations.updated_at as "updated_at: DateTimeUtc"
            FROM
              user_locations INNER JOIN locations USING (location_id)
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(locations)
}

/// Update the weather report at the given location.
pub async fn update_weather(
    weather: apis::open_weather::Weather,
    location_id: i32,
    pool: &DbPool,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        "
            UPDATE
              locations
            SET
              weather_description = $1,
              cloud_cover = $2
            WHERE
              location_id = $3
        ",
        weather.description,
        weather.cloud_cover,
        location_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Set the `last_updated_at` field for the given user to now.
pub async fn update_user_last_alerted_at(
    user_id: &Uuid,
    pool: &DbPool,
) -> Result<(), anyhow::Error> {
    let now = chrono::Utc::now();
    sqlx::query!(
        "
            UPDATE
              users
            SET
              last_alerted_at = $1
            WHERE 
              user_id = $2
        ",
        now,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Serialize)]
pub struct UserWithLocations {
    pub user_id: Uuid,
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
    user_id: Uuid,
    email: String,
    alert_threshold: types::AlertLevel,
    last_alerted_at: Option<DateTimeUtc>,
    location_id: i32,
    name: String,
    weather_description: String,
    cloud_cover: i16,
    updated_at: DateTimeUtc,
}

/// Return a list of all verified users.
pub async fn get_verified_users(pool: &DbPool) -> Result<Vec<UserWithLocations>, anyhow::Error> {
    let users = sqlx::query_as!(
        UserWithLocationModel,
        r#"
            SELECT
              users.user_id,
              users.email::TEXT as "email!",
              users.alert_threshold as "alert_threshold: types:: AlertLevel",
              users.last_alerted_at as "last_alerted_at: DateTimeUtc",
              locations.location_id,
              locations.name::TEXT as "name!",
              locations.weather_description,
              locations.cloud_cover,
              locations.updated_at as "updated_at: DateTimeUtc"
            FROM
              users
            JOIN user_locations USING (user_id)
            JOIN locations USING (location_id)
            WHERE 
              users.verified = true
        "#
    )
    .fetch_all(pool)
    .await?;

    let users = UserWithLocations::many_from_rows(users);

    Ok(users)
}

/// Delete all unverified users from the database, returning a count of how many were deleted if successful.
pub async fn delete_unverified_users(pool: &DbPool) -> Result<u64, anyhow::Error> {
    let deleted_users_count = sqlx::query!(
        r#"
            DELETE FROM 
              users
            WHERE 
              NOT verified
        "#,
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(deleted_users_count)
}

/// Store the latest aurora activity data.
pub async fn update_aurora_activity(
    activity: super::apis::aurora_watch::ActivityData,
    pool: &DbPool,
) -> Result<(), anyhow::Error> {
    let mut tx = pool.begin().await?;
    let acts = activity.activities;

    #[rustfmt::skip]
    sqlx::query!(
        "
            INSERT INTO activity_data
              (timestamp, value)
            VALUES
              ($1, $2), ($3, $4),  -- 2
              ($5, $6), ($7, $8),  -- 4
              ($9, $10), ($11, $12),  -- 6
              ($13, $14), ($15, $16),  -- 8
              ($17, $18), ($19, $20),  -- 10
              ($21, $22), ($23, $24),  -- 12
              ($25, $26), ($27, $28),  -- 14
              ($29, $30), ($31, $32),  -- 16
              ($33, $34), ($35, $36),  -- 18
              ($37, $38), ($39, $40),  -- 20
              ($41, $42), ($43, $44),  -- 22
              ($45, $46), ($47, $48)   -- 24
            ON CONFLICT (timestamp) DO UPDATE SET value = EXCLUDED.value
        ",
        // eugh!
        acts[0].timestamp,  acts[0].value,  acts[1].timestamp,  acts[1].value,
        acts[2].timestamp,  acts[2].value,  acts[3].timestamp,  acts[3].value,
        acts[4].timestamp,  acts[4].value,  acts[5].timestamp,  acts[5].value,
        acts[6].timestamp,  acts[6].value,  acts[7].timestamp,  acts[7].value,
        acts[8].timestamp,  acts[8].value,  acts[9].timestamp,  acts[9].value,
        acts[10].timestamp, acts[10].value, acts[11].timestamp, acts[11].value,
        acts[12].timestamp, acts[12].value, acts[13].timestamp, acts[13].value,
        acts[14].timestamp, acts[14].value, acts[15].timestamp, acts[15].value,
        acts[16].timestamp, acts[16].value, acts[17].timestamp, acts[17].value,
        acts[18].timestamp, acts[18].value, acts[19].timestamp, acts[19].value,
        acts[20].timestamp, acts[20].value, acts[21].timestamp, acts[21].value,
        acts[22].timestamp, acts[22].value, acts[23].timestamp, acts[23].value,
    )
    .execute(&mut tx)
    .await?;

    sqlx::query!(
        "
            INSERT INTO activity_data_meta
              (activity_data_meta_id, updated_at)
            VALUES
              (1, $1)
            ON CONFLICT (activity_data_meta_id) DO UPDATE SET updated_at = EXCLUDED.updated_at
        ",
        activity.updated_at
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

/// Retrieve the activity data for the 24 hours ending at `end`.
#[tracing::instrument()]
pub async fn get_activity_data(
    end: &DateTimeUtc,
    pool: &DbPool,
) -> Result<ActivityData, anyhow::Error> {
    let mut tx = pool.begin().await?;

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

    let activities = sqlx::query_as!(
        ActivityDataPoint,
        r#"
            SELECT 
              timestamps.timestamp as "timestamp!",
              COALESCE(activity_data.value, 0.0) as "value!"
            FROM 
              (
                SELECT timestamp
                FROM generate_series($1::timestamptz - '23 hours'::interval, $1::timestamptz, '1 hour'::interval) t(timestamp)
              ) as timestamps
            LEFT OUTER JOIN
              activity_data ON timestamps.timestamp = activity_data.timestamp
            ORDER BY
            timestamps.timestamp DESC;
        "#,
        end
    )
    .fetch_all(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(ActivityData {
        updated_at,
        // Safe to unwrap because our generate_series function will always return get 24 rows,
        // and we coalesce null (non-existent) values to 0.0.
        activities: activities.try_into().unwrap(),
    })
}
