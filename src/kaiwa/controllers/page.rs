use actix_web::HttpRequest;
use kaiwa::error::Error;

/// Get all comments, per page ID
pub fn index(req: &HttpRequest) -> Result<String, Error> {
    let id = req.match_info().get("id")?;
    Ok(format!("{}", id))
}
