-- Your SQL goes here
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    participant_id INTEGER NOT NULL,
    date TIMESTAMP NOT NULL DEFAULT NOW(),

    FOREIGN KEY (participant_id) REFERENCES participants (id)
        ON DELETE CASCADE ON UPDATE CASCADE
);
