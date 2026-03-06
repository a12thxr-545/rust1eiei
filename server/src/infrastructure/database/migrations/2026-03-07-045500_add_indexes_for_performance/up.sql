CREATE INDEX IF NOT EXISTS idx_missions_chief_id ON missions(chief_id);
CREATE INDEX IF NOT EXISTS idx_crew_memberships_brawler_id ON crew_memberships(brawler_id);
CREATE INDEX IF NOT EXISTS idx_mission_chat_messages_mission_id ON mission_chat_messages(mission_id);
CREATE INDEX IF NOT EXISTS idx_friendships_user_id ON friendships(user_id);
CREATE INDEX IF NOT EXISTS idx_friendships_friend_id ON friendships(friend_id);

