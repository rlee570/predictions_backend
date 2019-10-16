-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  points INTEGER NOT NULL,
  role TEXT NOT NULL,
  hash TEXT NOT NULL
);