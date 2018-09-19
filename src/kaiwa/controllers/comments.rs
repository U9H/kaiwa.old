use rocket::response::content::Json;
use kaiwa::models::comment::Comment;
use kaiwa::db::DbConn;

#[post("/comments", data = "<comment>")]
fn create(comment: Json<Comment>, connection: DbConn) -> Json<Comment> {
    let insert = Comment {
        id: 0,
        ..comment.into_inner()
    };
    Json(Comment::create(insert, &connection))
}
