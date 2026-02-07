use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MissionFilter {
    pub name: Option<String>,
    pub code: Option<String>,
    pub status: Option<String>,
    #[serde(alias = "chiefId")]
    pub chief_id: Option<i32>,
    #[serde(alias = "excludeChiefId")]
    pub exclude_chief_id: Option<i32>,
    #[serde(alias = "memberId")]
    pub member_id: Option<i32>,
    #[serde(alias = "excludeMemberId")]
    pub exclude_member_id: Option<i32>,
}
