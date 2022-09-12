INSERT INTO users (email, alert_threshold, verified)
VALUES ('my_email', 'yellow', true);

INSERT INTO alert_level (alert_level_id, alert_level, updated_at)
VALUES (1, 'green', '2020-01-01 00:00:00');

INSERT INTO locations (location_id, name, weather_description, cloud_cover, updated_at)
VALUES
    (2649169, 'Fort William', 'cloudy', 100, '2020-01-01 00:00:00'),
    (2637248, 'Spean Bridge', 'cloudy', 100, '2020-01-01 00:00:00');

INSERT INTO user_locations (user_id, location_id)
SELECT user_id, 2649169
FROM users
LIMIT 1;

INSERT INTO user_locations (user_id, location_id)
SELECT user_id, 2637248
FROM users
LIMIT 1;