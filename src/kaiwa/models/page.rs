use kaiwa::models::comment::Comment;
use serde_derive::Deserialize;
#[derive(Deserialize)]
pub struct PageForm {
    route: String,
}

impl Default for PageForm {
    fn default() -> Self {
        PageForm {
            route: String::new(),
        }
    }
}

pub struct Page {
    id: u32,
    site_id: u32,
    route: String,
    comments: Vec<Comment>,
}

impl Page {
    pub fn into_params() -> PageForm {
        PageForm::default()
    }
}
