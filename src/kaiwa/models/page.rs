use chrono::{DateTime, Utc};
use kaiwa::models::site::Site;
use kaiwa::schema::pages;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Site)]
pub struct Page {
    pub id: usize,
    pub site_id: usize,
    /// URL encoded path of the page.  For example,
    /// `https://mysite.com/pages/1/my-travels.html` becomes /pages/1/my-travels.html`.
    /// Trailing slashes are removed, so `https://mysite.com/pages/1/my-travels/
    /// should be /pages/1/my-travels.  That behavior is not verified serverside.
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
    pub id: usize,
    pub slug: Option<String>,
}
