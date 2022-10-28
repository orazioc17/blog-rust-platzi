-- Your SQL goes here
CREATE TABLE posts (
    id serial PRIMARY KEY,
    title VARCHAR NOT NULL,
    slug VARCHAR NOT NULL,
    body TEXT NOT NULL
)