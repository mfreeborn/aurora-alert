CREATE TABLE users (
    user_id TEXT NOT NULL PRIMARY KEY DEFAULT (LOWER(HEX(RANDOMBLOB(16)))),
    email TEXT NOT NULL UNIQUE,
    alert_threshold TEXT NOT NULL CHECK(alert_threshold in ('yellow', 'amber', 'red')),
    verified INTEGER NOT NULL DEFAULT 0 CHECK(verified in (0, 1)),
    last_alerted_at TEXT,
    registered_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_users_email ON users (email);

CREATE TABLE alert_level (
    alert_level_id INTEGER PRIMARY KEY NOT NULL,
    alert_level TEXT NOT NULL CHECK(alert_level in ('green', 'yellow', 'amber', 'red')),
    previous_alert_level TEXT NOT NULL CHECK(alert_level in ('green', 'yellow', 'amber', 'red')),
    updated_at TEXT NOT NULL
);

CREATE TABLE locations (
    location_id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    weather_description TEXT NOT NULL,
    cloud_cover INTEGER NOT NULL CHECK(0 <= cloud_cover <= 100),
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_locations_name ON locations (name COLLATE NOCASE);

CREATE TRIGGER update_location_updated_at
    AFTER UPDATE
    ON locations
BEGIN
    UPDATE locations SET updated_at = CURRENT_TIMESTAMP WHERE location_id = old.location_id;
END;

CREATE TABLE user_locations (
    user_id TEXT NOT NULL,
    location_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, location_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations (location_id) ON DELETE CASCADE
);

CREATE TABLE activity_data (
    datetime TEXT NOT NULL,
    value REAL NOT NULL,
    PRIMARY KEY (datetime) ON CONFLICT REPLACE
);

CREATE TABLE activity_data_meta (
    activity_data_meta_id INTEGER NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (activity_data_meta_id) ON CONFLICT REPLACE
);

-- insert seed data as desired
INSERT INTO users (email, alert_threshold, verified)
VALUES ('my_email', 'yellow', 1);

INSERT INTO alert_level (alert_level_id, alert_level, previous_alert_level, updated_at)
VALUES (1, 'green', 'green', '2020-01-01 00:00:00');

INSERT INTO locations (location_id, name, weather_description, cloud_cover, updated_at)
VALUES
    (2649169, 'Fort William', 'cloudy', '100', '2020-01-01 00:00:00'),
    (2637248, 'Spean Bridge', 'cloudy', '100', '2020-01-01 00:00:00');

INSERT INTO user_locations (user_id, location_id)
SELECT user_id, 2649169
FROM users
LIMIT 1;

INSERT INTO user_locations (user_id, location_id)
SELECT user_id, 2637248
FROM users
LIMIT 1;