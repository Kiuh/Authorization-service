DROP TABLE IF EXISTS private_key;
CREATE TABLE private_key (
    key VARCHAR NOT NULL
);

-- TODO: Add table 'password_recovery_request'

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id SERIAL,

    login VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL,

    new_password VARCHAR NULL,
    recover_password_access_code VARCHAR NULL,
    last_recover_password_nonce BIGINT NOT NULL DEFAULT 0,

    last_auth_nonce BIGINT NOT NULL DEFAULT 0,

    UNIQUE (login),
    UNIQUE (email)
);