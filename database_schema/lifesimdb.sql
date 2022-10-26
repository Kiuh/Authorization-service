DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id SERIAL,

    login VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL,

    UNIQUE (login),
    UNIQUE (email)
);

DROP TABLE IF EXISTS generations;
CREATE TABLE generations(
    id SERIAL,

    name VARCHAR NOT NULL,
    owner_id INTEGER NOT NULL,
    description VARCHAR NOT NULL DEFAULT '',

    map_id VARCHAR NOT NULL,
    life_type VARCHAR NOT NULL,
    feed_type VARCHAR NOT NULL,
    setup_type VARCHAR NOT NULL,
    setup_json VARCHAR NOT NULL,

    tick_period DECIMAL NOT NULL,
    time DECIMAL NOT NULL DEFAULT 0,

    last_send_num BIGINT NOT NULL DEFAULT 0,
    last_cell_num BIGINT NOT NULL DEFAULT 0,

    UNIQUE(name, owner_id)
);

DROP TABLE IF EXISTS maps;
CREATE TABLE maps(
    id SERIAL,
    name VARCHAR NOT NULL
);

DROP TABLE IF EXISTS life_types;
CREATE TABLE life_types(
    id SERIAL,
    name VARCHAR NOT NULL 
);

DROP TABLE IF EXISTS feed_types;
CREATE TABLE feed_types(
    id SERIAL,
    name VARCHAR NOT NULL 
);

DROP TABLE IF EXISTS tick_periods;
CREATE TABLE tick_periods(
    id SERIAL,
    period DECIMAL NOT NULL 
);

DROP TABLE IF EXISTS setup_types;
CREATE TABLE setup_types(
    id SERIAL,
    name VARCHAR NOT NULL,
    json VARCHAR NOT NULL 
);