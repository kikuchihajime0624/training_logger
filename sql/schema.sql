DROP TABLE IF EXISTS training_set;
DROP TABLE IF EXISTS training_parts, training_event;
DROP TABLE IF EXISTS users;


CREATE TABLE users
(
    username varchar(32) PRIMARY KEY,
    password varchar(100) NOT NULL
);

CREATE TABLE training_event
(

    event_id   SERIAL PRIMARY KEY,
    event_name varchar(20) NOT NULL,
    username   varchar(32) REFERENCES users (username)
);

CREATE TABLE training_parts
(
    parts_id   SERIAL PRIMARY KEY,
    parts_name varchar(20) NOT NULL,
    username   varchar(32) REFERENCES users (username)
);



CREATE TABLE training_set
(
    training_set_id SERIAL NOT NULL,
    event_id        INT REFERENCES training_event (event_id),
    parts_id        INT REFERENCES training_parts (parts_id),
    weight          REAL   NOT NULL,
    times           INT    NOT NULL,
    workout_date    DATE   NOT NULL,
    username        varchar(32) REFERENCES users (username),
    PRIMARY KEY (training_set_id)
);


