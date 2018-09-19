use chrono::{DateTime, Utc};
use kaiwa::models::site::Site;
use kaiwa::schema::pages;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Site)]
pub struct Page {
    pub id: u32,
    pub site_id: u32,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "pages"]
pub struct NewPage<'a> {
    pub slug: &'a str,
}

#[derive(AsChangeset, Identifiable, Deserialize, Serialize)]
#[table_name = "pages"]
pub struct PageForm {
    pub id: u32,
    pub slug: Option<String>,
}
