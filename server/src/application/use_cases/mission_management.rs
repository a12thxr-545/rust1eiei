use chrono::Utc;
use std::sync::Arc;

use crate::domain::{
    entities::crew_memberships::CrewMemberShips,
    repositories::{
        crew_operation::CrewOperationRepository, mission_management::MissionManagementRepository,
        mission_viewing::MissionViewingRepository,
    },
    value_objects::{
        base64_image::Base64Image,
        mission_model::{AddMissionModel, EditMissionModel},
        realtime::RealtimeEvent,
        uploaded_image::UploadedImage,
    },
};
use crate::infrastructure::{cloudinary::UploadImageOptions, realtime::SharedRealtimeHub};

pub struct MissionManagementUseCase<T1, T2, T3>
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: CrewOperationRepository + Send + Sync,
{
    mission_management_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
    crew_operation_repository: Arc<T3>,
    pub realtime_hub: SharedRealtimeHub,
}

use anyhow::Result;
impl<T1, T2, T3> MissionManagementUseCase<T1, T2, T3>
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: CrewOperationRepository + Send + Sync,
{
    pub fn new(
        mission_management_repository: Arc<T1>,
        mission_viewing_repository: Arc<T2>,
        crew_operation_repository: Arc<T3>,
        realtime_hub: SharedRealtimeHub,
    ) -> Self {
        Self {
            mission_management_repository,
            mission_viewing_repository,
            crew_operation_repository,
            realtime_hub,
        }
    }

    pub async fn add(&self, chief_id: i32, mut add_mission_model: AddMissionModel) -> Result<i32> {
        if add_mission_model.name.trim().is_empty() || add_mission_model.name.trim().len() < 3 {
            return Err(anyhow::anyhow!(
                "Mission name must be at least 3 characters long."
            ));
        }

        // Check if chief is already in a mission
        let current_mission = self
            .crew_operation_repository
            .get_current_mission(chief_id)
            .await?;
        if let Some(mid) = current_mission {
            let mission = self.mission_viewing_repository.get_one(mid).await?;
            return Err(anyhow::anyhow!(
                "You are already in an active mission: '{}' (#{}). Leave or end it first before creating a new one.",
                mission.name,
                mission.code
            ));
        }

        add_mission_model.description = add_mission_model.description.and_then(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });

        let code = self.generate_random_code();
        let insert_mission_entity = add_mission_model.to_entity_with_code(chief_id, code);

        let mission_id = self
            .mission_management_repository
            .add(insert_mission_entity)
            .await?;

        // Auto-join the chief to their own mission
        self.crew_operation_repository
            .join(CrewMemberShips {
                mission_id,
                brawler_id: chief_id,
            })
            .await?;

        self.realtime_hub.broadcast(RealtimeEvent::MissionCreated {
            mission_id,
            chief_id,
        });

        Ok(mission_id)
    }

    pub async fn edit(
        &self,
        mission_id: i32,
        chief_id: i32,
        mut edit_mission_model: EditMissionModel,
    ) -> Result<i32> {
        if let Some(mission_name) = &edit_mission_model.name {
            if mission_name.trim().is_empty() {
                edit_mission_model.name = None;
            } else if mission_name.trim().len() < 3 {
                return Err(anyhow::anyhow!(
                    "Mission name must be at least 3 characters long."
                ));
            } else {
                edit_mission_model.name = Some(mission_name.trim().to_string());
            }
        }

        edit_mission_model.description = edit_mission_model.description.and_then(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });

        let edit_mission_entity = edit_mission_model.to_entity(chief_id);

        let result = self
            .mission_management_repository
            .edit(mission_id, edit_mission_entity)
            .await?;

        self.realtime_hub.broadcast(RealtimeEvent::MissionUpdated {
            mission_id,
            chief_id,
        });

        Ok(result)
    }

    pub async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        // Broadcast to all members that the mission is being deleted
        self.realtime_hub
            .broadcast(RealtimeEvent::MissionDeleted { mission_id });

        self.mission_management_repository
            .remove(mission_id, chief_id)
            .await?;
        Ok(())
    }

    pub async fn upload_image(
        &self,
        base64_image: String,
        brawler_id: i32,
    ) -> Result<UploadedImage> {
        let option = UploadImageOptions {
            folder: Some("missions".to_string()),
            public_id: Some(format!("mission_{}_{}", brawler_id, Utc::now().timestamp())),
            transformation: Some("c_fill,w_800,h_450".to_string()), // 16:9 aspect ratio
        };

        let base64_image = Base64Image::new(&base64_image)?;

        let uploaded_image =
            crate::infrastructure::cloudinary::upload(base64_image, option).await?;

        Ok(uploaded_image)
    }

    fn generate_random_code(&self) -> String {
        use uuid::Uuid;
        Uuid::new_v4()
            .to_string()
            .replace("-", "")
            .chars()
            .take(5)
            .collect::<String>()
            .to_uppercase()
    }
}
