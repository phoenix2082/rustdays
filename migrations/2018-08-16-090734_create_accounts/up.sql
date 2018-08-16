-- Your SQL goes here

CREATE TABLE account (
  id SERIAL PRIMARY KEY,
  firstname VARCHAR NOT NULL,
  middlename VARCHAR,
  lastname VARCHAR NOT NULL,
  email_id VARCHAR NOT NULL
);
