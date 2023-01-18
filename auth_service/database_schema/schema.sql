DROP TABLE IF EXISTS keys;
CREATE TABLE keys (
    public VARCHAR NOT NULL,
    private VARCHAR NOT NULL
);

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id SERIAL,

    login VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    
    last_recover_password_nonce BIGINT NOT NULL,
    last_auth_nonce BIGINT NOT NULL,

    UNIQUE (login),
    UNIQUE (email)
);