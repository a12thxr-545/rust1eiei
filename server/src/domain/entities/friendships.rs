use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::infrastructure::database::schema::friendships;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = friendships)]
pub struct FriendshipEntity {
    pub id: i32,
    pub user_id: i32,
    pub friend_id: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = friendships)]
pub struct AddFriendshipEntity {
    pub user_id: i32,
    pub friend_id: i32,
    pub status: String,
}
