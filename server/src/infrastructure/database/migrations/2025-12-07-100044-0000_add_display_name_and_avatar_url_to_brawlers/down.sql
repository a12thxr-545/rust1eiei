-- This file should undo anything in `up.sql`
ALTER TABLE brawlers
DROP COLUMN avatar_url,
DROP COLUMN display_name,
DROP COLUMN avatar_public_id;