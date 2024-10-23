-- Your SQL goes here
-- Your SQL goes here
CREATE OR REPLACE VIEW view_users_by_chat AS
SELECT
    p.chat_id,
    c.title AS chat_title,
    p.user_id,
    u.username,
    u.bill,
    COUNT(m.id) AS message_count,
    (SELECT COUNT(*) FROM participants WHERE chat_id = p.chat_id) AS participant_count,
    (SELECT COUNT(*) FROM messages WHERE user_id = p.user_id) AS user_message_count,
    MAX(m.date) AS last_message_time,
    MIN(m.date) AS first_message_time,
    AVG(LENGTH(m.content)) AS avg_message_length
FROM participants p
JOIN users u ON p.user_id = u.id
JOIN chats c ON p.chat_id = c.id
LEFT JOIN messages m ON m.participant_id = p.id
GROUP BY p.chat_id, c.title, p.user_id, u.username, u.bill;
