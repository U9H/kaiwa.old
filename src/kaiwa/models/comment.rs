use chrono::{DateTime, Utc};
use super::schema::comments;
use serde_derive::Deserialize;

// Base data structure for a product
#[derive(Deserialize, Serialize, Queryable)]
#[primary_key(id)]
#[table_name = "comments"]
pub struct Comment {
    pub id: u32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub access_code: String,
    pub comment: String,
    pub created_at: DateTime<Utc>,
}

/// Used when creating products
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub comment: &'a str,
}

/// Used when updating products
#[derive(AsChangeset, Identifiable, Deserialize, Serialize)]
#[table_name = "comments"]
pub struct CommentForm {
    pub id: u32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub comment: Option<String>,
}
