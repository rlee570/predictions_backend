-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE votes(
  id SERIAL PRIMARY KEY,
  prediction INTEGER NOT NULL references predictions (id),
  user_id  INTEGER NOT NULL references users (id),
  points INTEGER NOT NULL default 1
);