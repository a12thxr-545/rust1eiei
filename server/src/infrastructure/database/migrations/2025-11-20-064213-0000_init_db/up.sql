-- Your SQL goes here
CREATE TABLE missions (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "description" TEXT,
    "status" VARCHAR(255) NOT NULL,
    chief_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP
);

CREATE TABLE brawlers (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);


CREATE TABLE crew_memberships (
    mission_id INTEGER NOT NULL,
    brawler_id INTEGER NOT NULL,
    joined_at TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (mission_id, brawler_id)
);

ALTER TABLE
    missions
ADD
    CONSTRAINT fk_chief FOREIGN KEY (chief_id) REFERENCES brawlers(id);

ALTER TABLE
    crew_memberships
ADD
    CONSTRAINT fk_mission FOREIGN KEY (mission_id) REFERENCES missions(id),
ADD
    CONSTRAINT fk_brawler FOREIGN KEY (brawler_id) REFERENCES brawlers(id);


SELECT diesel_manage_updated_at('missions');
SELECT diesel_manage_updated_at('brawlers');
