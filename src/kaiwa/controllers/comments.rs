use actix_web::{HttpRequest, HttpResponse, Path};
use kaiwa::error::Error;

/// Get all comments, site-wide
pub fn create(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let id = req.match_info().get("id")?;
    Ok(HttpResponse::Ok().into())
}

pub fn read(path: Path<u32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}

pub fn update(req: &HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}

pub fn destroy(req: &HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}
