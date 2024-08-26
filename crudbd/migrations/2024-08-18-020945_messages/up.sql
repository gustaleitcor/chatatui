-- Your SQL goes here
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    chat_id INTEGER NOT NULL,
    user_id INTEGER,
    date TIMESTAMP NOT NULL DEFAULT NOW(),

    FOREIGN KEY (chat_id) REFERENCES chats (id)
        ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id)
        ON DELETE SET NULL ON UPDATE CASCADE
);
