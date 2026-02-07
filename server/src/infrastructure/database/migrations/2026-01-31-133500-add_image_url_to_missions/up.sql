-- Add image_url field to missions table
ALTER TABLE missions
ADD COLUMN image_url VARCHAR(512);
