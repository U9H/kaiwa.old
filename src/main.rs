extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn loopback(_req: &HttpRequest) -> String {
    format!("{:?}", _req)
}

fn main() {
    server::new(|| App::new()
        .resource("/", |r| r.f(index))
        .resource("/{name}", |r| r.f(loopback)))
            .bind("127.0.0.1:8080")
            .unwrap()
            .run();
}