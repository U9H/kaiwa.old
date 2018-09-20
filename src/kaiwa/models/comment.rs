use chrono::{NaiveDateTime, Utc};
use kaiwa::models::page::Page;
use kaiwa::schema::{comments};
use serde_derive::{Deserialize, Serialize};
use kaiwa::db::Conn;

#[derive(Deserialize, Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Page)]
pub struct Comment {
    pub page_id: i32,
    pub id: i32,
    /// Name of the creator
    pub name: Option<String>,
    /// Email of the creator
    pub email: Option<String>,
    /// Access code for editing purposes
    pub access_code: String,
    /// Comment body
    pub comment: String,
    // Time created
    pub created_at: NaiveDateTime,
}

impl Default for Comment {
    fn default() -> Self {
        Comment {
            page_id: 0,
            id: 0,
            name: None,
            email: None,
            access_code: "".to_string(),
            comment: "".to_string(),
            created_at: Utc::now().naive_utc(),
        }
    }
}

/// Used when creating
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub name: Option<String>,
    pub email: Option<String>,
    pub comment: String,
    pub page_id: i32,
}

/// Used when updating
#[derive(AsChangeset, Identifiable, Deserialize, Serialize)]
#[table_name = "comments"]
pub struct CommentForm {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub comment: Option<String>,
}
