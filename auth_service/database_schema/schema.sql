DROP TABLE IF EXISTS private_key;
CREATE TABLE private_key (
    key VARCHAR NOT NULL
);

DROP TABLE IF EXISTS password_recovery_requests;
CREATE TABLE password_recovery_requests (
    user_id INTEGER,

    access_code VARCHAR,

    UNIQUE (user_id)
);

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id SERIAL,

    login VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL,

    last_recover_password_nonce BIGINT NOT NULL DEFAULT 0,
    last_auth_nonce BIGINT NOT NULL DEFAULT 0,

    UNIQUE (login),
    UNIQUE (email)
);