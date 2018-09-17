use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use std;

#[derive(Deserialize)]
pub struct CommentForm {
    name: Option<String>,
    comment: String,
    page_id: u32,
}

impl Default for CommentForm {
    fn default() -> Self {
        CommentForm {
            name: None,
            comment: String::new(),
            page_id: 0,
        }
    }
}

pub struct Comment {
    id: u32,
    page_id: u32,
    // User can be anonymous
    name: Option<String>,
    comment: String,
    created_at: DateTime<Utc>,
}

impl Comment {
    /// Generate a serializable version of the comment
    pub fn into_params(self) -> CommentForm {
        CommentForm {
            name: self.name,
            comment: self.comment,
            page_id: self.page_id,
        }
    }
}
