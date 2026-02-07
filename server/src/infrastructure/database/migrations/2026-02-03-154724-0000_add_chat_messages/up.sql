CREATE TABLE mission_chat_messages (
    id SERIAL PRIMARY KEY,
    mission_id INT NOT NULL REFERENCES missions(id),
    brawler_id INT NOT NULL REFERENCES brawlers(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
