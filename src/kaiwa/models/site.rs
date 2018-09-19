use chrono::{DateTime, Utc};
use kaiwa::schema::sites;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Identifiable, Queryable)]
pub struct Site {
    pub id: u32,
    pub domain: String,
    pub api_key: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "sites"]
pub struct NewSite<'a> {
    pub domain: &'a str,
}

#[derive(AsChangeset, Identifiable, Deserialize, Serialize)]
#[table_name = "sites"]
pub struct SiteForm {
    pub id: u32,
    pub domain: Option<String>,
}
