CREATE TYPE alert_level_enum AS ENUM ('green', 'yellow', 'amber', 'red');

CREATE TABLE users (
    user_id UUID NOT NULL DEFAULT gen_random_uuid(),
    email CITEXT NOT NULL UNIQUE,
    alert_threshold alert_level_enum NOT NULL,
    verified BOOL NOT NULL DEFAULT false,
    last_alerted_at TIMESTAMPTZ,
    registered_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id)
);


CREATE TABLE alert_level (
    alert_level_id SERIAL NOT NULL PRIMARY KEY,
    alert_level alert_level_enum NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE locations (
    location_id SERIAL NOT NULL PRIMARY KEY,
    name CITEXT NOT NULL UNIQUE,
    weather_description TEXT NOT NULL,
    cloud_cover SMALLINT NOT NULL CHECK(cloud_cover BETWEEN 0 AND 100),
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE FUNCTION update_location_updated_at_fn()
    RETURNS trigger AS
$$
BEGIN
    UPDATE locations SET updated_at = CURRENT_TIMESTAMP WHERE NEW.location_id = OLD.location_id;
    RETURN NEW;
END;
$$
LANGUAGE 'plpgsql';

CREATE TRIGGER update_location_updated_at
    AFTER UPDATE
    ON locations
    FOR EACH STATEMENT
    EXECUTE PROCEDURE update_location_updated_at_fn();


CREATE TABLE user_locations (
    user_id UUID NOT NULL,
    location_id SERIAL NOT NULL,
    PRIMARY KEY (user_id, location_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations (location_id) ON DELETE CASCADE
);

CREATE TABLE activity_data (
    timestamp TIMESTAMPTZ NOT NULL PRIMARY KEY,
    value REAL NOT NULL
);

CREATE TABLE activity_data_meta (
    activity_data_meta_id SERIAL NOT NULL PRIMARY KEY,
    updated_at TIMESTAMPTZ NOT NULL
);
