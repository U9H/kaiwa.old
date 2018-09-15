use actix_web;
use std;

#[derive(Debug)]
pub enum Error {
    NoUserGiven(std::option::NoneError),
}

impl From<std::option::NoneError> for Error {
    fn from(err: std::option::NoneError) -> Error {
        Error::NoUserGiven(err)
    }
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        match *self {
            Error::NoUserGiven(err) => {
                actix_web::HttpResponse::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            // Error::NoUserGiven(err) => err.description(),
            Error::NoUserGiven(err) => "No user found.",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            // Error::NoUserGiven(err) => Some(&self.side),
            Error::NoUserGiven(_err) => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "{}", &self.description())
        write!(f, "Nothing found")
    }
}
