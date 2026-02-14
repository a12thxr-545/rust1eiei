use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::{
    entities::missions::{AddMissionEntity, EditMissionEntity},
    value_objects::mission_statuses::MissionStatuses,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MissionModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub chief_id: i32,
    pub chief_name: String,
    pub crew_count: i64,
    pub image_url: Option<String>,
    pub code: String,
    pub max_participants: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddMissionModel {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub max_participants: i32,
}

impl AddMissionModel {
    pub fn to_entity_with_code(&self, chief_id: i32, code: String) -> AddMissionEntity {
        AddMissionEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            status: MissionStatuses::Open.to_string(),
            chief_id,
            image_url: self.image_url.clone(),
            code,
            max_participants: self.max_participants,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditMissionModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_participants: Option<i32>,
}

impl EditMissionModel {
    pub fn to_entity(&self, chief_id: i32) -> EditMissionEntity {
        EditMissionEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            chief_id,
            max_participants: self.max_participants,
        }
    }
}
