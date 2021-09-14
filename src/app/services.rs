use std::process::Command;

use actix_web::web::Payload;
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use log::info;

use crate::app::socket::ControlSocket;

// TODO: make cross-platform
#[cfg(windows)]
#[get("/shutdown")]
pub async fn shutdown() -> impl Responder {
    Command::new("cmd")
        .args(&["/C", "shutdown -s"])
        .output()
        .expect("failed to shutdown");

    HttpResponse::Ok().body("Bye!")
}

#[get("/control")]
pub async fn control(r: HttpRequest, stream: Payload) -> actix_web::Result<HttpResponse> {
    let res = ws::start(ControlSocket::new(), &r, stream);
    info!("Established control websocket connection");
    res
}
