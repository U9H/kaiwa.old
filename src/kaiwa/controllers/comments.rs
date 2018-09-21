use chrono::Utc;
use kaiwa::db::Conn;
use kaiwa::models::comment::*;
use rocket_contrib::Json;

use diesel;
use diesel::prelude::*;
use kaiwa::schema::comments;
use kaiwa::schema::comments::dsl::*;
// use kaiwa::error::Error;

#[post("/<site>/<page>/comments", data = "<new_comment>")]
pub fn create(
    site: String,
    page: String,
    new_comment: Json<NewComment>,
    conn: Conn,
) -> Json<Comment> {
    let c = diesel::insert_into(comments::table)
        // .values((&new_comment.into_inner(), page_id.eq(page))) // we need a way to lookup by urlencoded value, not id
        .values(&new_comment.into_inner())
        .get_result(&*conn)
        .unwrap();
    Json(c)
}

#[get("/<site>/<page>/comments/<comment_id>")]
pub fn read(site: String, page: String, comment_id: i32, conn: Conn) -> Json<Comment> {
    let c: Comment = comments.find(comment_id).get_result(&*conn).unwrap();
    Json(c)
}
