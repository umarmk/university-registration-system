-- Your SQL goes here
CREATE TABLE students (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    phone VARCHAR UNIQUE NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    course VARCHAR NOT NULL
);
