use crate::db;
use crate::types;

pub fn should_alert_user(
    user: &db::UserWithLocationsModel,
    alert_level: &types::AlertLevel,
) -> bool {
    if user.alert_threshold <= *alert_level {
        let now = chrono::Utc::now();
        let two_hours_ago = now - chrono::Duration::hours(2);
        if user.last_alerted_at.is_none() || user.last_alerted_at.unwrap() < two_hours_ago {
            return true;
        }
    }
    false
}
