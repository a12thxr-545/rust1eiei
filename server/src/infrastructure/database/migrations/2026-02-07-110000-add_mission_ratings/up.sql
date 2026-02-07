CREATE TABLE mission_ratings (
    id SERIAL PRIMARY KEY,
    mission_id INT NOT NULL REFERENCES missions(id),
    brawler_id INT NOT NULL REFERENCES brawlers(id),
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(mission_id, brawler_id)
);
