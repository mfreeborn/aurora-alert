use actix_web::rt as actix_rt;

use crate::apis;
use crate::db;
use crate::helpers;
use crate::mail;
use crate::templates;
use crate::types;

async fn maybe_alert(
    pool: &db::Pool,
    template_engine: &templates::Engine,
    mailer: &mail::Transport,
) -> anyhow::Result<()> {
    let stored_alert_level = db::get_alert_level(pool).await?;
    let live_alert_level = apis::aurora_watch::get_alert_level().await?;
    println!("{stored_alert_level:#?}");
    println!("{live_alert_level:#?}");

    if (stored_alert_level.updated_at == live_alert_level.updated_at.datetime.value)
        && (stored_alert_level.alert_level == types::AlertLevel::Green)
    {
        // the live alert level is the same as the stored alert level. If the alert
        // level is yellow or higher, we still need to check for alerts because the
        // cloud cover may have reduced
        return Ok(());
    }

    if live_alert_level.updated_at.datetime.value > stored_alert_level.updated_at {
        // the live alert level is more up to date than the stored alert level,
        // so we need to update the stored alert level
        db::update_alert_level(&live_alert_level, &stored_alert_level, pool).await?;
    }

    if live_alert_level.site_status.alert_level >= types::AlertLevel::Yellow {
        // we are at yellow or above: update the weather forecasts
        let locations = db::get_unique_user_locations(pool).await?;
        let now = chrono::Utc::now();
        let four_minutes_ago = now - chrono::Duration::minutes(4);
        for location in &locations {
            if location.updated_at < four_minutes_ago {
                let live_weather = apis::open_weather::get_weather(location.location_id).await?;
                db::update_weather(live_weather, location.location_id, pool).await?;
            }
        }

        let verified_users = db::get_verified_users(pool).await?;
        for user in &verified_users {
            if helpers::should_alert_user(user, &live_alert_level.site_status.alert_level) {
                mail::Email::new_alert(&user.email)
                    .add_context(user, &live_alert_level.site_status.alert_level)?
                    .render_body(template_engine)?
                    .build_email()?
                    .send(mailer.clone());
                db::update_user_last_alerted_at(&user.user_id, pool).await?;
            }
        }
    }

    Ok(())
}

/// A task which runs every 5 minutes and conditionally sends out email notifications of alert level changes
pub async fn alert_task(
    pool: &db::Pool,
    template_engine: &templates::Engine,
    mailer: &mail::Transport,
) -> ! {
    log::info!("Started alert_task");
    let mut interval = actix_rt::time::interval(std::time::Duration::from_secs(60 * 5));
    loop {
        interval.tick().await;
        if let Err(e) = maybe_alert(pool, template_engine, mailer).await {
            eprintln!("{e:#?}");
        }
    }
}

/// A task which runs every midnight and deletes an users who remain unverified.
pub async fn clear_unverified_users_task(pool: &db::Pool) -> ! {
    log::info!("Started clear_unverified_users_task");
    loop {
        let tomorrow_midnight = chrono::Utc::now().date().succ().and_hms(0, 0, 0);
        let sleep_duration = tomorrow_midnight - chrono::Utc::now();
        log::info!(
            r#"Next run of the "clear_unverified_users" task in ~{:?} hours"#,
            sleep_duration.num_hours(),
        );
        actix_rt::time::sleep_until(
            actix_rt::time::Instant::now() + sleep_duration.to_std().unwrap(),
        )
        .await;

        let deleted_users_count = db::delete_unverified_users(pool).await.unwrap();
        log::info!("{} user(s) deleted from the database", deleted_users_count);
    }
}

/// A task which periodically updates the locally cached aurora activity data
pub async fn update_activity_data_task(pool: &db::Pool) -> ! {
    log::info!("Started update_activity_data_task");
    let mut interval = actix_rt::time::interval(std::time::Duration::from_secs(60 * 5));
    loop {
        interval.tick().await;
        let activity = apis::aurora_watch::get_activity_data().await;

        let activity = match activity {
            Ok(act) => act,
            Err(e) => {
                log::warn!("Error fetching aurora activity levels: {e}");
                continue;
            }
        };

        let res = db::update_aurora_activity(activity, pool).await;

        if let Err(e) = res {
            log::warn!("Error updating activity data in database: {e}");
        };
    }
}
