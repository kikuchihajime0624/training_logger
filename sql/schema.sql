CREATE TABLE training_event
(
    event_id   varchar(10) PRIMARY KEY,
    event_name varchar(10) NOT NULL
);

CREATE TABLE training_parts
(
    parts_id   varchar(10) PRIMARY KEY,
    parts_name varchar(10) NOT NULL
);

CREATE TABLE users
(
    user_id   INT PRIMARY KEY,
    user_name TEXT NOT NULL
);

CREATE TABLE training_set
(
    training_set_id varchar(10) NOT NULL,
    event_id        varchar(10) REFERENCES training_event (event_id),
    parts_id        varchar(10) REFERENCES training_parts (parts_id),
    weight          INT         NOT NULL,
    times           INT         NOT NULL,
    workout_date    DATE        NOT NULL,
    user_id         INT REFERENCES users (user_id),
    PRIMARY KEY (training_set_id)
);


