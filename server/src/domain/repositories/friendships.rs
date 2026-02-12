use crate::domain::entities::friendships::{AddFriendshipEntity, FriendshipEntity};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait FriendshipRepository {
    async fn add(&self, entity: AddFriendshipEntity) -> Result<i32>;
    async fn accept(&self, user_id: i32, friend_id: i32) -> Result<()>;
    async fn reject(&self, user_id: i32, friend_id: i32) -> Result<()>;
    async fn get_friends(&self, user_id: i32) -> Result<Vec<FriendshipEntity>>;
    async fn get_pending_requests(&self, user_id: i32) -> Result<Vec<FriendshipEntity>>;
    async fn remove(&self, user_id1: i32, user_id2: i32) -> Result<()>;
    async fn check_friendship(
        &self,
        user_id1: i32,
        user_id2: i32,
    ) -> Result<Option<FriendshipEntity>>;
}
