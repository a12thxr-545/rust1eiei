use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{
        entities::friendships::{AddFriendshipEntity, FriendshipEntity},
        repositories::friendships::FriendshipRepository,
    },
    infrastructure::database::{postgresql_connection::PgPoolSquad, schema::friendships},
};

pub struct FriendshipPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl FriendshipPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl FriendshipRepository for FriendshipPostgres {
    async fn add(&self, entity: AddFriendshipEntity) -> Result<i32> {
        let db_pool = Arc::clone(&self.db_pool);
        let result = tokio::task::spawn_blocking(move || -> Result<i32> {
            let mut conn = db_pool.get()?;
            let res = diesel::insert_into(friendships::table)
                .values(&entity)
                .returning(friendships::id)
                .get_result::<i32>(&mut conn)?;
            Ok(res)
        })
        .await??;
        Ok(result)
    }

    async fn accept(&self, uid: i32, fid: i32) -> Result<()> {
        let db_pool = Arc::clone(&self.db_pool);
        tokio::task::spawn_blocking(move || -> Result<()> {
            let mut conn = db_pool.get()?;
            diesel::update(friendships::table)
                .filter(
                    friendships::user_id
                        .eq(uid)
                        .and(friendships::friend_id.eq(fid)),
                )
                .set((
                    friendships::status.eq("accepted"),
                    friendships::updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(&mut conn)?;
            Ok(())
        })
        .await??;
        Ok(())
    }

    async fn reject(&self, uid: i32, fid: i32) -> Result<()> {
        let db_pool = Arc::clone(&self.db_pool);
        tokio::task::spawn_blocking(move || -> Result<()> {
            let mut conn = db_pool.get()?;
            diesel::delete(friendships::table)
                .filter(
                    friendships::user_id
                        .eq(uid)
                        .and(friendships::friend_id.eq(fid)),
                )
                .execute(&mut conn)?;
            Ok(())
        })
        .await??;
        Ok(())
    }

    async fn get_friends(&self, uid: i32) -> Result<Vec<FriendshipEntity>> {
        let db_pool = Arc::clone(&self.db_pool);
        let result = tokio::task::spawn_blocking(move || -> Result<Vec<FriendshipEntity>> {
            let mut conn = db_pool.get()?;
            let res = friendships::table
                .filter(
                    (friendships::user_id
                        .eq(uid)
                        .or(friendships::friend_id.eq(uid)))
                    .and(friendships::status.eq("accepted")),
                )
                .load::<FriendshipEntity>(&mut conn)?;
            Ok(res)
        })
        .await??;
        Ok(result)
    }

    async fn get_pending_requests(&self, uid: i32) -> Result<Vec<FriendshipEntity>> {
        let db_pool = Arc::clone(&self.db_pool);
        let result = tokio::task::spawn_blocking(move || -> Result<Vec<FriendshipEntity>> {
            let mut conn = db_pool.get()?;
            let res = friendships::table
                .filter(
                    friendships::friend_id
                        .eq(uid)
                        .and(friendships::status.eq("pending")),
                )
                .load::<FriendshipEntity>(&mut conn)?;
            Ok(res)
        })
        .await??;
        Ok(result)
    }

    async fn remove(&self, uid1: i32, uid2: i32) -> Result<()> {
        let db_pool = Arc::clone(&self.db_pool);
        tokio::task::spawn_blocking(move || -> Result<()> {
            let mut conn = db_pool.get()?;
            diesel::delete(friendships::table)
                .filter(
                    (friendships::user_id
                        .eq(uid1)
                        .and(friendships::friend_id.eq(uid2)))
                    .or(friendships::user_id
                        .eq(uid2)
                        .and(friendships::friend_id.eq(uid1))),
                )
                .execute(&mut conn)?;
            Ok(())
        })
        .await??;
        Ok(())
    }

    async fn check_friendship(&self, uid1: i32, uid2: i32) -> Result<Option<FriendshipEntity>> {
        let db_pool = Arc::clone(&self.db_pool);
        let result = tokio::task::spawn_blocking(move || -> Result<Option<FriendshipEntity>> {
            let mut conn = db_pool.get()?;
            let res = friendships::table
                .filter(
                    (friendships::user_id
                        .eq(uid1)
                        .and(friendships::friend_id.eq(uid2)))
                    .or(friendships::user_id
                        .eq(uid2)
                        .and(friendships::friend_id.eq(uid1))),
                )
                .order_by(friendships::updated_at.desc())
                .first::<FriendshipEntity>(&mut conn)
                .optional()?;
            Ok(res)
        })
        .await??;
        Ok(result)
    }
}
