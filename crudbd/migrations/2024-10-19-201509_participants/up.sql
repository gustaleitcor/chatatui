-- Your SQL goes here
CREATE TABLE participants (
    id SERIAL PRIMARY KEY,
    chat_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,

    FOREIGN KEY (chat_id) REFERENCES chats (id)
        ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id)
        ON DELETE SET NULL ON UPDATE CASCADE
);
