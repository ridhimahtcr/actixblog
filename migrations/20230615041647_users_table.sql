-- Add migration script here
CREATE TABLE users (
   user_id serial PRIMARY KEY,
   username TEXT NOT NULL UNIQUE,
   password TEXT NOT NULL
);
