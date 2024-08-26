-- Your SQL goes here
CREATE TABLE chats (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    is_public BOOLEAN NOT NULL
);
