use actix_web::{Form, HttpRequest, HttpResponse, Path};
use kaiwa::{
    error::Error,
    models::comment::{Comment, CommentForm},
};

pub fn create(params: Form<CommentForm>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}

pub fn read(path: Path<u32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}

pub fn update(path: Path<u32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}

pub fn destroy(path: Path<u32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}
