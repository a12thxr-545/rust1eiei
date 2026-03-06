CREATE INDEX idx_missions_chief_id ON missions(chief_id);
CREATE INDEX idx_crew_memberships_brawler_id ON crew_memberships(brawler_id);
CREATE INDEX idx_chat_messages_mission_id ON chat_messages(mission_id);
CREATE INDEX idx_friendships_brawler_id ON friendships(brawler_id);
CREATE INDEX idx_friendships_friend_id ON friendships(friend_id);

