use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::infrastructure::database::schema::brawlers)]
pub struct BrawlerEntity {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub avatar_public_id: Option<String>,
    pub cover_url: Option<String>,
    pub cover_public_id: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::infrastructure::database::schema::brawlers)]
pub struct NewBrawlerEntity {
    pub username: String,
    pub password: String,
    pub display_name: String,
}
