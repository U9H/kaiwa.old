use kaiwa::models::page::Page;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SiteForm {
    domain: String,
    ip: String,
}

impl Default for SiteForm {
    fn default() -> Self {
        SiteForm {
            domain: String::new(),
            ip: String::new(),
        }
    }
}

pub struct Site {
    id: u32,
    domain: String,
    // User can be anonymous
    ip: String,
    pages: Vec<Page>,
}

impl Site {
    pub fn into_params() -> SiteForm {
        SiteForm::default()
    }
}
