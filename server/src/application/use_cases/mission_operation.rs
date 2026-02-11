use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{
        mission_operation::MissionOperationRepository, mission_viewing::MissionViewingRepository,
    },
    value_objects::{mission_statuses::MissionStatuses, realtime::RealtimeEvent},
};
use crate::infrastructure::realtime::SharedRealtimeHub;
pub struct MissionOperationUseCase<T1, T2>
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    mission_operation_repository: Arc<T1>,
    missiom_viewing_repository: Arc<T2>,
    pub realtime_hub: SharedRealtimeHub,
}

impl<T1, T2> MissionOperationUseCase<T1, T2>
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    pub fn new(
        mission_operation_repository: Arc<T1>,
        missiom_viewing_repository: Arc<T2>,
        realtime_hub: SharedRealtimeHub,
    ) -> Self {
        Self {
            mission_operation_repository,
            missiom_viewing_repository,
            realtime_hub,
        }
    }

    pub async fn in_progress(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self.missiom_viewing_repository.get_one(mission_id).await?;

        let crew_count = self
            .missiom_viewing_repository
            .crew_counting(mission_id)
            .await?;

        let is_status_open = mission.status == MissionStatuses::Open.to_string();

        let max_crew_per_mission = std::env::var("MAX_CREW_PER_MISSION")
            .expect("missing value")
            .parse::<i64>()?;

        let update_condition = is_status_open
            && crew_count > 0
            && crew_count < max_crew_per_mission
            && mission.chief_id == chief_id;
        if !update_condition {
            if !is_status_open {
                return Err(anyhow::anyhow!(
                    "Mission status must be Open to start. Current: {}",
                    mission.status
                ));
            }
            if crew_count <= 0 {
                return Err(anyhow::anyhow!(
                    "Mission must have at least one crew member"
                ));
            }
            if crew_count >= max_crew_per_mission {
                return Err(anyhow::anyhow!(
                    "Mission crew limit reached or exceeded (Max: {})",
                    max_crew_per_mission
                ));
            }
            if mission.chief_id != chief_id {
                return Err(anyhow::anyhow!("Only the Chief can start the mission"));
            }
            return Err(anyhow::anyhow!("Invalid condition to change stages!"));
        }

        let result = self
            .mission_operation_repository
            .to_progress(mission_id, chief_id)
            .await?;

        self.realtime_hub
            .broadcast(RealtimeEvent::MissionStatusChanged {
                mission_id,
                status: MissionStatuses::InProgress.to_string(),
                brawler_id: chief_id,
            });

        Ok(result)
    }
    pub async fn to_completed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self.missiom_viewing_repository.get_one(mission_id).await?;

        let update_condition = mission.status == MissionStatuses::InProgress.to_string()
            && mission.chief_id == chief_id;
        if !update_condition {
            if mission.status != MissionStatuses::InProgress.to_string() {
                return Err(anyhow::anyhow!(
                    "Mission must be In Progress to complete. Current: {}",
                    mission.status
                ));
            }
            if mission.chief_id != chief_id {
                return Err(anyhow::anyhow!("Only the Chief can complete the mission"));
            }
            return Err(anyhow::anyhow!("Invalid condition to change stages!"));
        }
        let result = self
            .mission_operation_repository
            .to_completed(mission_id, chief_id)
            .await?;

        self.realtime_hub
            .broadcast(RealtimeEvent::MissionStatusChanged {
                mission_id,
                status: MissionStatuses::Completed.to_string(),
                brawler_id: chief_id,
            });

        Ok(result)
    }
    pub async fn to_failed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self.missiom_viewing_repository.get_one(mission_id).await?;

        let update_condition = mission.status == MissionStatuses::InProgress.to_string()
            && mission.chief_id == chief_id;
        if !update_condition {
            if mission.status != MissionStatuses::InProgress.to_string() {
                return Err(anyhow::anyhow!(
                    "Mission must be In Progress to fail. Current: {}",
                    mission.status
                ));
            }
            if mission.chief_id != chief_id {
                return Err(anyhow::anyhow!("Only the Chief can fail the mission"));
            }
            return Err(anyhow::anyhow!("Invalid condition to change stages!"));
        }
        let result = self
            .mission_operation_repository
            .to_failed(mission_id, chief_id)
            .await?;

        self.realtime_hub
            .broadcast(RealtimeEvent::MissionStatusChanged {
                mission_id,
                status: MissionStatuses::Failed.to_string(),
                brawler_id: chief_id,
            });

        Ok(result)
    }
}
