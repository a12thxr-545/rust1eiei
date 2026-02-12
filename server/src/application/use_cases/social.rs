use crate::domain::{
    entities::{crew_memberships::CrewMemberShips, friendships::AddFriendshipEntity},
    repositories::{
        brawlers::BrawlerRepository, crew_operation::CrewOperationRepository,
        friendships::FriendshipRepository, mission_invitations::MissionInvitationRepository,
        mission_viewing::MissionViewingRepository,
    },
    value_objects::{
        mission_statuses::MissionStatuses,
        realtime::RealtimeEvent,
        social_model::{FriendModel, FriendshipStatusModel, MissionInvitationModel},
    },
};
use crate::infrastructure::realtime::SharedRealtimeHub;
use anyhow::{Result, anyhow};
use std::sync::Arc;

pub struct SocialUseCase<T1, T2, T3, T4, T5>
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    friendship_repo: Arc<T1>,
    invitation_repo: Arc<T2>,
    brawlers_repo: Arc<T3>,
    mission_repo: Arc<T4>,
    crew_repo: Arc<T5>,
    pub realtime_hub: SharedRealtimeHub,
}

impl<T1, T2, T3, T4, T5> SocialUseCase<T1, T2, T3, T4, T5>
where
    T1: FriendshipRepository + Send + Sync,
    T2: MissionInvitationRepository + Send + Sync,
    T3: BrawlerRepository + Send + Sync,
    T4: MissionViewingRepository + Send + Sync,
    T5: CrewOperationRepository + Send + Sync,
{
    pub fn new(
        friendship_repo: Arc<T1>,
        invitation_repo: Arc<T2>,
        brawlers_repo: Arc<T3>,
        mission_repo: Arc<T4>,
        crew_repo: Arc<T5>,
        realtime_hub: SharedRealtimeHub,
    ) -> Self {
        Self {
            friendship_repo,
            invitation_repo,
            brawlers_repo,
            mission_repo,
            crew_repo,
            realtime_hub,
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

        let res = self
            .friendship_repo
            .add(AddFriendshipEntity {
                user_id,
                friend_id,
                status: "pending".to_string(),
            })
            .await?;

        // Emit realtime event
        self.realtime_hub.broadcast(RealtimeEvent::FriendRequest {
            from_id: user_id,
            to_id: friend_id,
        });

        Ok(res)
    }

    pub async fn accept_friend(&self, user_id: i32, friend_id: i32) -> Result<()> {
        self.friendship_repo.accept(friend_id, user_id).await?;

        // Broadcast to the person who sent the request
        self.realtime_hub.broadcast(RealtimeEvent::FriendAccepted {
            from_id: user_id,
            to_id: friend_id,
        });

        Ok(())
    }

    pub async fn reject_friend(&self, user_id: i32, friend_id: i32) -> Result<()> {
        self.friendship_repo.reject(friend_id, user_id).await
    }

    pub async fn remove_friend(&self, user_id: i32, friend_id: i32) -> Result<()> {
        self.friendship_repo.remove(user_id, friend_id).await
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

        // Check if mission is active
        let mission = self.mission_repo.get_one(mission_id).await?;
        let mission_status_condition = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::InProgress.to_string()
            || mission.status == MissionStatuses::Failed.to_string();

        if !mission_status_condition {
            return Err(anyhow!(
                "You can only invite members to Open or In Progress missions"
            ));
        }

        // Check if inviter has permission (is a member of the mission)
        let is_inviter_member = self.crew_repo.is_member(mission_id, inviter_id).await?;
        if !is_inviter_member {
            return Err(anyhow!(
                "You must be a member of the mission to invite others"
            ));
        }

        // Check if invitee is already a member
        let is_invitee_member = self.crew_repo.is_member(mission_id, invitee_id).await?;
        if is_invitee_member {
            return Err(anyhow!("This friend is already in your crew!"));
        }

        // Clear any existing invitation to avoid unique constraint violation
        self.invitation_repo
            .delete_existing(mission_id, invitee_id)
            .await?;

        let res = self
            .invitation_repo
            .invite(
                crate::domain::entities::mission_invitations::AddMissionInvitationEntity {
                    mission_id,
                    inviter_id,
                    invitee_id,
                    status: "pending".to_string(),
                },
            )
            .await?;

        // Emit realtime event
        self.realtime_hub
            .broadcast(RealtimeEvent::MissionInvitation {
                mission_id,
                inviter_id,
                invitee_id,
            });

        Ok(res)
    }

    pub async fn get_my_invitations(&self, user_id: i32) -> Result<Vec<MissionInvitationModel>> {
        let invitations = self
            .invitation_repo
            .get_received_invitations(user_id)
            .await?;
        let mut result = Vec::new();

        for i in invitations {
            let mission = match self.mission_repo.get_one(i.mission_id).await {
                Ok(m) => m,
                Err(_) => continue, // Skip if mission not found or deleted
            };
            let inviter = match self.brawlers_repo.find_by_id(i.inviter_id).await {
                Ok(b) => b,
                Err(_) => continue, // Skip if inviter not found
            };
            let invitee = match self.brawlers_repo.find_by_id(i.invitee_id).await {
                Ok(b) => b,
                Err(_) => continue,
            };

            result.push(MissionInvitationModel {
                invitation_id: i.id,
                mission_id: i.mission_id,
                mission_name: mission.name,
                inviter_id: i.inviter_id,
                inviter_name: inviter.display_name,
                invitee_id: i.invitee_id,
                invitee_name: invitee.display_name,
                status: i.status,
                created_at: i.created_at.to_string(),
            });
        }
        Ok(result)
    }

    pub async fn get_mission_invitations(
        &self,
        mission_id: i32,
    ) -> Result<Vec<MissionInvitationModel>> {
        let invitations = self
            .invitation_repo
            .get_mission_invitations(mission_id)
            .await?;
        let mut result = Vec::new();

        for i in invitations {
            let mission = match self.mission_repo.get_one(i.mission_id).await {
                Ok(m) => m,
                Err(_) => continue,
            };
            let inviter = match self.brawlers_repo.find_by_id(i.inviter_id).await {
                Ok(b) => b,
                Err(_) => continue,
            };
            let invitee = match self.brawlers_repo.find_by_id(i.invitee_id).await {
                Ok(b) => b,
                Err(_) => continue,
            };

            result.push(MissionInvitationModel {
                invitation_id: i.id,
                mission_id: i.mission_id,
                mission_name: mission.name,
                inviter_id: i.inviter_id,
                inviter_name: inviter.display_name,
                invitee_id: i.invitee_id,
                invitee_name: invitee.display_name,
                status: i.status,
                created_at: i.created_at.to_string(),
            });
        }
        Ok(result)
    }

    pub async fn respond_to_invitation(
        &self,
        user_id: i32,
        invitation_id: i32,
        accept: bool,
    ) -> Result<i32> {
        let invitation = self.invitation_repo.get_by_id(invitation_id).await?;

        if invitation.invitee_id != user_id {
            return Err(anyhow!("This invitation is not for you"));
        }

        if accept {
            let mission = self.mission_repo.get_one(invitation.mission_id).await?;

            if mission.chief_id == user_id {
                return Err(anyhow!(
                    "You are the chief of this mission. You are already in the squad!"
                ));
            }

            // Check if user is already a member of *this* mission
            let already_member = self
                .crew_repo
                .is_member(invitation.mission_id, user_id)
                .await?;

            if already_member {
                // Already in this mission, just accept invitation and return success
                self.invitation_repo.accept(invitation_id).await?;
                return Ok(invitation.mission_id);
            }

            // Check if user is already in another *active* mission (Open/InProgress)
            let current_mission = self.crew_repo.get_current_mission(user_id).await?;
            if let Some(current_id) = current_mission {
                let current_mission_entity = self.mission_repo.get_one(current_id).await?;
                return Err(anyhow!(
                    "You are already in another active mission: '{}' (#{}). Leave or end it first before joining a new one.",
                    current_mission_entity.name,
                    current_mission_entity.code
                ));
            }

            let joinable = mission.status == MissionStatuses::Open.to_string()
                || mission.status == MissionStatuses::InProgress.to_string()
                || mission.status == MissionStatuses::Failed.to_string();

            if !joinable {
                return Err(anyhow!(
                    "Mission is no longer joinable (Status: {})",
                    mission.status
                ));
            }

            // Add to crew
            self.crew_repo
                .join(CrewMemberShips {
                    mission_id: invitation.mission_id,
                    brawler_id: user_id,
                })
                .await?;

            self.invitation_repo.accept(invitation_id).await?;

            // Broadcast to the inviter
            self.realtime_hub
                .broadcast(RealtimeEvent::MissionInvitationAccepted {
                    mission_id: invitation.mission_id,
                    user_id,
                    inviter_id: invitation.inviter_id,
                });

            Ok(invitation.mission_id)
        } else {
            self.invitation_repo.reject(invitation_id).await?;
            Ok(invitation.mission_id)
        }
    }

    pub async fn get_friendship_status(
        &self,
        user_id: i32,
        other_id: i32,
    ) -> Result<FriendshipStatusModel> {
        let friendship = self
            .friendship_repo
            .check_friendship(user_id, other_id)
            .await?;

        match friendship {
            Some(f) => Ok(FriendshipStatusModel {
                friendship_id: Some(f.id),
                initiator_id: Some(f.user_id),
                status: f.status,
            }),
            None => Ok(FriendshipStatusModel {
                friendship_id: None,
                initiator_id: None,
                status: "none".to_string(),
            }),
        }
    }
}
