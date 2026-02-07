-- Your SQL goes here
ALTER TABLE missions ADD COLUMN code VARCHAR(5);
-- For existing missions, generate a random code
UPDATE missions SET code = upper(substring(replace(gen_random_uuid()::text, '-', '') from 1 for 5)) WHERE code IS NULL;
ALTER TABLE missions ALTER COLUMN code SET NOT NULL;
ALTER TABLE missions ADD CONSTRAINT missions_code_unique UNIQUE (code);
