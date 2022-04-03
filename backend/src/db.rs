use std::collections::HashMap;

use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::{self, sqlite::SqlitePoolOptions, SqlitePool};

use crate::apis;
use crate::types;

pub type Pool = SqlitePool;
pub type Extractor = web::Data<Pool>;

pub fn init_pool(database_url: &str) -> Result<Pool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_lazy(database_url)
}

#[derive(Serialize)]
pub struct LocationNameModel {
    pub location_id: i64,
    pub name: String,
}

pub async fn get_locations(
    search_str: &str,
    pool: &Pool,
) -> anyhow::Result<Vec<LocationNameModel>> {
    let locations = sqlx::query_as!(
        LocationNameModel,
        r#"
            SELECT location_id, name
            FROM locations
            WHERE name LIKE ? || '%'
            ORDER BY name DESC
            LIMIT 5
        "#,
        search_str
    )
    .fetch_all(pool)
    .await?;
    Ok(locations)
}

pub async fn set_user_verified(
    user_id: &str,
    email: &str,
    pool: &Pool,
) -> anyhow::Result<Option<String>> {
    let user_id = sqlx::query_scalar!(
        "
            UPDATE users
            SET verified = 1
            WHERE user_id = ? AND email = ?
            RETURNING user_id
        ",
        user_id,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user_id)
}

pub async fn delete_user(
    user_id: &str,
    email: &str,
    pool: &Pool,
) -> anyhow::Result<Option<String>> {
    let user_id = sqlx::query_scalar!(
        "
            DELETE FROM users
            WHERE user_id = ? AND email = ?
            RETURNING user_id
        ",
        user_id,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user_id)
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub alert_threshold: types::AlertLevel,
    pub locations: Vec<types::Location>,
}

pub async fn insert_user(
    user: &RegisterUser,
    pool: &Pool,
) -> anyhow::Result<UserWithLocationsModel> {
    let mut tx = pool.begin().await?;
    let user_id = sqlx::query_scalar!(
        "
            INSERT INTO users (email, alert_threshold)
            VALUES (?, ?)
            RETURNING user_id
        ",
        user.email,
        user.alert_threshold
    )
    .fetch_one(&mut tx)
    .await?;

    for location in &user.locations {
        sqlx::query!(
            "
                INSERT INTO user_locations (user_id, location_id)
                VALUES (?, ?)
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
              users.last_alerted_at as "last_alerted_at: chrono::DateTime<chrono::Utc>",
              locations.location_id as "location_id: types::Location",
              locations.name,
              locations.weather_description,
              locations.cloud_cover,
              locations.updated_at as "updated_at: chrono::DateTime<chrono::Utc>"
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

    let user = UserWithLocationsModel::one_from_rows(user_locations);

    tx.commit().await?;

    Ok(user)
}

#[derive(Debug)]
pub struct AlertLevelModel {
    pub alert_level: types::AlertLevel,
    pub previous_alert_level: types::AlertLevel,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_alert_level(pool: &Pool) -> anyhow::Result<AlertLevelModel> {
    let alert_level = sqlx::query_as!(
        AlertLevelModel,
        r#"
            SELECT
              alert_level as "alert_level: types::AlertLevel",
              previous_alert_level as "previous_alert_level: types::AlertLevel",
              updated_at as "updated_at: chrono::DateTime<chrono::Utc>"
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

pub async fn update_alert_level(
    new_alert_level: &types::CurrentStatus,
    previous_alert_level: &AlertLevelModel,
    pool: &Pool,
) -> anyhow::Result<()> {
    sqlx::query!(
        "
            UPDATE
              alert_level
            SET
              alert_level = ?,
              previous_alert_level = ?,
              updated_at = ?
            WHERE
              alert_level_id = 1
        ",
        new_alert_level.site_status.alert_level,
        previous_alert_level.alert_level,
        new_alert_level.updated_at.datetime.value
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize, Clone)]
pub struct LocationModel {
    pub location_id: types::Location,
    pub name: String,
    pub weather_description: String,
    pub cloud_cover: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_unique_user_locations(pool: &Pool) -> anyhow::Result<Vec<LocationModel>> {
    let locations = sqlx::query_as!(
        LocationModel,
        r#"
            SELECT DISTINCT
              locations.location_id as "location_id: types::Location",
              locations.name,
              locations.weather_description,
              locations.cloud_cover,
              locations.updated_at as "updated_at: chrono::DateTime<chrono::Utc>"
            FROM
              user_locations INNER JOIN locations USING (location_id)
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(locations)
}

pub async fn update_weather(
    weather: apis::open_weather::CurrentWeather,
    location: types::Location,
    pool: &Pool,
) -> anyhow::Result<()> {
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
        weather.weather[0].description,
        weather.clouds.all,
        location
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_user_last_alerted_at(user_id: &str, pool: &Pool) -> anyhow::Result<()> {
    let now = chrono::Utc::now();
    sqlx::query!(
        "
            UPDATE users
            SET last_alerted_at = ?
            WHERE user_id = ?
        ",
        now,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct UserWithLocationsModel {
    pub user_id: String,
    pub email: String,
    pub alert_threshold: types::AlertLevel,
    pub last_alerted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub locations: Vec<LocationModel>,
}

impl UserWithLocationsModel {
    fn one_from_rows(user_rows: Vec<UserWithLocationModel>) -> Self {
        let mut locations = vec![];
        for user_row in &user_rows {
            let location = LocationModel {
                location_id: user_row.location_id,
                name: user_row.name.clone(),
                weather_description: user_row.weather_description.clone(),
                cloud_cover: user_row.cloud_cover,
                updated_at: user_row.updated_at,
            };
            locations.push(location);
        }
        let row = &user_rows[0];

        UserWithLocationsModel {
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
            let user_entry = user_map
                .entry(&user.user_id)
                .or_insert(UserWithLocationsModel {
                    user_id: user.user_id.clone(),
                    email: user.email.clone(),
                    alert_threshold: user.alert_threshold.clone(),
                    last_alerted_at: user.last_alerted_at,
                    locations: vec![],
                });
            let location = LocationModel {
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
    last_alerted_at: Option<chrono::DateTime<chrono::Utc>>,
    location_id: types::Location,
    name: String,
    weather_description: String,
    cloud_cover: i64,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_verified_users(pool: &Pool) -> anyhow::Result<Vec<UserWithLocationsModel>> {
    let users = sqlx::query_as!(
        UserWithLocationModel,
        r#"
            SELECT
              users.user_id,
              users.email,
              users.alert_threshold as "alert_threshold: types:: AlertLevel",
              users.last_alerted_at as "last_alerted_at: chrono::DateTime<chrono::Utc>",
              locations.location_id as "location_id: types::Location",
              locations.name,
              locations.weather_description,
              locations.cloud_cover,
              locations.updated_at as "updated_at: chrono::DateTime<chrono::Utc>"
            FROM
              users, locations, user_locations
            WHERE users.user_id = user_locations.user_id
            AND locations.location_id = user_locations.location_id
            AND users.verified = 1
        "#
    )
    .fetch_all(pool)
    .await?;

    let users = UserWithLocationsModel::many_from_rows(users);

    Ok(users)
}

pub async fn delete_unverified_users(pool: &Pool) -> anyhow::Result<usize> {
    let deleted_users_count = sqlx::query!(
        r#"
            DELETE FROM users
            WHERE NOT verified
            RETURNING changes() as "changes: i64"
        "#,
    )
    .fetch_all(pool)
    .await?
    .len();

    Ok(deleted_users_count)
}
