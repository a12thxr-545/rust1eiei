-- Add cover image fields to brawlers table
ALTER TABLE brawlers
ADD COLUMN cover_url VARCHAR(512),
ADD COLUMN cover_public_id VARCHAR(255);
