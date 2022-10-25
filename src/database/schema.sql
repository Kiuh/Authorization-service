DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id SERIAL,
    login VARCHAR,
    password VARCHAR,
    email VARCHAR,

    UNIQUE (login),
    UNIQUE (email)
);
