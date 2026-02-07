-- Remove cover image fields from brawlers table
ALTER TABLE brawlers
DROP COLUMN cover_url,
DROP COLUMN cover_public_id;
