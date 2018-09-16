#![feature(try_trait)]
extern crate actix_web;

mod kaiwa;

use actix_web::{http::Method, pred, server, App, Result};
use kaiwa::server_error::Error as ServerError;

static PORT: &'static str = "3000";

fn main() -> Result<(), ServerError> {
    let backend = server::new(|| {
        App::new().scope("/comments", |scope| {
            scope.resource("/{id}", |r| {
                r.get().with(kaiwa::controllers::comments::read)
            })
        })
    }).bind(format!("127.0.0.1:{}", PORT))?;
    backend.run();
    Ok(())
}
