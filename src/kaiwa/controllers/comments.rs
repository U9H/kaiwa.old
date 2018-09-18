use kaiwa::{error::Error, models::comment::Comment};
use rocket::response::content::Json;

#[post("/comments", data = "<comment>")]
fn create(comment: Json<Comment>, connection: db::Connection) -> Json<Comment> {
    let insert = Comment {
        id: None,
        ..comment.into_inner()
    };
    Json(Comment::create(insert, &connection))
}
