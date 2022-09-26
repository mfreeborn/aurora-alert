use crate::apis;
use crate::common::AlertLevel;
use crate::configuration::Settings;
use crate::db;
use crate::db::DbPool;
use crate::email::EmailClient;
use crate::helpers;
use crate::startup::{get_connection_pool, get_email_client};

/// Send an email alert to all users where the alert criteria are met.
async fn maybe_alert(
    open_weather_api_key: &str,
    pool: &DbPool,
    email_client: &EmailClient,
) -> Result<(), anyhow::Error> {
    let stored_alert_level = db::get_alert_level(pool).await?;
    let live_alert_level = apis::aurora_watch::get_alert_level().await?;

    if (stored_alert_level.updated_at == live_alert_level.updated_at)
        && (stored_alert_level.alert_level == AlertLevel::Green)
    {
        // The live alert level is the same as the stored alert level. If the alert
        // level is yellow or higher, we still need to check for alerts because the
        // cloud cover may have reduced.
        return Ok(());
    }

    if live_alert_level.updated_at > stored_alert_level.updated_at {
        // The live alert level is more up to date than the stored alert level,
        // so we need to update the stored alert level.
        db::update_alert_level(&live_alert_level, pool).await?;
    }

    if live_alert_level.level >= AlertLevel::Yellow {
        // We are at yellow or above: update the weather reports if they are stale.
        let locations = db::get_unique_user_locations(pool).await?;
        let now = chrono::Utc::now();
        let four_minutes_ago = now - chrono::Duration::minutes(4);
        for location in &locations {
            if location.updated_at < four_minutes_ago {
                let live_weather =
                    apis::open_weather::get_weather(location.location_id, &open_weather_api_key)
                        .await?;

                db::update_weather(live_weather, location.location_id, pool).await?;
            }
        }

        // Send out alerts to verified users, if they are due one.
        let verified_users = db::get_verified_users(pool).await?;
        for user in &verified_users {
            if helpers::should_alert_user(user, &live_alert_level.level) {
                let message = email_client
                    .new_alert(&user.email)
                    .add_context(user, &live_alert_level.level)?
                    .build_email()?;

                email_client.send(message).await;

                db::update_user_last_alerted_at(&user.user_id, pool).await?;
            }
        }
    }

    Ok(())
}

/// A task which runs every 5 minutes and conditionally sends out email notifications of alert level changes.
pub async fn alert_task(config: Settings) -> Result<(), anyhow::Error> {
    tracing::debug!("Started alert_task");
    let pool = get_connection_pool(&config.database);
    let email_client = get_email_client(&config.email);

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60 * 5));
    loop {
        interval.tick().await;
        if let Err(e) = maybe_alert(
            &config.application.open_weather_api_key,
            &pool,
            &email_client,
        )
        .await
        {
            tracing::error!("error within alert task: {e}");
        }
    }
}

/// A task which runs every midnight and deletes any users who remain unverified.
pub async fn clear_unverified_users_task(config: Settings) -> Result<(), anyhow::Error> {
    tracing::debug!("started clear_unverified_users_task");
    let pool = get_connection_pool(&config.database);
    loop {
        let now = chrono::Utc::now();
        let tomorrow_midnight = now.date().succ().and_hms(0, 0, 0);
        let sleep_duration = tomorrow_midnight - now;

        tracing::info!(
            r#"Next run of the "clear_unverified_users" task in ~{:?} hours"#,
            sleep_duration.num_hours(),
        );

        tokio::time::sleep_until(tokio::time::Instant::now() + sleep_duration.to_std().unwrap())
            .await;

        let deleted_users_count = db::delete_unverified_users(&pool).await;

        match deleted_users_count {
            Ok(count) => {
                tracing::info!(
                    "{} user(s) deleted from the database as part of daily maintenance",
                    count
                )
            }
            Err(e) => tracing::error!("error removing stale unverified users: {e}"),
        }
    }
}

/// A task which periodically updates the locally cached aurora activity data.
pub async fn update_activity_data_task(config: Settings) -> Result<(), anyhow::Error> {
    tracing::debug!("started update_activity_data_task");
    let pool = get_connection_pool(&config.database);
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60 * 5));
    loop {
        interval.tick().await;
        let activity = apis::aurora_watch::get_activity_data().await;

        let activity = match activity {
            Ok(act) => act,
            Err(e) => {
                tracing::error!("Error fetching aurora activity levels: {e}");
                continue;
            }
        };

        let res = db::update_aurora_activity(activity, &pool).await;

        if let Err(e) = res {
            tracing::error!("error updating activity data in database: {e}");
        };
    }
}
