-- Your SQL goes here
ALTER TABLE brawlers
ADD COLUMN display_name VARCHAR(50) NOT NULL,
ADD COLUMN avatar_url VARCHAR(512),
ADD COLUMN avatar_public_id VARCHAR(255);