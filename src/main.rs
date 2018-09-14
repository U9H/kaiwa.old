extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder};
static PORT: &'static str = "3000";

fn index(req: &HttpRequest) -> impl Responder {
    let site = req.match_info().get("site");
    let site = site.unwrap_or("Error");
    format!("{}", site)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
            .resource("/{site}", |r| r.f(index))
    }).bind(format!("127.0.0.1:{}", PORT))
        .expect(&format!("Can not bind to port {}", PORT))
        .run();
}
