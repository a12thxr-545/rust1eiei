-- Your SQL goes here
CREATE TABLE friendships (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES brawlers(id),
    friend_id INTEGER NOT NULL REFERENCES brawlers(id),
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'accepted'
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    UNIQUE(user_id, friend_id)
);

CREATE TABLE mission_invitations (
    id SERIAL PRIMARY KEY,
    mission_id INTEGER NOT NULL REFERENCES missions(id),
    inviter_id INTEGER NOT NULL REFERENCES brawlers(id),
    invitee_id INTEGER NOT NULL REFERENCES brawlers(id),
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'accepted', 'rejected'
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    UNIQUE(mission_id, invitee_id) -- Only one active invitation for a mission per invitee
);

SELECT diesel_manage_updated_at('friendships');
