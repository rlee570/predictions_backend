-- Your SQL goes here
CREATE TABLE predictions(
    id SERIAL PRIMARY KEY,
    owner integer NOT NULL references users (id),
    statement VARCHAR NOT NULL,
    expiry TIMESTAMP NOT NULL,
    outcome BOOLEAN NOT NULL,
    votes INTEGER NOT NULL DEFAULT 0
);