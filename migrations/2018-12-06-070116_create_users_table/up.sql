-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    gender BOOLEAN NOT NULL,
    age INTEGER NOT NULL,
    address VARCHAR NOT NULL,
    phone VARCHAR(11) NOT NULL
);