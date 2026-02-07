use crate::domain::{
    entities::{friendships::AddFriendshipEntity, mission_invitations::AddMissionInvitationEntity},
    repositories::{
        brawlers::BrawlerRepository, friendships::FriendshipRepository,
        mission_invitations::MissionInvitationRepository,
        mission_viewing::MissionViewingRepository,
    },
    value_objects::social_model::{FriendModel, MissionInvitationModel},
};
use anyhow::{Result, anyhow};
use std::sync::Arc;

pub struct SocialUseCase<T1, T2, T3, T4>
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
{
    friendship_repo: Arc<T1>,
    invitation_repo: Arc<T2>,
    brawlers_repo: Arc<T3>,
    mission_repo: Arc<T4>,
}

impl<T1, T2, T3, T4> SocialUseCase<T1, T2, T3, T4>
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
{
    pub fn new(
        friendship_repo: Arc<T1>,
        invitation_repo: Arc<T2>,
        brawlers_repo: Arc<T3>,
        mission_repo: Arc<T4>,
    ) -> Self {
        Self {
            friendship_repo,
            invitation_repo,
            brawlers_repo,
            mission_repo,
        }
    }

    pub async fn add_friend(&self, user_id: i32, friend_id: i32) -> Result<i32> {
        if user_id == friend_id {
            return Err(anyhow!("You cannot add yourself as a friend"));
        }

        if let Some(_) = self
            .friendship_repo
            .check_friendship(user_id, friend_id)
            .await?
        {
            return Err(anyhow!("Friendship request already exists"));
        }

        self.friendship_repo
            .add(AddFriendshipEntity {
                user_id,
                friend_id,
                status: "pending".to_string(),
            })
            .await
    }

    pub async fn accept_friend(&self, user_id: i32, friend_id: i32) -> Result<()> {
        self.friendship_repo.accept(friend_id, user_id).await
    }

    pub async fn reject_friend(&self, user_id: i32, friend_id: i32) -> Result<()> {
        self.friendship_repo.reject(friend_id, user_id).await
    }

    pub async fn get_friends(&self, user_id: i32) -> Result<Vec<FriendModel>> {
        let friendships = self.friendship_repo.get_friends(user_id).await?;
        let mut result = Vec::new();

        for f in friendships {
            let friend_id = if f.user_id == user_id {
                f.friend_id
            } else {
                f.user_id
            };
            let brawler = self.brawlers_repo.find_by_id(friend_id).await?;

            result.push(FriendModel {
                friendship_id: f.id,
                friend_id,
                display_name: brawler.display_name,
                username: brawler.username,
                avatar_url: brawler.avatar_url,
                status: f.status,
            });
        }
        Ok(result)
    }

    pub async fn get_pending_requests(&self, user_id: i32) -> Result<Vec<FriendModel>> {
        let requests = self.friendship_repo.get_pending_requests(user_id).await?;
        let mut result = Vec::new();

        for r in requests {
            let brawler = self.brawlers_repo.find_by_id(r.user_id).await?;
            result.push(FriendModel {
                friendship_id: r.id,
                friend_id: r.user_id,
                display_name: brawler.display_name,
                username: brawler.username,
                avatar_url: brawler.avatar_url,
                status: r.status,
            });
        }
        Ok(result)
    }

    pub async fn invite_to_mission(
        &self,
        inviter_id: i32,
        invitee_id: i32,
        mission_id: i32,
    ) -> Result<i32> {
        // Check if they are friends
        let friendship = self
            .friendship_repo
            .check_friendship(inviter_id, invitee_id)
            .await?;
        if friendship.is_none() || friendship.unwrap().status != "accepted" {
            return Err(anyhow!("You can only invite friends to your mission"));
        }

        self.invitation_repo
            .invite(AddMissionInvitationEntity {
                mission_id,
                inviter_id,
                invitee_id,
                status: "pending".to_string(),
            })
            .await
    }

    pub async fn get_my_invitations(&self, user_id: i32) -> Result<Vec<MissionInvitationModel>> {
        let invitations = self
            .invitation_repo
            .get_received_invitations(user_id)
            .await?;
        let mut result = Vec::new();

        for i in invitations {
            let mission = self.mission_repo.get_one(i.mission_id).await?;
            let inviter = self.brawlers_repo.find_by_id(i.inviter_id).await?;

            result.push(MissionInvitationModel {
                invitation_id: i.id,
                mission_id: i.mission_id,
                mission_name: mission.name,
                inviter_id: i.inviter_id,
                inviter_name: inviter.display_name,
                status: i.status,
                created_at: i.created_at.to_string(),
            });
        }
        Ok(result)
    }

    pub async fn respond_to_invitation(
        &self,
        _user_id: i32,
        invitation_id: i32,
        accept: bool,
    ) -> Result<()> {
        if accept {
            self.invitation_repo.accept(invitation_id).await
        } else {
            self.invitation_repo.reject(invitation_id).await
        }
    }
}
