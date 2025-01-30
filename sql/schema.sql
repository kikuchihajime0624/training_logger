DROP TABLE IF EXISTS training_set;
DROP TABLE IF EXISTS training_parts, training_event, users;

CREATE TABLE training_event
(
    event_id   SERIAL PRIMARY KEY,
    event_name varchar(20) NOT NULL
);

CREATE TABLE training_parts
(
    parts_id   SERIAL PRIMARY KEY,
    parts_name varchar(20) NOT NULL
);

CREATE TABLE users
(
    user_id   INT PRIMARY KEY,
    user_name TEXT NOT NULL
);

CREATE TABLE training_set
(
    training_set_id SERIAL NOT NULL,
    event_id        SERIAL REFERENCES training_event (event_id),
    parts_id        SERIAL REFERENCES training_parts (parts_id),
    weight          INT         NOT NULL,
    times           INT         NOT NULL,
    workout_date    DATE        NOT NULL,
    user_id         INT REFERENCES users (user_id),
    PRIMARY KEY (training_set_id)
);


