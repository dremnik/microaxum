CREATE TABLE users (
    id VARCHAR PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    first_name VARCHAR,
    last_name VARCHAR,
    updated_at BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);
